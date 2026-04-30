use crate::message::{AmsMessage, AmsTray};
use inventree::apis::configuration::{ApiKey, Configuration};
use inventree::apis::stock_api::{StockListParams, StockRemoveCreateParams};
use inventree::apis::{Api, ApiClient};
use inventree::models::{StockAdjustmentItem, StockRemove};
use log::{debug, info, warn};
use std::str::FromStr;

// Only update if more than 1 gram changed to prevent API spam from micro-updates
const MIN_USAGE_THRESHOLD: f64 = 1.0;
// Max allowed grams to remove in a single update tick.
// A printer physically cannot extrude 100g in the few seconds between AMS telemetry updates.
const MAX_USAGE_SANITY_LIMIT: f64 = 100.0;

pub async fn handle_ams_update(inv_api: &ApiClient, msg: &AmsMessage) -> Result<(), anyhow::Error> {
    for ams in &msg.ams {
        for tray in &ams.tray {
            // Skip untagged spools or known invalid states
            if tray.tag_uid == "0000000000000000" || tray.remain == -1 {
                continue;
            }

            let numeric_tray_weight = match f32::from_str(&tray.tray_weight) {
                Ok(w) => w,
                Err(e) => {
                    warn!(
                        "Failed to parse tray weight '{}' for tag {}: {}",
                        tray.tray_weight, tray.tag_uid, e
                    );
                    continue;
                }
            };

            let expected_remaining_weight = numeric_tray_weight * (tray.remain as f32 / 100.0);

            let data = StockListParams::builder()
                .batch(tray.tag_uid.clone())
                .part_detail(true)
                .limit(10)
                .build_struct();

            let item = match inv_api.stock_api().stock_list(data).await {
                Ok(x) => x,
                Err(e) => {
                    warn!(
                        "Failed to query Inventree API for batch code: {}, error: {:?}",
                        &tray.tag_uid, e
                    );
                    continue;
                }
            };

            if item.results.is_empty() {
                // TODO: Automatically create new spool.
                debug!(
                    "Could not find Stock Item for batch code: {}, expected remaining weight: {}g",
                    &tray.tag_uid, expected_remaining_weight
                );
                info!(
                    "Found Spool without matching Part: {}, TAG: {}",
                    get_filament_match_key(tray),
                    &tray.tag_uid
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
            let diff = stock_item.quantity - expected_remaining_weight as f64;

            if diff < MIN_USAGE_THRESHOLD {
                // Ignore tiny floating point differences or micro-extrusions
                continue;
            }

            // SMART SANITY CHECK: Differentiate between a glitch and a manual remeasure
            if diff > MAX_USAGE_SANITY_LIMIT {
                if expected_remaining_weight <= 0.0 {
                    // Huge drop to exactly 0g. This is the AMS telemetry glitch.
                    warn!(
                        "Anomalous drop to 0g detected ({}g diff) for batch {}. Skipping to prevent accidental stock wipe.",
                        diff, &tray.tag_uid
                    );
                    continue;
                } else {
                    // Huge drop, but there's still filament. This is your manual remeasurement.
                    info!(
                        "Large stock difference detected ({}g) for batch {}. Treating as manual remeasurement.",
                        diff, &tray.tag_uid
                    );
                }
            }

            info!(
                "Updating Stock Item {} (batch {}) quantity from {} to {}",
                stock_item.pk, &tray.tag_uid, stock_item.quantity, expected_remaining_weight
            );
            let remove_params = StockRemoveCreateParams {
                stock_remove: StockRemove {
                    items: vec![StockAdjustmentItem {
                        pk: stock_item.pk,
                        quantity: diff.to_string(),
                        ..Default::default()
                    }],
                    notes: Some("AMS Update - Removed".to_string()),
                },
            };

            // Don't crash the loop if the removal fails
            if let Err(e) = inv_api.stock_api().stock_remove_create(remove_params).await {
                warn!(
                    "Failed to update stock for batch code: {}. Error: {:?}",
                    &tray.tag_uid, e
                );
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
