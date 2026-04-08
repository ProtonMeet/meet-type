use serde::{Deserialize, Serialize};

/// Message for joining a room
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRoomMessage {
    pub room_id: String,
}

/// Response after attempting to join a room
#[derive(Debug, Serialize, Deserialize)]
pub struct JoinRoomResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// Message for leaving a room
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRoomMessage {
    pub room_id: String,
}

/// Response after attempting to leave a room
#[derive(Debug, Serialize, Deserialize)]
pub struct LeaveRoomResponse {
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupInfoSummaryResponse {
    pub meeting_id: String,
    pub epoch: Option<u64>,
    pub group_id: Option<Vec<u8>>,
    pub version: Option<u32>,
}

/// Known command kind. Useful for pointing to the failed command in error responses.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WebSocketCommandKind {
    JoinRoom,
    LeaveRoom,
    GroupInfoSummary,
}

/// Generic error response for websocket text commands.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebSocketTextErrorResponse {
    pub command: Option<WebSocketCommandKind>,
    pub code: String,
    pub message: String,
}
