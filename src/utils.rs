use http::{header, header::HeaderMap, request::Parts, status::StatusCode};
use hyper::body::Incoming;
use mime::Mime;
use serde::de::DeserializeOwned;
use volo_http::{
    body::Body,
    context::ServerContext,
    response::ServerResponse,
    server::{extract::FromRequest, IntoResponse},
};

pub struct PrettyJson<T>(pub T);

impl<T> IntoResponse for PrettyJson<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> ServerResponse {
        let Ok(json) = sonic_rs::to_vec_pretty(&self.0) else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };
        let body = Body::from(json);

        ServerResponse::builder()
            .status(StatusCode::OK)
            .header(
                http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.essence_str(),
            )
            .body(body)
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}

pub async fn try_deserialize<T>(
    cx: &mut ServerContext,
    parts: Parts,
    body: Incoming,
) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let is_json = json_content_type(&parts.headers);
    let Ok(data) = String::from_request(cx, parts, body).await else {
        return Err(String::default());
    };
    if !is_json {
        return Err(data);
    }
    sonic_rs::from_str(&data).map_err(|_| data)
}

fn json_content_type(headers: &HeaderMap) -> bool {
    let content_type = match headers.get(header::CONTENT_TYPE) {
        Some(content_type) => content_type,
        None => {
            return false;
        }
    };

    let content_type = match content_type.to_str() {
        Ok(s) => s,
        Err(_) => {
            return false;
        }
    };

    let mime_type = match content_type.parse::<Mime>() {
        Ok(mime_type) => mime_type,
        Err(_) => {
            return false;
        }
    };

    // `application/json` or `application/json+foo`
    if mime_type.type_() == mime::APPLICATION && mime_type.subtype() == mime::JSON {
        return true;
    }

    // `application/foo+json`
    if mime_type.suffix() == Some(mime::JSON) {
        return true;
    }

    false
}
