use chrono::prelude::*;
use std::net::IpAddr;
use server_info::basic_info::BasicInfo;
use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
pub struct ServerInfoView {
    datetime: DateTime<Utc>,
    hostname: Option<String>,
    local_ip: Option<IpAddr>,
    version: Option<String>,
}

impl ServerInfoView {
    pub fn from_basic_info(basic: BasicInfo) -> HttpResponse {
        HttpResponse::Ok().json(ServerInfoView{
            datetime: basic.datetime,
            hostname: basic.hostname,
            local_ip: basic.local_ip_address,
            version: basic.version,
        })
    }
}