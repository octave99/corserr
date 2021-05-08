#[macro_use]
extern crate serde;

use warp::{
    http::{header, Method},
    Filter, Rejection, Reply,
};

mod response;
pub use response::Response;

mod error;
pub use error::{handle_rejection, Error, Result};

mod handler;

fn cors() -> warp::cors::Cors {
    warp::cors()
        .allow_credentials(true)
        .allow_origin("http://localhost:3000")
        .allow_origin("http://localhost:3001")
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(&[
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
        ])
        .max_age(3600)
        .build()
}

pub fn routes() -> impl warp::Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let working_path = warp::path("yes")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handler::working);

    let not_working_path = warp::path("no")
        .and(warp::get())
        .and(warp::path::end())
        .and_then(handler::not_working);

    working_path.or(not_working_path)
}
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let routes = routes()
    .recover(handle_rejection)
    .with(warp::log("cors test"))
    .with(cors());

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;

    Ok(())
}
