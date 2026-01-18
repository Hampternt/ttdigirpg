use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::entities::database::Database;
use super::models::{ErrorResponse, SuccessResponse, UpdateControlsRequest};

pub async fn update_controls(
    State(db): State<Arc<Mutex<Database>>>,
    Json(payload): Json<UpdateControlsRequest>,
) -> Result<Json<SuccessResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate input first (no lock needed)
    if let Err(e) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Validation error: {}", e),
            }),
        ));
    }

    // Prepare JSON data for controls (no lock needed)
    let controls_json = json!(payload.controls);

    // Lock only for database operations
    let character_uuid = {
        let db = db.lock().await;

        // Query database for existing character
        let existing_character = db
            .get_character(&payload.character_name, &payload.game)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        success: false,
                        error: format!("Database query error: {}", e),
                    }),
                )
            })?;

        match existing_character {
            Some((uuid, _name, _game, data)) => {
                // Character exists - update controls in existing data
                let mut character_data: Value = if let Some(data_str) = data {
                    serde_json::from_str(&data_str).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                success: false,
                                error: format!("Failed to parse existing character data: {}", e),
                            }),
                        )
                    })?
                } else {
                    json!({})
                };

                // Replace or add controls array
                character_data["controls"] = controls_json;

                // Serialize back to string
                let updated_data = serde_json::to_string(&character_data).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            success: false,
                            error: format!("Failed to serialize character data: {}", e),
                        }),
                    )
                })?;

                // Update in database
                db.update_character(&payload.character_name, &payload.game, &updated_data)
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                success: false,
                                error: format!("Failed to update character: {}", e),
                            }),
                        )
                    })?;

                uuid
            }
            None => {
                // Character doesn't exist - create new with controls
                let character_data = json!({
                    "controls": controls_json
                });

                let data_str = serde_json::to_string(&character_data).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            success: false,
                            error: format!("Failed to serialize new character data: {}", e),
                        }),
                    )
                })?;

                db.insert_character(&payload.character_name, &payload.game, Some(&data_str))
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ErrorResponse {
                                success: false,
                                error: format!("Failed to insert character: {}", e),
                            }),
                        )
                    })?
            }
        }
    }; // Lock released here

    // Parse UUID for response (no lock needed)
    let uuid = uuid::Uuid::parse_str(&character_uuid).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Invalid UUID format: {}", e),
            }),
        )
    })?;

    Ok(Json(SuccessResponse {
        success: true,
        character_uuid: uuid,
        message: "Controls updated successfully".to_string(),
    }))
}
