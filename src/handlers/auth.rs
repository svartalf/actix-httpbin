use actix_web::{HttpRequest, HttpResponse, Path, Result, http};
use actix_web_httpauth::extractors::{AuthenticationError, basic};

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
