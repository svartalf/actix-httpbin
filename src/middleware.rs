use actix_web::http::header::HeaderValue;
use actix_web::{HttpRequest, HttpResponse, HttpMessage, Result};
use actix_web::middleware::{Middleware, Response};
use actix_web::http::header;

pub struct Headers;

impl<S> Middleware<S> for Headers {

    fn response(&self, req: &mut HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        let origin = req.headers().get(header::ORIGIN)
            .and_then(|value| Some(value.clone()))
            .unwrap_or_else(|| HeaderValue::from_static("*"));

        {
            let headers = resp.headers_mut();

            headers.insert(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("'true'"));
            headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin);
            headers.insert("X-Powered-By", HeaderValue::from_static("actix-httpbin"));
        }

        Ok(Response::Done(resp))
    }

}
