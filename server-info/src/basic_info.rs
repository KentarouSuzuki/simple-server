use hostname::get_hostname;
use chrono::prelude::*;
use local_ip;
use std::env;
use std::net::IpAddr;

pub struct BasicInfo {
    pub datetime: DateTime<Utc>,
    pub hostname: Option<String>,
    pub local_ip_address: Option<IpAddr>,
    pub version: Option<String>,
}

impl BasicInfo {
    pub fn new() -> BasicInfo {
        BasicInfo {
            datetime: Utc::now(),
            hostname: get_hostname(),
            local_ip_address: local_ip::get(),
            version: match env::var("VERSION") {
                Ok(val) => Some(val),
                Err(_) => None
            },
        }
    }
}