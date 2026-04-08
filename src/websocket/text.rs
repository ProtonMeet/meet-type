use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use super::message::{
    GroupInfoSummaryResponse, JoinRoomMessage, JoinRoomResponse, LeaveRoomMessage,
    LeaveRoomResponse, WebSocketTextErrorResponse,
};

/// WebSocket text request envelope sent from client to server.
/// `request_id` can be used to correlate a server response with this request.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebSocketTextRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub command: WebSocketTextRequestCommand,
}

/// Request command for WebSocket text messages.
/// Add new variants here to support additional commands in the future.
#[derive(Debug)]
pub enum WebSocketTextRequestCommand {
    JoinRoom(JoinRoomMessage),
    LeaveRoom(LeaveRoomMessage),
    GroupInfoSummary,
    Unknown(UnknownWebSocketCommand),
}

/// WebSocket text response envelope sent from server to client.
/// `request_id` should match the request when this is a direct response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WebSocketTextResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(flatten)]
    pub command: WebSocketTextResponseCommand,
}

/// Response command for WebSocket text messages.
/// Success responses mirror command variants; `Error` is generic.
#[derive(Debug)]
pub enum WebSocketTextResponseCommand {
    JoinRoom(JoinRoomResponse),
    LeaveRoom(LeaveRoomResponse),
    GroupInfoSummary(GroupInfoSummaryResponse),
    Error(WebSocketTextErrorResponse),
    Unknown(UnknownWebSocketCommand),
}

/// Unknown command payload passthrough.
/// This keeps parsing forward-compatible when new commands are introduced.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownWebSocketCommand {
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WebSocketCommandWire {
    command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payload: Option<serde_json::Value>,
}

impl Serialize for WebSocketTextRequestCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire = match self {
            WebSocketTextRequestCommand::JoinRoom(payload) => WebSocketCommandWire {
                command: "JoinRoom".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextRequestCommand::LeaveRoom(payload) => WebSocketCommandWire {
                command: "LeaveRoom".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextRequestCommand::GroupInfoSummary => WebSocketCommandWire {
                command: "GroupInfoSummary".to_string(),
                payload: None,
            },
            WebSocketTextRequestCommand::Unknown(payload) => WebSocketCommandWire {
                command: payload.command.clone(),
                payload: payload.payload.clone(),
            },
        };

        wire.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WebSocketTextRequestCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = WebSocketCommandWire::deserialize(deserializer)?;
        match wire.command.as_str() {
            "JoinRoom" => {
                let payload = wire
                    .payload
                    .ok_or_else(|| de::Error::custom("Missing payload for JoinRoom command"))?;
                serde_json::from_value(payload)
                    .map(WebSocketTextRequestCommand::JoinRoom)
                    .map_err(de::Error::custom)
            }
            "LeaveRoom" => {
                let payload = wire
                    .payload
                    .ok_or_else(|| de::Error::custom("Missing payload for LeaveRoom command"))?;
                serde_json::from_value(payload)
                    .map(WebSocketTextRequestCommand::LeaveRoom)
                    .map_err(de::Error::custom)
            }
            "GroupInfoSummary" => Ok(WebSocketTextRequestCommand::GroupInfoSummary),
            _ => Ok(WebSocketTextRequestCommand::Unknown(
                UnknownWebSocketCommand {
                    command: wire.command,
                    payload: wire.payload,
                },
            )),
        }
    }
}

impl Serialize for WebSocketTextResponseCommand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let wire = match self {
            WebSocketTextResponseCommand::JoinRoom(payload) => WebSocketCommandWire {
                command: "JoinRoom".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextResponseCommand::LeaveRoom(payload) => WebSocketCommandWire {
                command: "LeaveRoom".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextResponseCommand::GroupInfoSummary(payload) => WebSocketCommandWire {
                command: "GroupInfoSummary".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextResponseCommand::Error(payload) => WebSocketCommandWire {
                command: "Error".to_string(),
                payload: Some(serde_json::to_value(payload).map_err(serde::ser::Error::custom)?),
            },
            WebSocketTextResponseCommand::Unknown(payload) => WebSocketCommandWire {
                command: payload.command.clone(),
                payload: payload.payload.clone(),
            },
        };

        wire.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WebSocketTextResponseCommand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = WebSocketCommandWire::deserialize(deserializer)?;
        match wire.command.as_str() {
            "JoinRoom" => {
                let payload = wire
                    .payload
                    .ok_or_else(|| de::Error::custom("Missing payload for JoinRoom response"))?;
                serde_json::from_value(payload)
                    .map(WebSocketTextResponseCommand::JoinRoom)
                    .map_err(de::Error::custom)
            }
            "LeaveRoom" => {
                let payload = wire
                    .payload
                    .ok_or_else(|| de::Error::custom("Missing payload for LeaveRoom response"))?;
                serde_json::from_value(payload)
                    .map(WebSocketTextResponseCommand::LeaveRoom)
                    .map_err(de::Error::custom)
            }
            "GroupInfoSummary" => {
                let payload = wire.payload.ok_or_else(|| {
                    de::Error::custom("Missing payload for GroupInfoSummary response")
                })?;
                serde_json::from_value(payload)
                    .map(WebSocketTextResponseCommand::GroupInfoSummary)
                    .map_err(de::Error::custom)
            }
            "Error" => {
                let payload = wire
                    .payload
                    .ok_or_else(|| de::Error::custom("Missing payload for Error response"))?;
                serde_json::from_value(payload)
                    .map(WebSocketTextResponseCommand::Error)
                    .map_err(de::Error::custom)
            }
            _ => Ok(WebSocketTextResponseCommand::Unknown(
                UnknownWebSocketCommand {
                    command: wire.command,
                    payload: wire.payload,
                },
            )),
        }
    }
}
