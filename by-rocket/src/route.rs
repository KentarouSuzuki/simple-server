use crate::service::hello;

#[get("/")]
pub fn root() -> &'static str {
    hello()
}