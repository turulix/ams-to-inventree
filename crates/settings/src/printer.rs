use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrinterConfig {
    pub ip_addr: IpAddr,
    pub port: u16,
    pub serial: String,
    pub access_code: String,
}
