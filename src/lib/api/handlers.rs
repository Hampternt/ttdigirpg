use axum::{Json, http::Response};
use rusqlite::ffi::sqlite3_memory_used;
use crate::{api::models::PingResponse, entities::database::Database};

use super::models::{TestRequest, TestResponse};

fn handle_db_result(result: Result<String, rusqlite::Error>) -> Json<TestResponse> {
    match result {
        Ok(data) => Json(TestResponse {
            message: "Success".to_string(),
            echo: serde_json::json!({"success": true, "data": data}),
        }),
        Err(e) => Json(TestResponse {
            message: format!("Failed: {}", e),
            echo: serde_json::json!({"success": false}),
        })
    }
}


/// Test echo handler.
///
/// # Arguments
/// * `Json(payload): Json<TestRequset>` - is the boyd of the api request being passed to the echo.
///
/// # Returns
/// * `Json<TestResponse>` - returns a resposne affirming the receival af the api trigger or echoes
/// back the original payload. I cant fully remember.
pub async fn test_echo(
    Json(payload): Json<TestRequest>,
) -> Json<TestResponse> {
    Json(TestResponse {
        message: "Echo test successful".to_string(),
        echo: payload.data,
    })
}

pub async fn create_db() -> Json<TestResponse> {
    let result = Database::new("src/database/test_api.db")
        .map(|_| "Database created".to_string());

    handle_db_result(result)
}

// pub async fn add_character(
//     Json(payload): Json<TestRequest>) 
// -> Json<TestResponse> {
//     let result = Database::new("src/database/test_api.db")
//         .and_then(|db| db.insert_character("Test Hero", "Test Game", Some(r#"{"level": 1}"#)));
//
//
//     handle_db_result(result)
// }

pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse { message: "".to_string()})
}
