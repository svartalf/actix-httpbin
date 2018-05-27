use std::borrow::Borrow;
use std::time::SystemTime;

use uuid;
use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result, Path};
use actix_web::http::{StatusCode, header};
use actix_web::http::header::Header;
use futures::{future, Future};

use application::State;
use response::{Body, Builder, ResponseFuture};


/// Returns a `HTTP 304` if an `If-Modified-Since` or `If-None-Match` header is present.
///
/// Otherwise, returns the same content as `/get` endpoint.
pub fn cache(req: HttpRequest<State>) -> Box<ResponseFuture> {
    let is_conditional = req.headers().contains_key(header::IF_MODIFIED_SINCE) ||
        req.headers().contains_key(header::IF_NONE_MATCH);

    if is_conditional {
        let resp = req.build_response(StatusCode::NOT_MODIFIED)
            .finish();
        Box::new(future::ok(resp))
    } else {
        let future = Body::new(req)
            .with_url()
            .with_headers()
            .with_origin()
            .with_args()
            .and_then(|body| {
                let uuid = uuid::Uuid::new_v4().hyphenated().to_string();
                let resp = HttpResponse::build(StatusCode::OK)
                    .set(header::LastModified(SystemTime::now().into()))
                    .set(header::ETag(header::EntityTag::new(false, uuid)))
                    .json(body);

                Ok(resp)
            });

        Box::new(future)
    }
}

/// Sets a `Cache-Control` header for `n` seconds.
pub fn cache_control((req, info): (HttpRequest<State>, Path<u32>)) -> Box<ResponseFuture> {
    let future = Body::new(req)
        .with_url()
        .with_headers()
        .with_origin()
        .with_args()
        .and_then(move |body| {
            let resp = HttpResponse::build(StatusCode::OK)
                .set(header::CacheControl(vec![
                    header::CacheDirective::Public,
                    header::CacheDirective::MaxAge(*info),
                ]))
                .json(body);

            Ok(resp)
        });

    Box::new(future)
}

/// Assumes the resource has the given `etag` and responds to `If-None-Match` and `If-Match` headers appropriately.
pub fn etag((req, info): (HttpRequest<State>, Path<String>)) -> Box<ResponseFuture> {
    if let Ok(if_none_match) = header::IfNoneMatch::parse(&req) {
        let matches = match if_none_match {
            header::IfNoneMatch::Any => true,
            header::IfNoneMatch::Items(tags) => tags.iter().any(|entity| entity.tag() == *info)
        };

        if matches {
            let resp = req.build_response(StatusCode::NOT_MODIFIED)
                .set(header::ETag(header::EntityTag::new(false, (*info).to_string())))
                .finish();

            return Box::new(future::ok(resp));
        }
    }

    if let Ok(if_match) = header::IfMatch::parse(&req) {
        let matches = match if_match {
            header::IfMatch::Any => true,
            header::IfMatch::Items(tags) => tags.iter().any(|entity| entity.tag() == *info)
        };

        if !matches {
            return Box::new(future::ok(HttpResponse::PreconditionFailed().finish()))
        }
    }

    let future = Body::new(req)
        .with_url()
        .with_headers()
        .with_origin()
        .with_args()
        .and_then(move |body| {
            let resp = HttpResponse::build(StatusCode::OK)
                .json(body);

            Ok(resp)
        });

    Box::new(future)
}


#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn headers(req: HttpRequest<State>) -> Result<HttpResponse> {
    let mut resp = req.build_response(StatusCode::OK);
    for (key, value) in req.query().iter() {
        resp.header::<&str, &str>(key.borrow(), value.borrow());
    }

    Ok(resp.finish())
}
