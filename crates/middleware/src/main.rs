mod handle;
mod message;
mod tls_validator;

use crate::handle::handle_ams_update;
use crate::message::PrinterMessage;
use crate::tls_validator::DangerAcceptAllCertVerifier;
use log::{debug, error, info, warn};
use rumqttc::tokio_rustls::rustls::ClientConfig;
use rumqttc::Packet::Publish;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, Transport};
use settings::printer::PrinterConfig;
use settings::SETTINGS;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut listen_tasks = JoinSet::new();

    // Spawn a dedicated monitoring task for each printer
    for printer in &SETTINGS.printers {
        let printer_clone = printer.clone(); // Assuming your config struct implements Clone
        listen_tasks.spawn(async move {
            monitor_printer(printer_clone).await;
        });
    }

    // Robustly keep the main thread alive and log if a task unexpectedly dies
    while let Some(res) = listen_tasks.join_next().await {
        if let Err(e) = res {
            error!("A printer task panicked: {:?}", e);
        }
    }
}

/// Handles the entire lifecycle of a single printer's MQTT connection
async fn monitor_printer(printer: PrinterConfig) {
    // 1. Create a channel to separate network I/O from business logic.
    // The channel holds up to 100 unprocessed messages before applying backpressure.
    let (tx, mut rx) = mpsc::channel::<String>(100);
    let serial_for_processor = printer.serial.clone();

    // 2. Spawn a dedicated worker task to process messages.
    // This ensures handle_ams_update NEVER blocks the MQTT event loop.
    tokio::spawn(async move {
        while let Some(payload) = rx.recv().await {
            match serde_json::from_str::<PrinterMessage>(&payload) {
                Ok(serialized) => {
                    if let Some(msg) = serialized.print.ams
                        && let Err(e) = handle_ams_update(&msg).await
                    {
                        error!(
                            "Error handling AMS update for {}: {:?}",
                            serial_for_processor, e
                        );
                    }
                }
                Err(e) => error!("JSON parse error for {}: {}", serial_for_processor, e),
            }
        }
    });

    // 3. Generate a unique client ID to prevent broker collisions on restart
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let client_id = format!("{}-{}", printer.serial, timestamp);

    let mut options = MqttOptions::new(client_id, printer.ip_addr.to_string(), printer.port);
    options.set_credentials("bblp", &printer.access_code);
    options.set_keep_alive(Duration::from_secs(5));

    let client_config = ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(DangerAcceptAllCertVerifier))
        .with_no_client_auth();

    options.set_transport(Transport::tls_with_config(client_config.into()));

    let (client, mut eventloop) = AsyncClient::new(options, 10);

    info!(
        "Started monitoring printer {} at {}:{}",
        printer.serial, printer.ip_addr, printer.port
    );

    // Queue the subscription
    let topic = format!("device/{}/report", printer.serial);

    // 4. The main event loop: Fast, non-blocking, strictly for network I/O
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                debug!(
                    "Connected to broker for {}. (Re)subscribing...",
                    printer.serial
                );

                // Fire off the subscription request
                if let Err(e) = client.subscribe(&topic, QoS::AtMostOnce).await {
                    error!("Failed to subscribe for {}: {:?}", printer.serial, e);
                } else {
                    info!(
                        "Subscribed to topic '{}' for printer {}",
                        topic, printer.serial
                    );
                }
            }
            Ok(Event::Incoming(Publish(x))) => {
                if let Ok(payload) = String::from_utf8(x.payload.to_vec()) {
                    debug!("Received message from {}: {}", printer.serial, payload);

                    // Send payload to the worker task.
                    if tx.send(payload).await.is_err() {
                        error!(
                            "Worker task died for printer {}. Exiting monitor.",
                            printer.serial
                        );
                        break; // Exit loop if the receiver task panicked/died
                    }
                } else {
                    warn!("Invalid UTF-8 received from {}", printer.serial);
                }
            }
            Ok(_) => {} // Silently handle SubAck, PingResp, etc.
            Err(e) => {
                warn!(
                    "Connection issue with {}: {:?}. Reconnecting...",
                    printer.serial, e
                );
                // Prevent tight CPU spinning if the connection is aggressively refused
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }
}
