use volo_http::server::{
    extract::Json,
    route::{get, Router},
};

const JSON_STR: &str = r#"{
  "slideshow": {
    "author": "Yours Truly",
    "date": "date of publication",
    "slides": [
      {
        "title": "Wake up to WonderWidgets!",
        "type": "all"
      },
      {
        "items": [
          "Why <em>WonderWidgets</em> are great",
          "Who <em>buys</em> WonderWidgets"
        ],
        "title": "Overview",
        "type": "all"
      }
    ],
    "title": "Sample Slide Show"
  }
}"#;

async fn json_response() -> Json<sonic_rs::Value> {
    Json(sonic_rs::from_str(JSON_STR).unwrap_or_default())
}

pub fn router() -> Router {
    Router::new().route("/json", get(json_response))
}
