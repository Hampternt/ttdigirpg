use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateControlsRequest {
    pub character_name: String,
    pub game: String,
    pub controls: Vec<ControlItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ControlItem {
    pub num: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub info: String,
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
