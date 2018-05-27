use actix_web::{HttpRequest, HttpResponse, Path, Query, Result, http};

use application::State;


#[derive(Debug, Deserialize)]
pub struct RedirectQuery {
    absolute: Option<bool>,
}

pub fn redirect_n((req, path, query): (HttpRequest<State>, Path<u8>, Query<RedirectQuery>)) -> HttpResponse {
    if query.absolute.unwrap_or(false) {
        absolute_redirect_n((req, path))
    } else {
        relative_redirect_n((req, path))
    }
}


/// `HTTP 302` redirects `n` times with an absolute URL.
pub fn absolute_redirect_n((req, path): (HttpRequest<State>, Path<u8>)) -> HttpResponse {
    // TODO: Use `req.url_for()` here
    let info = req.connection_info();
    let url = if *path == 1 {
        format!("{}://{}/get", info.scheme(), info.host())
    } else {
        format!("{}://{}/absolute-redirect/{}", info.scheme(), info.host(), (*path - 1))
    };

    req.build_response(http::StatusCode::FOUND)
        .header("Location", url)
        .finish()
}

#[derive(Debug, Deserialize)]
pub struct RedirectTo {
    url: String,
    status_code: Option<u16>,
}

impl RedirectTo {
    fn parts(self) -> (String, http::StatusCode) {
        let code = match self.status_code {
            Some(code) if code >= 300 && code < 400 => http::StatusCode::from_u16(code).expect("Unexpected"),
            _ => http::StatusCode::FOUND,
        };

        (self.url, code)
    }
}

pub fn redirect_to((req, query): (HttpRequest<State>, Query<RedirectTo>)) -> Result<HttpResponse> {
    let (url, code) = query.into_inner().parts();
    let resp = req.build_response(code)
        .header(http::header::LOCATION, url)
        .finish();
    Ok(resp)
}


/// `HTTP 302` redirects `n` times with a relative URL.
pub fn relative_redirect_n((req, path): (HttpRequest<State>, Path<u8>)) -> HttpResponse {
    // TODO: Use `req.url_for()` here
    let url = if *path == 1 {
        "/get".to_string()
    } else {
        format!("/relative-redirect/{}", (*path - 1))
    };

    req.build_response(http::StatusCode::FOUND)
        .header("Location", url)
        .finish()
}
