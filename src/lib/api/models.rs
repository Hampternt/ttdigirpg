use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateControlsRequest {
    pub character_name: String,
    pub game: String,
    pub controls: Vec<ControlItem>,
}

impl UpdateControlsRequest {
    /// Validates the request data to ensure all fields meet requirements
    pub fn validate(&self) -> Result<(), String> {
        // Validate character_name
        if self.character_name.trim().is_empty() {
            return Err("Character name cannot be empty".to_string());
        }
        if self.character_name.len() > 100 {
            return Err("Character name exceeds maximum length (100)".to_string());
        }

        // Validate game
        if self.game.trim().is_empty() {
            return Err("Game name cannot be empty".to_string());
        }
        if self.game.len() > 100 {
            return Err("Game name exceeds maximum length (100)".to_string());
        }

        // Validate controls array
        if self.controls.len() > 100 {
            return Err("Too many controls (maximum 100)".to_string());
        }

        // Validate each control item
        for (idx, control) in self.controls.iter().enumerate() {
            control.validate().map_err(|e| format!("Control {}: {}", idx + 1, e))?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlItem {
    pub num: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub info: String,
}

impl ControlItem {
    /// Validates a single control item
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Control name cannot be empty".to_string());
        }
        if self.name.len() > 200 {
            return Err("Control name too long (max 200)".to_string());
        }
        if self.item_type.trim().is_empty() {
            return Err("Control type cannot be empty".to_string());
        }
        if self.info.len() > 1000 {
            return Err("Control info too long (max 1000)".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub character_uuid: Uuid,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}
