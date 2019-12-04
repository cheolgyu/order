#![deny(warnings)]
extern crate hyper;
extern crate pretty_env_logger;

use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};

fn main() {
    pretty_env_logger::init();
    let addr = ([127, 0, 0, 1], 3003).into();

    let _api_key = "3xqNQimLXHJovDSRVPDnLdymk9HR1qLdAtFhcmkb";
    let _api_secret="LGpty7tGavt3C06sqECtaABMjNVzZ2Rf88EF3tM2";


    let server = Server::bind(&addr)
        .serve(|| {
            // This is the `Service` that will handle the connection.
            // `service_fn_ok` is a helper to convert a function that
            // returns a Response into a `Service`.
            service_fn_ok(move |_: Request<Body>| Response::new(Body::from("Hello World!")))
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
