use std::net::SocketAddr;

use volo::net::Address;
use volo_http::server::{route::any, Router, Server};

async fn index() -> &'static str {
    "It Works!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", any(index))
        .merge(httpbin::method::router())
        .merge(httpbin::status::router())
        .merge(httpbin::req_inspect::router())
        .merge(httpbin::resp_inspect::router())
        .merge(httpbin::resp_format::router())
        .merge(httpbin::dyn_data::router());

    let addr = "[::]:8080".parse::<SocketAddr>().unwrap();
    let addr = Address::from(addr);

    Server::new(app).run(addr).await.unwrap();
}
