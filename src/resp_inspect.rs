use std::collections::BTreeMap;

use volo_http::{
    body::Body,
    response::ServerResponse,
    server::{
        extract::{Form, Query},
        route::{get, Router},
    },
};

fn apply_headermap(map: BTreeMap<String, String>) -> ServerResponse {
    let mut builder = ServerResponse::builder();
    for (k, v) in map {
        builder = builder.header(k, v);
    }
    builder.body(Body::empty()).unwrap_or_default()
}

async fn response_headers_from_query(
    Query(map): Query<BTreeMap<String, String>>,
) -> ServerResponse {
    apply_headermap(map)
}

async fn response_headers_from_form(Form(map): Form<BTreeMap<String, String>>) -> ServerResponse {
    apply_headermap(map)
}

pub fn router() -> Router {
    Router::new().route(
        "/response-headers",
        get(response_headers_from_query).post(response_headers_from_form),
    )
}
