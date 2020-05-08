use actix_web::{web, Resource, HttpResponse};
use crate::service::get_server_info;
use crate::view::ServerInfoView;

async fn server_info() -> HttpResponse {
    ServerInfoView::from_basic_info(get_server_info())
}

pub fn root() -> Resource {
    web::resource("/").route(web::to(server_info))
}