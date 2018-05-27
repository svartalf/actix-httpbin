use actix_web::{HttpRequest, HttpResponse, Result, http, error};
use tera::{self, Tera};
pub use tera::Context;

use application::State;

// Since all assets are bundled into an application,
// we need to create tera with a bundled templates too.
pub fn create_tera() -> tera::Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
        ("index.html", include_str!("./assets/index.html")),
        ("links.html", include_str!("./assets/links.html")),
        ("forms-post.html", include_str!("./assets/forms-post.html"))
    ])?;

    Ok(tera)
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn render(req: HttpRequest<State>, template: &'static str, context: &Context) -> Result<HttpResponse> {
    let body = req.state().template
        .render(template, context)
        .map_err(|e| {
            error::ErrorInternalServerError(format!("{:#?}", e))
        })?;

    Ok(req.build_response(http::StatusCode::OK)
        .content_type("text/html")
        .body(body))
}
