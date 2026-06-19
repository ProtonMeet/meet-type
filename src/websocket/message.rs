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

/// Agent-side join: agent sends its KeyPackage (and HPKE public key) for the
/// host to seal a PSK against. `Credential` is the same string carried in the KP.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UploadKeyPackageMessage {
    /// Base64 of the TLS-encoded signed KeyPackage.
    pub key_package: String,
    /// Base64 of the X25519 HPKE public key bytes.
    pub hpke_public_key: String,
    /// String credential.
    pub credential: String,
}

/// Server ack of an [`UploadKeyPackageMessage`].
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UploadKeyPackageResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// Agent lifecycle status the server can surface to humans (e.g. "captions on").
/// `Status` values are open-ended but expected: `"ready" | "error" | "done"`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AgentStatusMessage {
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AgentStatusResponse {
    pub success: bool,
    pub error: Option<String>,
}

/// Known command kind. Useful for pointing to the failed command in error responses.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WebSocketCommandKind {
    JoinRoom,
    LeaveRoom,
    GroupInfoSummary,
    UploadKeyPackage,
    AgentStatus,
}

/// Generic error response for websocket text commands.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebSocketTextErrorResponse {
    pub command: Option<WebSocketCommandKind>,
    pub code: String,
    pub message: String,
}
