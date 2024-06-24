use std::{collections::BTreeMap, convert::Infallible};

use http::{
    header::{self, HeaderValue},
    method::Method,
    request::Parts,
};
use hyper::body::Incoming;
use serde::Serialize;
use volo::context::Context;
use volo_http::{
    context::ServerContext,
    error::ExtractBodyError,
    server::{
        extract::{FromContext, FromRequest, Query},
        route::{delete, get, patch, post, put, Router},
    },
};

use crate::utils::{try_deserialize, PrettyJson};

#[derive(Serialize)]
struct RequestData {
    data: String,
    json: sonic_rs::Value,
}

impl FromRequest for RequestData {
    type Rejection = ExtractBodyError;

    async fn from_request(
        cx: &mut ServerContext,
        parts: Parts,
        body: Incoming,
    ) -> Result<Self, Self::Rejection> {
        let res = match try_deserialize(cx, parts, body).await {
            Ok(json) => Self {
                data: Default::default(),
                json,
            },
            Err(data) => Self {
                data,
                json: Default::default(),
            },
        };
        Ok(res)
    }
}

#[derive(Serialize)]
pub(crate) struct RequestInfo {
    url: String,
    origin: String,
    args: BTreeMap<String, String>,
    headers: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    data: Option<RequestData>,
}

impl FromRequest for RequestInfo {
    type Rejection = Infallible;

    async fn from_request(
        cx: &mut ServerContext,
        mut parts: Parts,
        body: Incoming,
    ) -> Result<Self, Self::Rejection> {
        let host = parts
            .headers
            .get(header::HOST)
            .map(HeaderValue::to_str)
            .map(Result::unwrap)
            .map(ToOwned::to_owned)
            .unwrap_or_default();
        let uri = &parts.uri;
        let url = format!("http://{host}{uri}");
        let origin = cx.rpc_info().caller().address().unwrap().to_string();
        let headers = parts
            .headers
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.to_str().map(ToOwned::to_owned).unwrap_or_default(),
                )
            })
            .collect();
        let args = Query::from_context(cx, &mut parts)
            .await
            .unwrap_or_default()
            .0;
        let data = if parts.method != Method::GET {
            RequestData::from_request(cx, parts, body).await.ok()
        } else {
            None
        };

        Ok(Self {
            url,
            origin,
            args,
            headers,
            data,
        })
    }
}

async fn method_handler(req_info: RequestInfo) -> PrettyJson<RequestInfo> {
    PrettyJson(req_info)
}

pub fn router() -> Router {
    Router::new()
        .route("/delete", delete(method_handler))
        .route("/get", get(method_handler))
        .route("/patch", patch(method_handler))
        .route("/post", post(method_handler))
        .route("/put", put(method_handler))
}
