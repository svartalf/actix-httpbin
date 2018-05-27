use actix_web::HttpRequest;

use application::State;
use response::{Body, Builder, JsonFuture};


/// Return the requester IP address.
pub fn origin(req: HttpRequest<State>) -> Box<JsonFuture<State>> {
    Box::new(Body::new(req)
        .with_origin()
        .into_json())
}

/// Return the request' `User-Agent` header.
pub fn user_agent(req: HttpRequest<State>) -> Box<JsonFuture<State>> {
    Box::new(Body::new(req)
        .with_user_agent()
        .into_json())
}

/// Return the request' HTTP headers.
pub fn headers(req: HttpRequest<State>) -> Box<JsonFuture<State>> {
    Box::new(Body::new(req)
        .with_headers()
        .into_json())
}
