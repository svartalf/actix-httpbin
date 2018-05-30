use actix_web::{HttpRequest, HttpResponse, Path, Result, FromRequest, http};
use actix_web_httpauth::extractors::{AuthenticationError, basic, bearer};

use application::State;

#[derive(Deserialize)]
pub struct Credentials {
    user: String,
    passwd: String,
}

/// Prompts the user for authorization using HTTP Basic Auth.
pub fn basic_auth((req, path, auth): (HttpRequest<State>, Path<Credentials>, basic::BasicAuth)) -> Result<HttpResponse> {
    if path.user == auth.username() && Some(path.passwd.as_str()) == auth.password() {
        let body = json!({
            "authenticated": true,
            "user": auth.username(),
        });
        let resp = req.build_response(http::StatusCode::OK)
            .body(body.to_string());
        Ok(resp)
    } else {
        Err(AuthenticationError::from(basic::Config::default()).into())
    }
}

/// Prompts the user for authorization using HTTP Basic Auth,
/// but returns `HTTP 404` instead of `HTTP 401`
pub fn hidden_basic_auth((req, path): (HttpRequest<State>, Path<Credentials>)) -> HttpResponse {
    if let Ok(auth) = basic::BasicAuth::extract(&req) {
        if path.user == auth.username() && Some(path.passwd.as_str()) == auth.password() {
            let body = json!({
                "authenticated": true,
                "user": auth.username(),
            });

            return req.build_response(http::StatusCode::OK)
                .body(body.to_string());
        }
    }

    req.build_response(http::StatusCode::NOT_FOUND)
        .finish()

}

/// Checks for HTTP Bearer auth
pub fn bearer_auth((req, auth): (HttpRequest<State>, bearer::BearerAuth)) -> HttpResponse {
    let body = json!({
        "authenticated": true,
        "token": auth.token(),
    });

    req.build_response(http::StatusCode::OK)
        .body(body.to_string())
}
