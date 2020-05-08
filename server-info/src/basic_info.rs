use hostname::get_hostname;
use chrono::prelude::*;
use std::env;

pub struct BasicInfo {
    pub datetime: DateTime<Utc>,
    pub hostname: Option<String>,
    pub version: Option<String>,
}

impl BasicInfo {
    pub fn new() -> BasicInfo {
        BasicInfo {
            datetime: Utc::now(),
            hostname: get_hostname(),
            version: match env::var("VERSION") {
                Ok(val) => Some(val),
                Err(_) => None
            },
        }
    }
}