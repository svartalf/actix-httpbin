use std::collections::HashMap;

use serde_json::Value;
use actix_web::{HttpRequest, HttpResponse, Path, Json, Result};
use actix_web::http::{header, StatusCode, Cookie};

use application::State;

#[derive(Deserialize)]
pub struct CookieInfo {
    name: String,
    value: String,
}

/// Returns cookie data.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn cookies(req: HttpRequest<State>) -> Result<Json<Value>> {
    let cookies = req.cookies()?.iter().map(|cookie| {
        (cookie.name(), cookie.value())
    }).collect::<HashMap<_, _>>();

    let body = json!({
        "cookies": cookies,
    });

    Ok(Json(body))
}

/// Set cookie based on the path params.
pub fn cookie_set((req, info): (HttpRequest<State>, Path<CookieInfo>)) -> Result<HttpResponse> {
    let is_secure = req.connection_info().scheme() == "https";
    let mut cookie = Cookie::new(info.name.clone(), info.value.clone());
    cookie.set_secure(is_secure);
    cookie.set_path("/");

    let resp = req.build_response(StatusCode::FOUND)
        .cookie(cookie)
        // TODO: Use `req.url_for()` here
        .header(header::LOCATION, "/cookies")
        .finish();
    Ok(resp)
}


/// Set cookies based on the query string params.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn cookies_set(req: HttpRequest<State>) -> Result<HttpResponse> {
    let is_secure = req.connection_info().scheme() == "https";
    let mut resp = req.build_response(StatusCode::FOUND);

    for &(ref name, ref value) in req.query().iter() {
        let mut cookie = Cookie::new(name.to_string(), value.to_string());
        cookie.set_secure(is_secure);
        cookie.set_path("/");
        resp.cookie(cookie);
    }

    // TODO: Use `req.url_for()` here
    Ok(resp.header(header::LOCATION, "/cookies").finish())
}


/// Delete cookies based on the query string params.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn cookies_delete(req: HttpRequest<State>) -> Result<HttpResponse> {
    let mut resp = req.build_response(StatusCode::FOUND);

    for &(ref name, _) in req.query().iter() {
        let cookie = Cookie::named(name.to_string());
        resp.del_cookie(&cookie);
    }

    // TODO: Use `req.url_for()` here
    Ok(resp.header(header::LOCATION, "/cookies").finish())
}
