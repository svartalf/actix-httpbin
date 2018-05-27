use std::cmp;
use std::mem;

use rand::{self, XorShiftRng, Rng, SeedableRng};
use bytes::Bytes;
use actix_web::{HttpRequest, HttpResponse, Path, Query};
use actix_web::http::StatusCode;
use futures::{Stream, Poll, Async};

use application::State;
use response:: Error;


#[derive(Deserialize)]
pub struct StreamBytesQuery {
    seed: Option<u64>,
    chunk_size: Option<u64>,
}

impl StreamBytesQuery {
    fn seed(&self) -> [u32; 4] {
        if let Some(seed) = self.seed {
            unsafe { mem::transmute(u128::from(seed)) }
        } else {
            [rand::random(), rand::random(), rand::random(), rand::random()]
        }
    }

    fn chunk_size(&self) -> usize {
        if let Some(size) = self.chunk_size {
            cmp::max(1, size) as usize
        } else {
            10 * 1024
        }
    }
}

pub struct StreamBytes {
    limit: usize,
    sent: usize,
    chunk_size: usize,
    rng: XorShiftRng,
}

impl StreamBytes {
    pub fn new(limit: usize, seed: [u32; 4], chunk_size: usize) -> StreamBytes {
        StreamBytes {
            sent: 0,
            limit: limit,
            rng: XorShiftRng::from_seed(seed),
            chunk_size: chunk_size,
        }
    }
}

impl Stream for StreamBytes {
    type Item = Bytes;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Bytes>, Error> {
        if self.sent >= self.limit {
            return Ok(Async::Ready(None))
        }

        let size = cmp::min(self.chunk_size, self.limit - self.sent);
        let mut buffer = vec![0; size];
        self.rng.fill_bytes(&mut buffer);
        self.sent += size;

        Ok(Async::Ready(Some(Bytes::from(buffer))))
    }
}

/// Streams `n` random bytes generated with given seed, at given chuck size per packet.
pub fn stream_bytes((req, path, query): (HttpRequest<State>, Path<u8>, Query<StreamBytesQuery>)) -> HttpResponse {
    let stream = StreamBytes::new(
        usize::from(*path),
        query.seed(),
        query.chunk_size());

    req.build_response(StatusCode::OK)
        .content_type("application/octet-stream")
        .streaming(stream)
}
