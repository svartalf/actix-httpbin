use actix_web::{HttpRequest, HttpResponse, Result, Query};
use actix_web::http::header::ContentRange;
use actix_web::http::StatusCode;


#[derive(Debug, Deserialize)]
pub struct RangeQuery {
    chunk_size: Option<u16>,
    duration: Option<u8>,
}


pub fn range((req, info, query): (HttpRequest<State>, Path<u16>, Query<RangeQuery>)) -> Result<HttpResponse> {
    let num_bytes = *info;

    Ok(req.build_response(StatusCode::OK)
        .finish())
}
