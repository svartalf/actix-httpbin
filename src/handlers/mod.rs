use actix_web::{HttpRequest, HttpResponse, Result};
use tera::Context;

pub mod methods;
pub mod auth;
pub mod statuses;
pub mod request;
pub mod response;
pub mod formats;
pub mod dynamic;
pub mod cookies;
pub mod images;
pub mod redirects;

use application::State;
use templates;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn index(req: HttpRequest<State>) -> Result<HttpResponse> {
    templates::render(req, "index.html", &Context::new())
}
