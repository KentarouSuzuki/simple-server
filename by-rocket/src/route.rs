use rocket_contrib::json::Json;
use crate::service::get_server_info;
use crate::view::ServerInfoView;

#[get("/")]
pub fn root() -> Json<ServerInfoView> {
    ServerInfoView::from_basic_info(get_server_info())
}