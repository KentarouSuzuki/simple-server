use chrono::prelude::*;
use server_info::basic_info::BasicInfo;
use serde::Serialize;
use rocket_contrib::json::Json;
use std::net::IpAddr;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct ServerInfoView {
    datetime: DateTime<Utc>,
    hostname: Option<String>,
    local_ip: Option<IpAddr>,
    version: Option<String>,
}

impl ServerInfoView {
    pub fn from_basic_info(basic: BasicInfo) -> Json<ServerInfoView> {
        Json(ServerInfoView{
            datetime: basic.datetime,
            hostname: basic.hostname,
            local_ip: basic.local_ip_address,
            version: basic.version,
        })
    }
}