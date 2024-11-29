use std::time::Duration;

use async_stream::stream;
use bytes::{BufMut, Bytes, BytesMut};
use http::header::{self, HeaderName};
use http_body::Frame;
use serde::Serialize;
use volo_http::{
    body::Body,
    response::Response,
    server::route::{any, get, Router},
    PathParams,
};

use crate::{method::RequestInfo, utils::PrettyJson};

async fn bytes_handler(PathParams(num): PathParams<usize>) -> ((HeaderName, &'static str), Bytes) {
    let mut res = BytesMut::with_capacity(num);
    for _ in 0..num {
        res.put_u8(rand::random())
    }
    (
        (
            header::CONTENT_TYPE,
            mime::APPLICATION_OCTET_STREAM.essence_str(),
        ),
        res.freeze(),
    )
}

async fn delay_handler(
    PathParams(delay): PathParams<u64>,
    req_info: RequestInfo,
) -> PrettyJson<RequestInfo> {
    let secs = if delay > 10 { 10 } else { delay };
    tokio::time::sleep(Duration::from_secs(secs)).await;
    PrettyJson(req_info)
}

async fn stream_data_handler(PathParams(num): PathParams<usize>) -> Response {
    let stream = stream! {
        for _ in 0..num {
            yield Ok(Frame::data(Bytes::copy_from_slice(&[rand::random()])))
        }
    };
    let body = Body::from_stream(stream);
    Response::builder()
        .header(
            header::CONTENT_TYPE,
            mime::APPLICATION_OCTET_STREAM.essence_str(),
        )
        .header(header::CONTENT_LENGTH, num)
        .body(body)
        .unwrap_or_default()
}

#[derive(Serialize)]
struct StreamJson {
    #[serde(flatten)]
    req_info: RequestInfo,
    id: usize,
}

async fn stream_handler(PathParams(num): PathParams<usize>, req_info: RequestInfo) -> Response {
    let mut info = StreamJson { req_info, id: 0 };
    let stream = stream! {
        for i in 0..num {
            info.id = i;
            let vec = sonic_rs::to_vec(&info).unwrap_or_default();
            yield Ok(Frame::data(Bytes::from(vec)));
        }
    };
    let body = Body::from_stream(stream);
    Response::builder()
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.essence_str())
        .body(body)
        .unwrap_or_default()
}

pub fn router() -> Router {
    Router::new()
        .route("/bytes/{num}", get(bytes_handler))
        .route("/delay/{delay}", any(delay_handler))
        .route("/stream-bytes/{num}", get(stream_data_handler))
        .route("/stream/{num}", get(stream_handler))
}
