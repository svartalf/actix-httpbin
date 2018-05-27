use actix_web::{HttpRequest, HttpResponse, Path, http};
use actix_web_httpauth::basic::{BasicAuth, Config};

use application::State;

#[derive(Deserialize)]
pub struct Credentials {
    user: String,
    passwd: String,
}

pub fn valid_auth(path: &Path<Credentials>, auth: &BasicAuth) -> bool {
    path.user == auth.username && path.passwd == auth.password
}

/// Prompts the user for authorization using HTTP Basic Auth.
pub fn basic_auth((req, path, auth): (HttpRequest<State>, Path<Credentials>, BasicAuth)) -> HttpResponse {
    if !valid_auth(&path, &auth) {
        BasicAuth::error_response(&Config::default())
    } else {
        let body = json!({
            "authenticated": true,
            "user": path.user,
        });
        req.build_response(http::StatusCode::OK)
            .body(body.to_string())
    }
}
