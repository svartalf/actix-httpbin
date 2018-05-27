# actix-httpbin

[httpbin.org](https://httpbin.org) clone with an [actix-web](https://actix.rs) under the hood.

## Advantages

 1. [Rust](https://rust-lang.org) is cool
 2. All images, templates and other files are bundled into one binary, so you can just run it;
 there is no need in python anymore.
 3. Amazingly fast, low CPU and memory footprint

## API

Till `1.0.0` version it will follow the `httpbin(1)` API,
so any requests for changes that are not present at the `https://httpbin.org`
will be closed or postponed.

After the `1.0.0` I might consider to go separately, but let's just reach the first milestone, are'nt we?

Any way, if you need to get list of all available endpoints,
run and open it in your browser.

### Differences

There are few known (and maybe some unknown) differences from the `httpbin(1)` right now.

 1. `"headers"` object keys in the `/get`, `/post` and other similar endpoints are lower-cased,
 it comes from the `http` crate in that way and I'm not sure if I should force some Camel-Dashed-Case on them
 2. Many methods may respond not only to `GET` method, it is not intended.
 If you think that this behaviour is wrong, do not hesitate to create an issue.
 3. Not all methods are implemented for now,
 you can find them by searching for `TODO` comment in the `src/application.rs` file

## How to run?

### Via Docker

    $ docker run -p 80:80 svartalf/actix-httpbin

### Install with `cargo`

    $ cargo install actix-httpbin

### From sources

    $ git clone https://github.com/svartalf/actix-httpbin.git
    $ cargo run --release
