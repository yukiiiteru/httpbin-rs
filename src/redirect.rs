use http::{header, StatusCode};
use serde::Deserialize;
use volo_http::{
    request::Request,
    server::{
        extract::Query,
        route::{any, get, Router},
        Redirect,
    },
    PathParams,
};

async fn absolute_redirect_handler(PathParams(num): PathParams<usize>, req: Request) -> Redirect {
    let host = req
        .headers()
        .get(header::HOST)
        .unwrap()
        .to_str()
        .map(ToOwned::to_owned)
        .unwrap_or_default();
    if num <= 1 {
        Redirect::found(&format!("http://{host}/get"))
    } else {
        let num = num - 1;
        Redirect::found(&format!("http://{host}/absolute-redirect/{num}"))
    }
}

async fn relative_redirect_handler(PathParams(num): PathParams<usize>) -> Redirect {
    if num <= 1 {
        Redirect::found("/get")
    } else {
        let num = num - 1;
        Redirect::found(&format!("/absolute-redirect/{num}"))
    }
}

#[derive(Deserialize)]
struct RedirectTo {
    url: String,
    status_code: Option<u16>,
}

async fn redirect_to(Query(redirect): Query<RedirectTo>) -> Redirect {
    let status_code = match redirect
        .status_code
        .map(StatusCode::from_u16)
        .and_then(Result::ok)
    {
        Some(status_code) if status_code.is_redirection() => status_code,
        _ => StatusCode::FOUND,
    };
    Redirect::with_status_code(status_code, &redirect.url)
}

pub fn router() -> Router {
    Router::new()
        .route("/absolute-redirect/{num}", get(absolute_redirect_handler))
        .route("/relative-redirect/{num}", get(relative_redirect_handler))
        .route("/redirect-to", any(redirect_to))
}
