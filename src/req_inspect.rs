use std::collections::BTreeMap;

use http::{header, status::StatusCode};
use serde::Serialize;
use volo::net::Address;
use volo_http::{
    request::ServerRequest,
    server::route::{get, Router},
};

use crate::utils::PrettyJson;

#[derive(Serialize)]
struct HeadersResponse {
    headers: BTreeMap<String, String>,
}

async fn headers_handler(req: ServerRequest) -> PrettyJson<HeadersResponse> {
    let (parts, _) = req.into_parts();
    let mut headers = BTreeMap::new();
    for (k, v) in parts.headers.into_iter() {
        let Some(k) = k else {
            continue;
        };
        let k = k.to_string();
        let v = v.to_str().map(ToOwned::to_owned).unwrap_or_default();
        headers.insert(k, v);
    }
    PrettyJson(HeadersResponse { headers })
}

#[derive(Serialize)]
struct IpResponse {
    origin: String,
}

async fn ip_handler(addr: Address) -> PrettyJson<IpResponse> {
    PrettyJson(IpResponse {
        origin: addr.to_string(),
    })
}

#[derive(Serialize)]
struct UserAgentResponse {
    #[serde(rename = "user-agent")]
    user_agent: String,
}

async fn user_agent_handler(
    req: ServerRequest,
) -> Result<PrettyJson<UserAgentResponse>, StatusCode> {
    let Some(user_agent) = req.headers().get(header::USER_AGENT) else {
        return Err(StatusCode::BAD_REQUEST);
    };
    let Ok(user_agent) = user_agent.to_str().map(ToOwned::to_owned) else {
        return Err(StatusCode::BAD_REQUEST);
    };
    Ok(PrettyJson(UserAgentResponse { user_agent }))
}

pub fn router() -> Router {
    Router::new()
        .route("/headers", get(headers_handler))
        .route("/ip", get(ip_handler))
        .route("/user-agent", get(user_agent_handler))
}
