use http::status::StatusCode;
use volo_http::{
    server::route::{any, Router},
    PathParams,
};

async fn status_handler(PathParams(status): PathParams<u16>) -> StatusCode {
    StatusCode::from_u16(status).unwrap_or(StatusCode::BAD_REQUEST)
}

pub fn router() -> Router {
    Router::new().route("/status/{status}", any(status_handler))
}
