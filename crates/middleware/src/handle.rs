use crate::message::{AmsMessage, AmsTray};
use inventree::apis::configuration::{ApiKey, Configuration};
use inventree::apis::stock_api::{StockListParams, StockRemoveCreateParams};
use inventree::apis::{Api, ApiClient};
use inventree::models::{StockAdjustmentItem, StockRemove};
use log::{debug, info, warn};
use settings::SETTINGS;
use std::str::FromStr;
use std::sync::Arc;

pub async fn handle_ams_update(msg: &AmsMessage) -> Result<(), anyhow::Error> {
    let inv_api = ApiClient::new(Arc::new(Configuration {
        base_path: SETTINGS.inventree_url.clone(),
        api_key: Some(ApiKey {
            key: SETTINGS.inventree_token.clone(),
            prefix: Some("Token".to_string()),
        }),
        ..Default::default()
    }));

    for ams in &msg.ams {
        for tray in &ams.tray {
            // Skip none tagged spools.
            if tray.tag_uid == "0000000000000000" || tray.remain == -1 {
                continue;
            }

            let numeric_tray_weight = f32::from_str(&tray.tray_weight)?;
            let expected_remaining_weight = numeric_tray_weight * (tray.remain as f32 / 100.0);

            let data = StockListParams::builder()
                .batch(tray.tag_uid.clone())
                .part_detail(true)
                .limit(10)
                .build_struct();

            let item = inv_api.stock_api().stock_list(data).await?;

            if item.results.is_empty() {
                //TODO: Automatically create new spool.

                debug!(
                    "Could not find Stock Item for batch code: {}, expected remaining weight: {}g",
                    &tray.tag_uid, expected_remaining_weight
                );

                info!(
                    "Found Spool without matching Part: {}",
                    get_filament_match_key(tray)
                );

                continue;
            }

            if item.results.len() > 1 {
                warn!(
                    "Multiple Stock Items found for batch code: {}, expected remaining weight: {}g",
                    &tray.tag_uid, expected_remaining_weight
                );
                continue;
            }

            let stock_item = item.results.first().unwrap();

            if stock_item.quantity > expected_remaining_weight as f64 {
                info!(
                    "Updating Stock Item {} (batch code: {}) quantity from {} to {} based on AMS update",
                    stock_item.pk, &tray.tag_uid, stock_item.quantity, expected_remaining_weight
                );

                let diff = stock_item.quantity - expected_remaining_weight as f64;

                inv_api
                    .stock_api()
                    .stock_remove_create(StockRemoveCreateParams {
                        stock_remove: StockRemove {
                            items: vec![StockAdjustmentItem {
                                pk: stock_item.pk,
                                quantity: diff.to_string(),
                                ..Default::default()
                            }],
                            notes: Some("AMS Update - Removed".to_string()),
                        },
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
