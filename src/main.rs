#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")] extern crate test;
extern crate url;
extern crate http;
extern crate uuid;
extern crate base64;
extern crate mime;
extern crate actix_web;
extern crate actix_web_httpauth;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate serde_urlencoded;
#[macro_use] extern crate serde_derive;
extern crate linked_hash_map;
extern crate env_logger;
extern crate clap;
extern crate futures;
extern crate bytes;
extern crate rand;
extern crate tokio_timer;
extern crate tera;

use actix_web::server;
use clap::{Arg, App};

mod application;
mod middleware;
mod response;
mod handlers;
mod templates;
mod map;

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let matches = App::new("actix-httpbin")
        .version(env!("CARGO_PKG_VERSION"))
        .about("httpbin.org clone with Rust and actix-web")
        .arg(Arg::with_name("bind")
            .short("b")
            .long("bind")
            .takes_value(true)
        )
        .get_matches();

    let bind = matches.value_of("bind")
        .unwrap_or("127.0.0.1:8080");

    server::new(application::app)
        .backlog(8192)
        .bind(bind).expect(&format!("Unable to bind to {}", bind))
        .run();
}
