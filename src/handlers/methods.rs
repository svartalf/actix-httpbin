use actix_web::HttpRequest;

use application::State;
use response::{Body, Builder, JsonFuture};

/// The request's query parameters.
pub fn get(req: HttpRequest<State>) -> Box<JsonFuture<State>> {
    Box::new(Body::new(req)
        .with_url()
        .with_headers()
        .with_origin()
        .with_args()
        .into_json())
}

/// Returns anything passed in request data.
pub fn anything(req: HttpRequest<State>) -> Box<JsonFuture<State>> {
    Box::new(Body::new(req)
        .with_method()
        .with_url()
        .with_headers()
        .with_origin()
        .with_args()
        .into_json())
}
