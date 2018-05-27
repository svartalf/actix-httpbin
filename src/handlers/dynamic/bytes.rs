use std::cmp;
use std::mem;

use actix_web::{HttpRequest, HttpResponse, Path, Query};
use actix_web::http::StatusCode;
use rand::{self, XorShiftRng, Rng, SeedableRng};

use application::State;

#[derive(Deserialize)]
pub struct BytesQuery {
    seed: Option<u64>,
}

/// Returns `n` random bytes generated with given seed.
pub fn bytes((req, path, query): (HttpRequest<State>, Path<u32>, Query<BytesQuery>)) -> HttpResponse {
    let n = cmp::min(*path, 100 * 1024);

    let seed = if let Some(seed) = (*query).seed {
        unsafe { mem::transmute(u128::from(seed)) }
    } else {
        [rand::random(), rand::random(), rand::random(), rand::random()]
    };

    let mut rng = XorShiftRng::from_seed(seed);
    let mut body = vec![0u8; n as usize];
    rng.fill_bytes(&mut body);

    req.build_response(StatusCode::OK)
        .content_type("application/octet-stream")
        .body(body)
}

