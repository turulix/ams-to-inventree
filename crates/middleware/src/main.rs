mod handle;
mod message;
mod tls_validator;

use crate::handle::handle_ams_update;
use crate::message::PrinterMessage;
use crate::tls_validator::DangerAcceptAllCertVerifier;
use log::debug;
use rumqttc::tokio_rustls::rustls::ClientConfig;
use rumqttc::Packet::Publish;
use rumqttc::{AsyncClient, Event, MqttOptions, QoS, Transport};
use settings::SETTINGS;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut listen_tasks = JoinSet::new();

    for printer in &SETTINGS.printers {
        listen_tasks.spawn(async move {
            let mut options = MqttOptions::new(
                format!("{}-kasjds", printer.serial),
                printer.ip_addr.to_string(),
                printer.port,
            );

            options.set_credentials("bblp", &printer.access_code);
            options.set_keep_alive(Duration::from_secs(5));

            let client_config = ClientConfig::builder()
                .dangerous() // Tells rustls we are doing something unsafe
                .with_custom_certificate_verifier(Arc::new(DangerAcceptAllCertVerifier))
                .with_no_client_auth();

            options.set_transport(Transport::tls_with_config(client_config.into()));

            let (client, mut eventloop) = AsyncClient::new(options, 10);

            client
                .subscribe(format!("device/{}/report", printer.serial), QoS::AtMostOnce)
                .await
                .unwrap();

            loop {
                let msg = eventloop.poll().await.unwrap();

                match msg {
                    Event::Incoming(Publish(x)) => {
                        let payload = String::from_utf8(x.payload.to_vec()).unwrap();
                        debug!("Received message on topic {}: {}", x.topic, payload);
                        let serialized = serde_json::from_str::<PrinterMessage>(&payload).unwrap();

                        if let Some(msg) = &serialized.print.ams {
                            handle_ams_update(msg).await.unwrap();
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    listen_tasks.join_all().await;
}
