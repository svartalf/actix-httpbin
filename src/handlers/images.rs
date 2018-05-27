use mime;
use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::http::header::{self, Header};

use application::State;

static IMAGE_PNG: &'static [u8] = include_bytes!("../assets/images/pig_icon.png");
static IMAGE_JPEG: &'static [u8] = include_bytes!("../assets/images/jackal.jpg");
static IMAGE_WEBP: &'static [u8] = include_bytes!("../assets/images/wolf_1.webp");
static IMAGE_SVG: &'static [u8] = include_bytes!("../assets/images/svg_logo.svg");


/// Returns a simple image of the type suggested by the `Accept` header.
pub fn image(req: HttpRequest<State>) -> HttpResponse {
    if let Ok(accept) = header::Accept::parse(&req) {
        for qi in accept.0 {
            match () {
                () if qi.item == mime::IMAGE_PNG => return image_png(req),
                () if qi.item == mime::IMAGE_JPEG => return image_jpeg(req),
                () if qi.item == "image/webp" => return image_webp(req),
                () if qi.item == "image/svg+xml" => return image_svg(req),
                _ => continue,
            }
        }
    }

    image_png(req)
}

/// Returns a PNG image.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn image_png(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("image/png")
        .body(IMAGE_PNG)
}

/// Returns a JPG image.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn image_jpeg(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("image/jpeg")
        .body(IMAGE_JPEG)
}

/// Returns a WEBP image.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn image_webp(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("image/webp")
        .body(IMAGE_WEBP)
}

/// Returns a SVG image.
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn image_svg(req: HttpRequest<State>) -> HttpResponse {
    req.build_response(StatusCode::OK)
        .content_type("image/svg+xml")
        .body(IMAGE_SVG)
}
