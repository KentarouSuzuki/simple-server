use chrono::prelude::*;
use server_info::basic_info::BasicInfo;
use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct ServerInfoView {
    datetime: DateTime<Utc>,
    hostname: Option<String>,
    version: Option<String>
}

impl ServerInfoView {
    pub fn from_basic_info(basic: BasicInfo) -> HttpResponse {
        HttpResponse::Ok().json(ServerInfoView{
            datetime: basic.datetime,
            hostname: basic.hostname,
            version: basic.version,
        })
    }
}