pub mod card;
pub mod note;

#[cfg(test)]
mod test {
    use axum::body::Body;
    use axum::http::{header, Method, Request, StatusCode};
    use axum::Router;
    use serde_json::Value;
    use tower::ServiceExt;

    pub async fn send_request(app: Router, request: Request<Body>) -> (StatusCode, Value) {
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body = if body_bytes.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&body_bytes).unwrap()
        };
        (status, body)
    }

    pub fn make_get_request(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    pub fn make_post_request(uri: &str, body: Body) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap()
    }
}
