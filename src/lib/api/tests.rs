#[cfg(test)]
mod tests {
    use super::super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, Method, header},
        Router,
    };
    use serde_json::json;
    use tower::util::ServiceExt;
    use tower_http::cors::CorsLayer;

    /// Helper function to create a test router
    fn create_test_router() -> Router {
        let cors = CorsLayer::new()
            .allow_origin([
                "http://localhost:30000".parse().unwrap(),
                "http://127.0.0.1:30000".parse().unwrap(),
            ])
            .allow_methods([Method::POST])
            .allow_headers([header::CONTENT_TYPE]);

        Router::new()
            .route("/api/test/echo", axum::routing::post(handlers::test_echo))
            .layer(cors)
    }

    #[tokio::test]
    async fn test_echo_endpoint() {
        let app = create_test_router();

        let request_body = json!({
            "data": {
                "test": "hello",
                "number": 42,
                "array": [1, 2, 3]
            }
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/test/echo")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["message"], "Echo test successful");
        assert_eq!(body_json["echo"]["test"], "hello");
        assert_eq!(body_json["echo"]["number"], 42);
    }
}
