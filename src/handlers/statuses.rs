use actix_web::{HttpResponse, Path, Result, error};
use actix_web::http::StatusCode;

/// Return given status code.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn status(info: Path<u16>) -> Result<HttpResponse> {
    let code = StatusCode::from_u16(*info)
        .map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::new(code))
}
