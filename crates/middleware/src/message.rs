use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Deserialize)]
pub struct PrinterMessage {
    pub print: Option<PrintUpdate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrintUpdate {
    pub ams: Option<AmsMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AmsMessage {
    #[serde(default)]
    pub ams: Vec<AmsUnit>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AmsUnit {
    // We do not want to deserialize empty trays
    #[serde(deserialize_with = "skip_empty_trays")]
    pub tray: Vec<AmsTray>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AmsTray {
    pub remain: i32,
    pub tag_uid: String,
    /// For example PLA, ABS, PETG, etc.
    pub tray_type: String,
    /// For example PLA Silk+
    pub tray_sub_brands: String,
    /// "C8C8C8FF", 8-digit hex RGBA color code
    pub tray_color: String,
    pub tray_weight: String,
    pub tray_diameter: String,
}

/// Custom deserializer that attempts to parse each item as an AmsTray.
/// If it fails (e.g., missing fields because it's empty), it silently ignores it.
fn skip_empty_trays<'de, D>(deserializer: D) -> Result<Vec<AmsTray>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeTray {
        // Serde tries this first. If all fields are present, we get our Tray.
        Valid(AmsTray),
        // If the above fails, Serde falls back to this, gobbling up the JSON without error.
        Invalid(serde::de::IgnoredAny),
    }

    // Deserialize the JSON array into a Vec of our enum
    let trays: Vec<MaybeTray> = Vec::deserialize(deserializer)?;

    // Filter out the invalid ones and extract the valid AmsTrays
    Ok(trays
        .into_iter()
        .filter_map(|t| match t {
            MaybeTray::Valid(tray) => Some(tray),
            MaybeTray::Invalid(_) => None,
        })
        .collect())
}
