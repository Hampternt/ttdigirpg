#[cfg(test)]
mod tests {
    use super::super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode, Method, header},
        Router,
    };
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tower::util::ServiceExt;
    use tower_http::cors::CorsLayer;

    use crate::entities::database::Database;

    /// Helper function to create a test router with a temporary database
    fn create_test_router() -> Router {
        let db = Database::new(":memory:").expect("Failed to create in-memory database");
        let db = Arc::new(Mutex::new(db));

        let cors = CorsLayer::new()
            .allow_origin([
                "http://localhost:30000".parse().unwrap(),
                "http://127.0.0.1:30000".parse().unwrap(),
            ])
            .allow_methods([Method::POST])
            .allow_headers([header::CONTENT_TYPE]);

        Router::new()
            .route("/api/character/controls", axum::routing::post(handlers::update_controls))
            .layer(cors)
            .with_state(db)
    }

    #[tokio::test]
    async fn test_create_character_with_controls() {
        let app = create_test_router();

        let request_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": [
                {
                    "num": 1,
                    "name": "Test Building",
                    "type": "building",
                    "info": "A test building"
                }
            ]
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
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

        assert_eq!(body_json["success"], true);
        assert!(body_json["character_uuid"].is_string());
        assert_eq!(body_json["message"], "Controls updated successfully");
    }

    #[tokio::test]
    async fn test_update_existing_character_controls() {
        let app = create_test_router();

        // First, create a character
        let request_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": [
                {
                    "num": 1,
                    "name": "First Building",
                    "type": "building",
                    "info": "First version"
                }
            ]
        });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Now update the same character
        let update_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": [
                {
                    "num": 1,
                    "name": "Updated Building",
                    "type": "building",
                    "info": "Updated version"
                },
                {
                    "num": 2,
                    "name": "Second Building",
                    "type": "building",
                    "info": "New building"
                }
            ]
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(update_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], true);
    }

    #[tokio::test]
    async fn test_validation_empty_character_name() {
        let app = create_test_router();

        let request_body = json!({
            "character_name": "",
            "game": "Test Campaign",
            "controls": []
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], false);
        assert!(body_json["error"].as_str().unwrap().contains("Character name cannot be empty"));
    }

    #[tokio::test]
    async fn test_validation_empty_game_name() {
        let app = create_test_router();

        let request_body = json!({
            "character_name": "Test Hero",
            "game": "",
            "controls": []
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], false);
        assert!(body_json["error"].as_str().unwrap().contains("Game name cannot be empty"));
    }

    #[tokio::test]
    async fn test_validation_oversized_controls() {
        let app = create_test_router();

        // Create 101 controls (exceeds the limit of 100)
        let mut controls = Vec::new();
        for i in 0..101 {
            controls.push(json!({
                "num": i,
                "name": format!("Control {}", i),
                "type": "building",
                "info": "Test"
            }));
        }

        let request_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": controls
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], false);
        assert!(body_json["error"].as_str().unwrap().contains("Too many controls"));
    }

    #[tokio::test]
    async fn test_validation_empty_control_name() {
        let app = create_test_router();

        let request_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": [
                {
                    "num": 1,
                    "name": "",
                    "type": "building",
                    "info": "Test"
                }
            ]
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], false);
        assert!(body_json["error"].as_str().unwrap().contains("Control name cannot be empty"));
    }

    #[tokio::test]
    async fn test_validation_character_name_too_long() {
        let app = create_test_router();

        let long_name = "a".repeat(101); // Exceeds 100 character limit

        let request_body = json!({
            "character_name": long_name,
            "game": "Test Campaign",
            "controls": []
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(body_json["success"], false);
        assert!(body_json["error"].as_str().unwrap().contains("Character name exceeds maximum length"));
    }

    #[tokio::test]
    async fn test_cors_headers() {
        let app = create_test_router();

        let request_body = json!({
            "character_name": "Test Hero",
            "game": "Test Campaign",
            "controls": []
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/character/controls")
                    .header("content-type", "application/json")
                    .header("origin", "http://localhost:30000")
                    .body(Body::from(request_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Check that CORS headers are present
        let headers = response.headers();
        assert!(headers.contains_key("access-control-allow-origin"));
    }
}
