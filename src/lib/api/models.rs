use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct TestRequest {
    pub data: Value,  // Accept any JSON
}

#[derive(Debug, Serialize)]
pub struct TestResponse {
    pub message: String,
    pub echo: Value,  // Echo back whatever was sent
}
