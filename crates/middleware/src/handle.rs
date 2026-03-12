use crate::message::{AmsMessage, AmsTray};
use inventree::stock::{RemoveCreateBody, RemoveCreateInner, StockListQuery};
use inventree::InventreeApiClient;
use log::{debug, info, warn};
use settings::SETTINGS;
use std::str::FromStr;

pub async fn handle_ams_update(msg: &AmsMessage) -> Result<(), anyhow::Error> {
    let inv_api = InventreeApiClient::new(&SETTINGS.inventree_url, &SETTINGS.inventree_token);

    for ams in &msg.ams {
        for tray in &ams.tray {
            // Skip none tagged spools.
            if tray.tag_uid == "0000000000000000" || tray.remain == -1 {
                continue;
            }

            let numeric_tray_weight = f32::from_str(&tray.tray_weight)?;
            let expected_remaining_weight = numeric_tray_weight * (tray.remain as f32 / 100.0);

            let stock_item = inv_api
                .stock()
                .list(&Some(StockListQuery {
                    batch: Some(tray.tag_uid.clone()),
                    ..Default::default()
                }))
                .await?;

            if stock_item.is_empty() {
                //TODO: Automatically create new spool.

                debug!(
                    "Could not find Stock Item for batch code: {}, expected remaining weight: {}g",
                    &tray.tag_uid, expected_remaining_weight
                );

                info!(
                    "Found Spool without matching Part: {}",
                    get_filament_match_key(&tray)
                );

                continue;
            }

            if stock_item.len() > 1 {
                warn!(
                    "Multiple Stock Items found for batch code: {}, expected remaining weight: {}g",
                    &tray.tag_uid, expected_remaining_weight
                );
                continue;
            }

            let stock_item = stock_item.first().unwrap();

            if stock_item.quantity > expected_remaining_weight as f64 {
                info!(
                    "Updating Stock Item {} (batch code: {}) quantity from {} to {} based on AMS update",
                    stock_item.pk.0, &tray.tag_uid, stock_item.quantity, expected_remaining_weight
                );

                let diff = stock_item.quantity - expected_remaining_weight as f64;

                inv_api
                    .stock()
                    .remove_create(&RemoveCreateBody {
                        items: vec![RemoveCreateInner {
                            pk: stock_item.pk,
                            quantity: diff.to_string(),
                        }],
                        notes: "AMS Update - Removed".to_string(),
                    })
                    .await?;
            }
        }
    }

    Ok(())
}

pub fn get_filament_match_key(tray: &AmsTray) -> String {
    format!(
        "BambuLab-{}-{}-{}-{}",
        tray.tray_type, tray.tray_sub_brands, tray.tray_color, tray.tray_diameter
    )
}
