extern crate pretty_env_logger;
extern crate warp;

use std::thread;
use std::time::Duration;

use warp::Filter;

fn main() {
    pretty_env_logger::init();

    // blocking pool of 4 threads
    let sleep = warp::blocking(4, |x: u8| {
        thread::sleep(Duration::from_secs(x.into()));
        x.to_string()
    });

    // GET /:num => waits `num` seconds before responding.
    let routes = warp::get(
        warp::path::param()
            .and_then(sleep)
    );

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030));
}
