use axum::Json;
use super::models::{TestRequest, TestResponse};

pub async fn test_echo(
    Json(payload): Json<TestRequest>,
) -> Json<TestResponse> {
    Json(TestResponse {
        message: "Echo test successful".to_string(),
        echo: payload.data,
    })
}
