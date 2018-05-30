use actix_web::App;
use actix_web::http::Method;
use actix_web::middleware::Logger;
use tera::Tera;

use middleware;
use handlers;
use templates;

pub struct State {
    pub template: Tera,
}

pub fn app() -> App<State> {
    let state = State{
        template: templates::create_tera().expect("Templates are not bundled"),
    };

    App::with_state(state)
        .middleware(Logger::default())
        .middleware(middleware::Headers)
        .resource("/", |r| r.with(handlers::index))

        // HTTP methods
        .resource("/get", |r| r.method(Method::GET).with(handlers::methods::get))
        // TODO: /post
        // TODO: /patch
        // TODO: /put
        // TODO: /delete

        // Auth
        .resource("/basic-auth/{user}/{passwd}", |r| r.with(handlers::auth::basic_auth))
        .resource("/bearer", |r| r.with(handlers::auth::bearer_auth))
        // TODO: /digest-auth/{qop}/{user}{passwd}
        // TODO: /digest-auth/{qop}/{user}{passwd}/{algorithm}
        // TODO: /digest-auth/{qop}/{user}{passwd}/{algorithm}/{stale_after}
        // TODO: /digest-auth/{qop}/{user}{passwd}
        // TODO: /hidden-basic-auth/{user}/{passwd}

        // Status codes
        .resource("/status/{code}", |r| r.with(handlers::statuses::status))

        // Request inspection
        .resource("/ip", |r| r.f(handlers::request::origin))
        .resource("/headers", |r| r.with(handlers::request::headers))
        .resource("/user-agent", |r| r.f(handlers::request::user_agent))

        // Response inspection
        .resource("/cache", |r| r.with(handlers::response::cache))
        .resource("/cache/{n}", |r| r.with(handlers::response::cache_control))
        .resource("/etag/{etag}", |r| r.with(handlers::response::etag))
        .resource("/response-headers", |r| r.with(handlers::response::headers))
        // TODO: /response-headers

        // Response formats
        .resource("/gzip", |r| r.with(handlers::formats::gzip))
        .resource("/deflate", |r| r.with(handlers::formats::deflate))
        .resource("/brotli", |r| r.with(handlers::formats::brotli))
        .resource("/encoding/utf8", |r| r.with(handlers::formats::encoding_utf8))
        .resource("/html", |r| r.with(handlers::formats::html))
        .resource("/robots.txt", |r| r.with(handlers::formats::robots_txt))
        .resource("/deny", |r| r.with(handlers::formats::deny))
        .resource("/json", |r| r.with(handlers::formats::json))
        .resource("/xml", |r| r.with(handlers::formats::xml))
        .resource("/forms/post", |r| r.with(handlers::formats::form))

        // Dynamic data
        .resource("/uuid", |r| r.with(handlers::dynamic::uuid))
        .resource("/base64/{value}", |r| r.with(handlers::dynamic::base64))
        .resource("/stream/{n}", |r| r.with(handlers::dynamic::stream))
        .resource("/delay/{n}", |r| r.with(handlers::dynamic::delay))
        .resource("/bytes/{n}", |r| r.with(handlers::dynamic::bytes))
        .resource("/stream-bytes/{n}", |r| r.with(handlers::dynamic::stream_bytes))
        .resource("/drip", |r| r.with(handlers::dynamic::drip))
        // TODO: /range/1024
        .resource("/links/{total}/{current}", |r| r.with(handlers::dynamic::links))

        // Cookies
        .resource("/cookies", |r| r.with(handlers::cookies::cookies))
        .resource("/cookies/set", |r| r.with(handlers::cookies::cookies_set))
        .resource("/cookies/set/{name}/{value}", |r| r.with(handlers::cookies::cookie_set))
        .resource("/cookies/delete", |r| r.with(handlers::cookies::cookies_delete))

        // Images
        .resource("/image", |r| r.with(handlers::images::image))
        .resource("/image/png", |r| r.with(handlers::images::image_png))
        .resource("/image/jpeg", |r| r.with(handlers::images::image_jpeg))
        .resource("/image/webp", |r| r.with(handlers::images::image_webp))
        .resource("/image/svg", |r| r.with(handlers::images::image_svg))

        // Redirects
        .resource("/absolute-redirect/{n}", |r| r.with(handlers::redirects::absolute_redirect_n))
        .resource("/redirect-to", |r| r.with(handlers::redirects::redirect_to))
        .resource("/redirect/{n}", |r| r.with(handlers::redirects::redirect_n))
        .resource("/relative-redirect/{n}", |r| r.with(handlers::redirects::relative_redirect_n))

        // Anything
        .resource("/anything", |r| r.with(handlers::methods::anything))
        // TODO: /anything/{anything}



}
