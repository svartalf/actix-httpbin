use futures::Future;
use actix_web::{HttpRequest, HttpResponse, Json, Result};
use actix_web::http::{StatusCode, ContentEncoding};
use serde_json::Value;

use application::State;
use response::{Body, Builder, ResponseFuture};
use templates::{Context, render};

static ENCODING_UTF8: &'static str = include_str!("../assets/UTF-8-demo.txt");
static HTML: &'static str = include_str!("../assets/moby.html");
static XML: &'static str = include_str!("../assets/sample.xml");
static ROBOTS_TXT: &'static str = "User-agent: *\nDisallow: /deny\n";
static ANGRY_ASCII: &'static str = include_str!("../assets/angry_ascii.txt");

fn compress(req: HttpRequest<State>, extra_key: &'static str, encoding: ContentEncoding) -> Box<ResponseFuture> {
    let resp = Body::new(req)
        .with_url()
        .with_headers()
        .with_origin()
        .with_args()
        .with_extra(extra_key, true)
        .into_response()
        .map(move |mut response| {
            response.set_content_encoding(encoding);
            response
        });

    Box::new(resp)
}

/// Returns Gzip-encoded data.
pub fn gzip(req: HttpRequest<State>) -> Box<ResponseFuture> {
    compress(req, "gzipped", ContentEncoding::Gzip)
}

/// Returns Deflate-encoded data.
pub fn deflate(req: HttpRequest<State>) -> Box<ResponseFuture> {
    compress(req, "deflated", ContentEncoding::Deflate)
}

/// Returns Brotli-encoded data.
pub fn brotli(req: HttpRequest<State>) -> Box<ResponseFuture> {
    compress(req, "brotli", ContentEncoding::Br)
}

/// Returns an UTF-8 encoded HTML page.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn encoding_utf8(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("text/html; charset=utf8")
        .body(ENCODING_UTF8)
}

/// Returns a simple XML document.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn xml(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("application/xml")
        .body(XML)
}

/// Returns a simple HTML document.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn html(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("text/html; charset=utf8")
        .body(HTML)
}

/// Returns a simple JSON document.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn json(_req: HttpRequest<State>) -> Json<Value> {
    let body = json!({
        "slideshow": {
            "title": "Sample Slide Show",
            "date": "date of publication",
            "author": "Yours Truly",
            "slides": [
                {
                    "type": "all",
                    "title": "Wake up to WonderWidgets!",
                },
                {
                    "type": "all",
                    "title": "Overview",
                    "items": [
                        "Why <em>WonderWidgets</em> are great",
                        "Who <em>buys</em> WonderWidgets",
                    ]
                }
            ],
        }
    });

    Json(body)
}

/// Returns some `robots.txt` rules
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn robots_txt(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("text/plain; charset=utf8")
        .body(ROBOTS_TXT)
}

/// Returns page denied by `robots.txt` rules.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn deny(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("text/plain; charset=utf8")
        .body(ANGRY_ASCII)
}

/// HTML form that posts to `/post` URL
pub fn form(req: HttpRequest<State>) -> Result<HttpResponse> {
    render(req, "forms-post.html", &Context::default())
}
