//! Shared types for meet-server and meet-client
//!
//! This crate contains data types that are shared between the meet server and client,
//! including MLS message types and WebSocket message types.

pub mod error;
pub mod fanout;
pub mod websocket;
use serde::{Deserialize, Serialize};
pub use websocket::*;

/// Service metric type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[repr(u8)]
pub enum MetricType {
    UserJoinTime = 0,
    UserRetryCount = 1,
    ErrorCode = 2,
    ConnectionLost = 3,
    UserEpochHealth = 4,
    DesignatedCommitter = 5,
    UserRejoin = 6,
}

/// Error type for metric conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricTypeError;

impl std::fmt::Display for MetricTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid metric type")
    }
}

impl std::error::Error for MetricTypeError {}

impl TryFrom<u8> for MetricType {
    type Error = MetricTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MetricType::UserJoinTime),
            1 => Ok(MetricType::UserRetryCount),
            2 => Ok(MetricType::ErrorCode),
            3 => Ok(MetricType::ConnectionLost),
            4 => Ok(MetricType::UserEpochHealth),
            5 => Ok(MetricType::DesignatedCommitter),
            6 => Ok(MetricType::UserRejoin),
            _ => Err(MetricTypeError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum JoinType {
    ExternalProposal,
    ExternalCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RejoinReason {
    EpochMismatch,
    WebsocketDisconnected,
    MemberNotFoundInMLS,
    FetchTimeout,
    LivekitStateMismatch,
    LivekitConnectionTimeout,
    Other,
}

/// User join time metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserJoinTimeMetric {
    pub join_type: Option<JoinType>,
    /// Time taken to join in milliseconds
    pub room_join_time_ms: u64, // time from client initialize to entering the room (need client to mark as in room)
    pub mls_join_time_ms: u64, // time from client initialize to joining the mls group
    /// Whether VP9 decode is supported
    pub is_vp9_decode_supported: Option<bool>,
    /// Whether VP9 encode is supported
    pub is_vp9_encode_supported: Option<bool>,
}

/// User retry count metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserRetryCountMetric {
    /// Number of retry attempts
    pub retry_count: u32,
}

/// Error code metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorCodeMetric {
    /// Error code encountered
    pub error_code: String,
    /// Optional error message
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConnectionLostType {
    EpochMismatch,
    WebsocketDisconnected,
    MemberNotFoundInMLS,
    FetchTimeout,
    Other,
}

/// Connection lost metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConnectionLostMetric {
    pub connection_lost_type: Option<ConnectionLostType>,
    /// Local epoch value
    pub local_epoch: u32,
    /// Server epoch value
    pub server_epoch: u32,
    /// Whether user device is in group info
    pub is_user_device_in_group_info: bool,
    /// Whether websocket is disconnected
    pub is_websocket_disconnected: bool,
    /// Whether websocket has reconnected
    pub has_websocket_reconnected: bool,
    /// Round trip time in milliseconds
    pub rtt: u32,
    /// Whether getting group info was successful
    pub is_get_group_info_success: bool,
}

/// User epoch health metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserEpochHealthMetric {
    /// Local epoch value
    pub local_epoch: u32,
    /// Epoch authenticator (optional)
    pub epoch_authenticator: Option<String>,
    /// Round trip time in milliseconds
    pub rtt: u32,
    /// WebSocket round trip time in milliseconds (optional)
    pub websocket_rtt: Option<u32>,
}

/// Designated committer metric data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DesignatedCommitterMetric {
    /// Epoch value for the commit
    pub epoch: u32,
    /// Designated committer rank
    pub designated_committer_rank: u64,
    /// New added member count
    pub new_member_count: Option<u32>,
    /// Removed member count
    pub removed_member_count: Option<u32>,
}

/// User rejoin metric data (room_id and user_id are in the request level)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserRejoinMetric {
    /// Reason for rejoin
    pub reason: RejoinReason,
    /// Incremental count indicating how many times the user has rejoined the same room
    pub incremental_count: u32,
    /// Time taken to rejoin in milliseconds
    pub rejoin_time_ms: u64,
    /// Whether the rejoin was successful
    pub success: bool,
}

/// Service metric data - supports multiple metric types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum ServiceMetric {
    UserJoinTime(UserJoinTimeMetric),
    UserRetryCount(UserRetryCountMetric),
    ErrorCode(ErrorCodeMetric),
    ConnectionLost(ConnectionLostMetric),
    UserEpochHealth(UserEpochHealthMetric),
    DesignatedCommitter(DesignatedCommitterMetric),
    UserRejoin(UserRejoinMetric),
}

impl ServiceMetric {
    /// Get the metric type
    pub fn metric_type(&self) -> MetricType {
        match self {
            ServiceMetric::UserJoinTime(_) => MetricType::UserJoinTime,
            ServiceMetric::UserRetryCount(_) => MetricType::UserRetryCount,
            ServiceMetric::ErrorCode(_) => MetricType::ErrorCode,
            ServiceMetric::ConnectionLost(_) => MetricType::ConnectionLost,
            ServiceMetric::UserEpochHealth(_) => MetricType::UserEpochHealth,
            ServiceMetric::DesignatedCommitter(_) => MetricType::DesignatedCommitter,
            ServiceMetric::UserRejoin(_) => MetricType::UserRejoin,
        }
    }

    /// Try to convert to UserJoinTimeMetric
    pub fn try_into_user_join_time(self) -> Result<UserJoinTimeMetric, Self> {
        match self {
            ServiceMetric::UserJoinTime(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to UserRetryCountMetric
    pub fn try_into_user_retry_count(self) -> Result<UserRetryCountMetric, Self> {
        match self {
            ServiceMetric::UserRetryCount(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to ErrorCodeMetric
    pub fn try_into_error_code(self) -> Result<ErrorCodeMetric, Self> {
        match self {
            ServiceMetric::ErrorCode(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to ConnectionLostMetric
    pub fn try_into_connection_lost(self) -> Result<ConnectionLostMetric, Self> {
        match self {
            ServiceMetric::ConnectionLost(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to UserEpochHealthMetric
    pub fn try_into_user_epoch_health(self) -> Result<UserEpochHealthMetric, Self> {
        match self {
            ServiceMetric::UserEpochHealth(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to DesignatedCommitterMetric
    pub fn try_into_designated_committer(self) -> Result<DesignatedCommitterMetric, Self> {
        match self {
            ServiceMetric::DesignatedCommitter(metric) => Ok(metric),
            other => Err(other),
        }
    }

    /// Try to convert to UserRejoinMetric
    pub fn try_into_user_rejoin(self) -> Result<UserRejoinMetric, Self> {
        match self {
            ServiceMetric::UserRejoin(metric) => Ok(metric),
            other => Err(other),
        }
    }
}

impl TryFrom<(MetricType, serde_json::Value)> for ServiceMetric {
    type Error = serde_json::Error;

    fn try_from(
        (metric_type, value): (MetricType, serde_json::Value),
    ) -> Result<Self, Self::Error> {
        match metric_type {
            MetricType::UserJoinTime => {
                Ok(ServiceMetric::UserJoinTime(serde_json::from_value(value)?))
            }
            MetricType::UserRetryCount => Ok(ServiceMetric::UserRetryCount(
                serde_json::from_value(value)?,
            )),
            MetricType::ErrorCode => Ok(ServiceMetric::ErrorCode(serde_json::from_value(value)?)),
            MetricType::ConnectionLost => Ok(ServiceMetric::ConnectionLost(
                serde_json::from_value(value)?,
            )),
            MetricType::UserEpochHealth => Ok(ServiceMetric::UserEpochHealth(
                serde_json::from_value(value)?,
            )),
            MetricType::DesignatedCommitter => Ok(ServiceMetric::DesignatedCommitter(
                serde_json::from_value(value)?,
            )),
            MetricType::UserRejoin => Ok(ServiceMetric::UserRejoin(serde_json::from_value(value)?)),
        }
    }
}

/// Request for submitting service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricsRequest {
    /// List of metrics to submit
    pub metrics: Vec<ServiceMetric>,
}

/// Unified service metrics request - allows submitting multiple metric types in a single request
/// All metric fields are optional, so you can include only the metrics you need
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceMetricsRequest {
    /// User join time metric (optional)
    pub user_join_time: Option<UserJoinTimeMetric>,
    /// User retry count metric (optional)
    pub user_retry_count: Option<UserRetryCountMetric>,
    /// Error code metric (optional)
    pub error_code: Option<ErrorCodeMetric>,
    /// Connection lost metric (optional)
    pub connection_lost: Option<ConnectionLostMetric>,
    /// User epoch health metric (optional)
    pub user_epoch_health: Option<UserEpochHealthMetric>,
    /// Designated committer metric (optional)
    pub designated_committer: Option<DesignatedCommitterMetric>,
    /// User rejoin metric (optional)
    pub user_rejoin: Option<UserRejoinMetric>,
}

impl ServiceMetricsRequest {
    pub fn new() -> Self {
        Self {
            user_join_time: None,
            user_retry_count: None,
            error_code: None,
            connection_lost: None,
            user_epoch_health: None,
            designated_committer: None,
            user_rejoin: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_room_message_serde() {
        let msg = JoinRoomMessage {
            room_id: "test-room".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: JoinRoomMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg.room_id, deserialized.room_id);
    }

    #[test]
    fn test_join_room_response_serde() {
        let resp = JoinRoomResponse {
            success: true,
            error: None,
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: JoinRoomResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.success, deserialized.success);
        assert_eq!(resp.error, deserialized.error);
    }

    #[test]
    fn test_leave_room_message_serde() {
        let msg = LeaveRoomMessage {
            room_id: "test-room".to_string(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        let deserialized: LeaveRoomMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(msg.room_id, deserialized.room_id);
    }

    #[test]
    fn test_leave_room_response_serde() {
        let resp = LeaveRoomResponse {
            success: false,
            error: Some("Room not found".to_string()),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: LeaveRoomResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp.success, deserialized.success);
        assert_eq!(resp.error, deserialized.error);
    }

    #[test]
    fn test_websocket_text_request_join_room_serde() {
        let request = WebSocketTextRequest {
            request_id: Some("req-1".to_string()),
            command: WebSocketTextRequestCommand::JoinRoom(JoinRoomMessage {
                room_id: "test-room".to_string(),
            }),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: WebSocketTextRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.request_id, Some("req-1".to_string()));
        match deserialized.command {
            WebSocketTextRequestCommand::JoinRoom(payload) => {
                assert_eq!(payload.room_id, "test-room");
            }
            _ => panic!("Expected JoinRoom command"),
        }
    }

    #[test]
    fn test_websocket_text_request_group_info_summary_serde() {
        let request = WebSocketTextRequest {
            request_id: None,
            command: WebSocketTextRequestCommand::GroupInfoSummary,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: WebSocketTextRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.request_id, None);
        match deserialized.command {
            WebSocketTextRequestCommand::GroupInfoSummary => {}
            _ => panic!("Expected GroupInfoSummary command"),
        }
    }

    #[test]
    fn test_websocket_text_request_unknown_command_deserialize() {
        let raw = serde_json::json!({
            "Command": "FutureCommand",
            "Payload": {
                "Flag": true,
                "Count": 2
            }
        });

        let deserialized: WebSocketTextRequest = serde_json::from_value(raw).unwrap();
        assert_eq!(deserialized.request_id, None);
        match deserialized.command {
            WebSocketTextRequestCommand::Unknown(payload) => {
                assert_eq!(payload.command, "FutureCommand");
                assert_eq!(
                    payload.payload,
                    Some(serde_json::json!({
                        "Flag": true,
                        "Count": 2
                    }))
                );
            }
            _ => panic!("Expected Unknown command"),
        }
    }

    #[test]
    fn test_websocket_text_request_unknown_command_roundtrip() {
        let raw = serde_json::json!({
            "RequestId": "req-unknown-1",
            "Command": "FutureCommand",
            "Payload": {
                "Flag": true,
                "Count": 2
            }
        });

        let deserialized: WebSocketTextRequest = serde_json::from_value(raw).unwrap();
        let serialized = serde_json::to_value(&deserialized).unwrap();
        let reparsed: WebSocketTextRequest = serde_json::from_value(serialized).unwrap();

        assert_eq!(reparsed.request_id, Some("req-unknown-1".to_string()));
        match reparsed.command {
            WebSocketTextRequestCommand::Unknown(payload) => {
                assert_eq!(payload.command, "FutureCommand");
                assert_eq!(
                    payload.payload,
                    Some(serde_json::json!({
                        "Flag": true,
                        "Count": 2
                    }))
                );
            }
            _ => panic!("Expected Unknown command"),
        }
    }

    #[test]
    fn test_websocket_text_request_join_room_missing_payload_fails() {
        let raw = serde_json::json!({
            "Command": "JoinRoom"
        });
        let result: Result<WebSocketTextRequest, _> = serde_json::from_value(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_websocket_text_request_join_room_invalid_payload_fails() {
        let raw = serde_json::json!({
            "Command": "JoinRoom",
            "Payload": {}
        });
        let result: Result<WebSocketTextRequest, _> = serde_json::from_value(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_websocket_text_response_join_room_serde() {
        let response = WebSocketTextResponse {
            request_id: Some("req-1".to_string()),
            command: WebSocketTextResponseCommand::JoinRoom(JoinRoomResponse {
                success: true,
                error: None,
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: WebSocketTextResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.request_id, Some("req-1".to_string()));
        match deserialized.command {
            WebSocketTextResponseCommand::JoinRoom(payload) => {
                assert!(payload.success);
                assert!(payload.error.is_none());
            }
            _ => panic!("Expected JoinRoom response"),
        }
    }

    #[test]
    fn test_websocket_text_response_group_info_summary_serde() {
        let response = WebSocketTextResponse {
            request_id: Some("req-3".to_string()),
            command: WebSocketTextResponseCommand::GroupInfoSummary(GroupInfoSummaryResponse {
                meeting_id: "meeting-1".to_string(),
                epoch: Some(42),
                group_id: Some(vec![1, 2, 3]),
                version: Some(1),
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: WebSocketTextResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.request_id, Some("req-3".to_string()));
        match deserialized.command {
            WebSocketTextResponseCommand::GroupInfoSummary(payload) => {
                assert_eq!(payload.meeting_id, "meeting-1");
                assert_eq!(payload.epoch, Some(42));
                assert_eq!(payload.group_id, Some(vec![1, 2, 3]));
                assert_eq!(payload.version, Some(1));
            }
            _ => panic!("Expected GroupInfoSummary response"),
        }
    }

    #[test]
    fn test_websocket_text_response_error_serde() {
        let response = WebSocketTextResponse {
            request_id: Some("req-2".to_string()),
            command: WebSocketTextResponseCommand::Error(WebSocketTextErrorResponse {
                command: Some(WebSocketCommandKind::LeaveRoom),
                code: "RoomNotFound".to_string(),
                message: "Room not found".to_string(),
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        let deserialized: WebSocketTextResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.request_id, Some("req-2".to_string()));
        match deserialized.command {
            WebSocketTextResponseCommand::Error(payload) => {
                match payload.command {
                    Some(WebSocketCommandKind::LeaveRoom) => {}
                    _ => panic!("Expected failed command to be LeaveRoom"),
                }
                assert_eq!(payload.code, "RoomNotFound");
                assert_eq!(payload.message, "Room not found");
            }
            _ => panic!("Expected Error response"),
        }
    }

    #[test]
    fn test_websocket_text_response_unknown_command_deserialize() {
        let raw = serde_json::json!({
            "RequestId": "req-4",
            "Command": "FutureResponse",
            "Payload": {
                "Ack": false
            }
        });

        let deserialized: WebSocketTextResponse = serde_json::from_value(raw).unwrap();
        assert_eq!(deserialized.request_id, Some("req-4".to_string()));
        match deserialized.command {
            WebSocketTextResponseCommand::Unknown(payload) => {
                assert_eq!(payload.command, "FutureResponse");
                assert_eq!(payload.payload, Some(serde_json::json!({ "Ack": false })));
            }
            _ => panic!("Expected Unknown response command"),
        }
    }

    #[test]
    fn test_websocket_text_response_unknown_command_roundtrip() {
        let raw = serde_json::json!({
            "RequestId": "req-unknown-2",
            "Command": "FutureResponse",
            "Payload": {
                "Ack": false
            }
        });

        let deserialized: WebSocketTextResponse = serde_json::from_value(raw).unwrap();
        let serialized = serde_json::to_value(&deserialized).unwrap();
        let reparsed: WebSocketTextResponse = serde_json::from_value(serialized).unwrap();

        assert_eq!(reparsed.request_id, Some("req-unknown-2".to_string()));
        match reparsed.command {
            WebSocketTextResponseCommand::Unknown(payload) => {
                assert_eq!(payload.command, "FutureResponse");
                assert_eq!(payload.payload, Some(serde_json::json!({ "Ack": false })));
            }
            _ => panic!("Expected Unknown response command"),
        }
    }

    #[test]
    fn test_websocket_text_response_join_room_missing_payload_fails() {
        let raw = serde_json::json!({
            "RequestId": "req-join-room-err",
            "Command": "JoinRoom"
        });
        let result: Result<WebSocketTextResponse, _> = serde_json::from_value(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_websocket_text_response_group_info_summary_invalid_payload_fails() {
        let raw = serde_json::json!({
            "RequestId": "req-group-info-err",
            "Command": "GroupInfoSummary",
            "Payload": {}
        });
        let result: Result<WebSocketTextResponse, _> = serde_json::from_value(raw);
        assert!(result.is_err());
    }

    #[test]
    fn test_metric_type_try_from() {
        assert_eq!(MetricType::try_from(0), Ok(MetricType::UserJoinTime));
        assert_eq!(MetricType::try_from(1), Ok(MetricType::UserRetryCount));
        assert_eq!(MetricType::try_from(2), Ok(MetricType::ErrorCode));
        assert_eq!(MetricType::try_from(3), Ok(MetricType::ConnectionLost));
        assert_eq!(MetricType::try_from(4), Ok(MetricType::UserEpochHealth));
        assert_eq!(MetricType::try_from(5), Ok(MetricType::DesignatedCommitter));
        assert_eq!(MetricType::try_from(6), Ok(MetricType::UserRejoin));
        assert!(MetricType::try_from(99).is_err());
    }

    #[test]
    fn test_service_metric_user_join_time() {
        let metric = ServiceMetric::UserJoinTime(UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalCommit),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: None,
            is_vp9_encode_supported: None,
        });

        assert_eq!(metric.metric_type(), MetricType::UserJoinTime);
        let join_time = metric.clone().try_into_user_join_time().unwrap();
        assert_eq!(join_time.join_type, Some(JoinType::ExternalCommit));
        assert_eq!(join_time.room_join_time_ms, 1500);
        assert_eq!(join_time.mls_join_time_ms, 1000);
    }

    #[test]
    fn test_service_metric_user_retry_count() {
        let metric = ServiceMetric::UserRetryCount(UserRetryCountMetric { retry_count: 3 });

        assert_eq!(metric.metric_type(), MetricType::UserRetryCount);
        let retry_count = metric.clone().try_into_user_retry_count().unwrap();
        assert_eq!(retry_count.retry_count, 3);
    }

    #[test]
    fn test_service_metric_error_code() {
        let metric = ServiceMetric::ErrorCode(ErrorCodeMetric {
            error_code: "ERR_CONNECTION_FAILED".to_string(),
            error_message: Some("Connection timeout".to_string()),
        });

        assert_eq!(metric.metric_type(), MetricType::ErrorCode);
        let error_code = metric.clone().try_into_error_code().unwrap();
        assert_eq!(error_code.error_code, "ERR_CONNECTION_FAILED");
        assert_eq!(
            error_code.error_message,
            Some("Connection timeout".to_string())
        );
    }

    #[test]
    fn test_service_metric_try_into_wrong_type() {
        let metric = ServiceMetric::UserJoinTime(UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalProposal),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: None,
            is_vp9_encode_supported: None,
        });

        // Try to convert to wrong type should return the original metric
        let result = metric.clone().try_into_error_code();
        assert!(result.is_err());
        let returned_metric = result.unwrap_err();
        assert_eq!(returned_metric.metric_type(), MetricType::UserJoinTime);
    }

    #[test]
    fn test_metrics_request_serde() {
        let request = MetricsRequest {
            metrics: vec![
                ServiceMetric::UserJoinTime(UserJoinTimeMetric {
                    join_type: Some(JoinType::ExternalCommit),
                    room_join_time_ms: 1500,
                    mls_join_time_ms: 1000,
                    is_vp9_decode_supported: None,
                    is_vp9_encode_supported: None,
                }),
                ServiceMetric::UserRetryCount(UserRetryCountMetric { retry_count: 2 }),
                ServiceMetric::ErrorCode(ErrorCodeMetric {
                    error_code: "ERR_TIMEOUT".to_string(),
                    error_message: None,
                }),
            ],
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: MetricsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.metrics.len(), 3);
        assert_eq!(
            deserialized.metrics[0].metric_type(),
            MetricType::UserJoinTime
        );
        assert_eq!(
            deserialized.metrics[1].metric_type(),
            MetricType::UserRetryCount
        );
        assert_eq!(deserialized.metrics[2].metric_type(), MetricType::ErrorCode);
    }

    #[test]
    fn test_service_metric_try_from_json() {
        let json_value = serde_json::json!({
            "JoinType": "ExternalProposal",
            "RoomJoinTimeMs": 2000,
            "MlsJoinTimeMs": 1000
        });

        let metric = ServiceMetric::try_from((MetricType::UserJoinTime, json_value)).unwrap();
        match metric {
            ServiceMetric::UserJoinTime(data) => {
                assert_eq!(data.join_type, Some(JoinType::ExternalProposal));
                assert_eq!(data.room_join_time_ms, 2000);
                assert_eq!(data.mls_join_time_ms, 1000);
            }
            _ => panic!("Wrong metric type"),
        }
    }

    #[test]
    fn test_service_metrics_request_single_request() {
        let request = ServiceMetricsRequest {
            user_join_time: Some(UserJoinTimeMetric {
                join_type: Some(JoinType::ExternalCommit),
                room_join_time_ms: 1500,
                mls_join_time_ms: 1000,
                is_vp9_decode_supported: None,
                is_vp9_encode_supported: None,
            }),
            user_retry_count: Some(UserRetryCountMetric { retry_count: 3 }),
            error_code: Some(ErrorCodeMetric {
                error_code: "ERR_TIMEOUT".to_string(),
                error_message: Some("Connection timeout".to_string()),
            }),
            connection_lost: None,
            user_epoch_health: None,
            designated_committer: None,
            user_rejoin: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ServiceMetricsRequest = serde_json::from_str(&json).unwrap();

        assert!(deserialized.user_join_time.is_some());
        assert!(deserialized.user_retry_count.is_some());
        assert!(deserialized.error_code.is_some());

        let join_time = deserialized.user_join_time.unwrap();
        assert_eq!(join_time.room_join_time_ms, 1500);
        assert_eq!(join_time.mls_join_time_ms, 1000);
        assert_eq!(deserialized.user_retry_count.unwrap().retry_count, 3);
        assert_eq!(deserialized.error_code.unwrap().error_code, "ERR_TIMEOUT");
    }

    #[test]
    fn test_service_metrics_request_partial() {
        let request = ServiceMetricsRequest {
            user_join_time: Some(UserJoinTimeMetric {
                join_type: Some(JoinType::ExternalProposal),
                room_join_time_ms: 2000,
                mls_join_time_ms: 1000,
                is_vp9_decode_supported: None,
                is_vp9_encode_supported: None,
            }),
            user_retry_count: None,
            error_code: None,
            connection_lost: None,
            user_epoch_health: None,
            designated_committer: None,
            user_rejoin: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ServiceMetricsRequest = serde_json::from_str(&json).unwrap();

        assert!(deserialized.user_join_time.is_some());
        assert!(deserialized.user_retry_count.is_none());
        assert!(deserialized.error_code.is_none());
    }

    #[test]
    fn test_service_metrics_request_new() {
        let request = ServiceMetricsRequest::new();
        assert!(request.user_join_time.is_none());
        assert!(request.user_retry_count.is_none());
        assert!(request.error_code.is_none());
        assert!(request.connection_lost.is_none());
        assert!(request.user_epoch_health.is_none());
        assert!(request.designated_committer.is_none());
        assert!(request.user_rejoin.is_none());
    }

    #[test]
    fn test_service_metric_connection_lost() {
        let metric = ServiceMetric::ConnectionLost(ConnectionLostMetric {
            connection_lost_type: Some(ConnectionLostType::EpochMismatch),
            local_epoch: 100,
            server_epoch: 101,
            is_user_device_in_group_info: true,
            is_websocket_disconnected: true,
            has_websocket_reconnected: false,
            rtt: 50,
            is_get_group_info_success: true,
        });

        assert_eq!(metric.metric_type(), MetricType::ConnectionLost);
        let connection_lost = metric.clone().try_into_connection_lost().unwrap();
        assert_eq!(
            connection_lost.connection_lost_type,
            Some(ConnectionLostType::EpochMismatch)
        );
        assert_eq!(connection_lost.local_epoch, 100);
        assert_eq!(connection_lost.server_epoch, 101);
        assert_eq!(connection_lost.is_user_device_in_group_info, true);
        assert_eq!(connection_lost.is_websocket_disconnected, true);
        assert_eq!(connection_lost.has_websocket_reconnected, false);
        assert_eq!(connection_lost.rtt, 50);
        assert_eq!(connection_lost.is_get_group_info_success, true);
    }

    #[test]
    fn test_service_metrics_request_with_connection_lost() {
        let request = ServiceMetricsRequest {
            user_join_time: None,
            user_retry_count: None,
            error_code: None,
            connection_lost: Some(ConnectionLostMetric {
                connection_lost_type: Some(ConnectionLostType::WebsocketDisconnected),
                local_epoch: 200,
                server_epoch: 201,
                is_user_device_in_group_info: false,
                is_websocket_disconnected: true,
                has_websocket_reconnected: true,
                rtt: 100,
                is_get_group_info_success: false,
            }),
            user_epoch_health: None,
            designated_committer: None,
            user_rejoin: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ServiceMetricsRequest = serde_json::from_str(&json).unwrap();

        assert!(deserialized.connection_lost.is_some());

        let connection_lost = deserialized.connection_lost.unwrap();
        assert_eq!(
            connection_lost.connection_lost_type,
            Some(ConnectionLostType::WebsocketDisconnected)
        );
        assert_eq!(connection_lost.local_epoch, 200);
        assert_eq!(connection_lost.server_epoch, 201);
        assert_eq!(connection_lost.is_user_device_in_group_info, false);
        assert_eq!(connection_lost.is_websocket_disconnected, true);
        assert_eq!(connection_lost.has_websocket_reconnected, true);
        assert_eq!(connection_lost.rtt, 100);
        assert_eq!(connection_lost.is_get_group_info_success, false);
    }

    #[test]
    fn test_service_metric_designated_committer() {
        let metric = ServiceMetric::DesignatedCommitter(DesignatedCommitterMetric {
            epoch: 100,
            designated_committer_rank: 5,
            new_member_count: Some(3),
            removed_member_count: Some(1),
        });

        assert_eq!(metric.metric_type(), MetricType::DesignatedCommitter);
        let designated_committer = metric.clone().try_into_designated_committer().unwrap();
        assert_eq!(designated_committer.epoch, 100);
        assert_eq!(designated_committer.designated_committer_rank, 5);
        assert_eq!(designated_committer.new_member_count, Some(3));
        assert_eq!(designated_committer.removed_member_count, Some(1));
    }

    #[test]
    fn test_service_metric_designated_committer_with_none() {
        let metric = ServiceMetric::DesignatedCommitter(DesignatedCommitterMetric {
            epoch: 200,
            designated_committer_rank: 10,
            new_member_count: None,
            removed_member_count: None,
        });

        assert_eq!(metric.metric_type(), MetricType::DesignatedCommitter);
        let designated_committer = metric.clone().try_into_designated_committer().unwrap();
        assert_eq!(designated_committer.epoch, 200);
        assert_eq!(designated_committer.designated_committer_rank, 10);
        assert_eq!(designated_committer.new_member_count, None);
        assert_eq!(designated_committer.removed_member_count, None);
    }

    #[test]
    fn test_service_metric_try_into_designated_committer_wrong_type() {
        let metric = ServiceMetric::UserJoinTime(UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalProposal),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: None,
            is_vp9_encode_supported: None,
        });

        // Try to convert to wrong type should return the original metric
        let result = metric.clone().try_into_designated_committer();
        assert!(result.is_err());
        let returned_metric = result.unwrap_err();
        assert_eq!(returned_metric.metric_type(), MetricType::UserJoinTime);
    }

    #[test]
    fn test_service_metric_try_from_json_designated_committer() {
        let json_value = serde_json::json!({
            "Epoch": 150,
            "DesignatedCommitterRank": 7,
            "NewMemberCount": 2,
            "RemovedMemberCount": 0
        });

        let metric =
            ServiceMetric::try_from((MetricType::DesignatedCommitter, json_value)).unwrap();
        match metric {
            ServiceMetric::DesignatedCommitter(data) => {
                assert_eq!(data.epoch, 150);
                assert_eq!(data.designated_committer_rank, 7);
                assert_eq!(data.new_member_count, Some(2));
                assert_eq!(data.removed_member_count, Some(0));
            }
            _ => panic!("Wrong metric type"),
        }
    }

    #[test]
    fn test_service_metrics_request_with_designated_committer() {
        let request = ServiceMetricsRequest {
            user_join_time: None,
            user_retry_count: None,
            error_code: None,
            connection_lost: None,
            user_epoch_health: None,
            designated_committer: Some(DesignatedCommitterMetric {
                epoch: 300,
                designated_committer_rank: 8,
                new_member_count: Some(5),
                removed_member_count: Some(2),
            }),
            user_rejoin: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: ServiceMetricsRequest = serde_json::from_str(&json).unwrap();

        assert!(deserialized.designated_committer.is_some());

        let designated_committer = deserialized.designated_committer.unwrap();
        assert_eq!(designated_committer.epoch, 300);
        assert_eq!(designated_committer.designated_committer_rank, 8);
        assert_eq!(designated_committer.new_member_count, Some(5));
        assert_eq!(designated_committer.removed_member_count, Some(2));
    }

    #[test]
    fn test_service_metrics_request_new_includes_designated_committer() {
        let request = ServiceMetricsRequest::new();
        assert!(request.user_join_time.is_none());
        assert!(request.user_retry_count.is_none());
        assert!(request.error_code.is_none());
        assert!(request.connection_lost.is_none());
        assert!(request.user_epoch_health.is_none());
        assert!(request.designated_committer.is_none());
        assert!(request.user_rejoin.is_none());
    }

    #[test]
    fn test_user_join_time_metric_with_vp9_support() {
        let metric = ServiceMetric::UserJoinTime(UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalCommit),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: Some(true),
            is_vp9_encode_supported: Some(false),
        });

        assert_eq!(metric.metric_type(), MetricType::UserJoinTime);
        let join_time = metric.clone().try_into_user_join_time().unwrap();
        assert_eq!(join_time.is_vp9_decode_supported, Some(true));
        assert_eq!(join_time.is_vp9_encode_supported, Some(false));
    }

    #[test]
    fn test_user_join_time_metric_vp9_support_serde() {
        let json_value = serde_json::json!({
            "JoinType": "ExternalProposal",
            "RoomJoinTimeMs": 2000,
            "MlsJoinTimeMs": 1000,
            "IsVp9DecodeSupported": true,
            "IsVp9EncodeSupported": true
        });

        let metric = ServiceMetric::try_from((MetricType::UserJoinTime, json_value)).unwrap();
        match metric {
            ServiceMetric::UserJoinTime(data) => {
                assert_eq!(data.join_type, Some(JoinType::ExternalProposal));
                assert_eq!(data.room_join_time_ms, 2000);
                assert_eq!(data.mls_join_time_ms, 1000);
                assert_eq!(data.is_vp9_decode_supported, Some(true));
                assert_eq!(data.is_vp9_encode_supported, Some(true));
            }
            _ => panic!("Wrong metric type"),
        }
    }

    #[test]
    fn test_user_join_time_metric_vp9_support_optional() {
        let metric = UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalCommit),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: None,
            is_vp9_encode_supported: None,
        };

        let json = serde_json::to_string(&metric).unwrap();
        let deserialized: UserJoinTimeMetric = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.is_vp9_decode_supported, None);
        assert_eq!(deserialized.is_vp9_encode_supported, None);
    }

    #[test]
    fn test_service_metric_user_rejoin() {
        let metric = ServiceMetric::UserRejoin(UserRejoinMetric {
            reason: RejoinReason::EpochMismatch,
            incremental_count: 2,
            rejoin_time_ms: 1500,
            success: true,
        });

        assert_eq!(metric.metric_type(), MetricType::UserRejoin);
        let rejoin = metric.clone().try_into_user_rejoin().unwrap();
        assert_eq!(rejoin.reason, RejoinReason::EpochMismatch);
        assert_eq!(rejoin.incremental_count, 2);
        assert_eq!(rejoin.rejoin_time_ms, 1500);
        assert_eq!(rejoin.success, true);
    }

    #[test]
    fn test_service_metric_user_rejoin_all_reasons() {
        let reasons = vec![
            RejoinReason::EpochMismatch,
            RejoinReason::WebsocketDisconnected,
            RejoinReason::MemberNotFoundInMLS,
            RejoinReason::FetchTimeout,
            RejoinReason::LivekitStateMismatch,
            RejoinReason::LivekitConnectionTimeout,
            RejoinReason::Other,
        ];

        for (i, reason) in reasons.iter().enumerate() {
            let metric = ServiceMetric::UserRejoin(UserRejoinMetric {
                reason: *reason,
                incremental_count: i as u32 + 1,
                rejoin_time_ms: 1000 + (i as u64 * 100),
                success: i % 2 == 0,
            });

            assert_eq!(metric.metric_type(), MetricType::UserRejoin);
            let rejoin = metric.clone().try_into_user_rejoin().unwrap();
            assert_eq!(rejoin.reason, *reason);
            assert_eq!(rejoin.incremental_count, i as u32 + 1);
            assert_eq!(rejoin.rejoin_time_ms, 1000 + (i as u64 * 100));
            assert_eq!(rejoin.success, i % 2 == 0);
        }
    }

    #[test]
    fn test_service_metric_try_into_user_rejoin_wrong_type() {
        let metric = ServiceMetric::UserJoinTime(UserJoinTimeMetric {
            join_type: Some(JoinType::ExternalProposal),
            room_join_time_ms: 1500,
            mls_join_time_ms: 1000,
            is_vp9_decode_supported: None,
            is_vp9_encode_supported: None,
        });

        // Try to convert to wrong type should return the original metric
        let result = metric.clone().try_into_user_rejoin();
        assert!(result.is_err());
        let returned_metric = result.unwrap_err();
        assert_eq!(returned_metric.metric_type(), MetricType::UserJoinTime);
    }

    #[test]
    fn test_metrics_request_with_user_rejoin() {
        let request = MetricsRequest {
            metrics: vec![
                ServiceMetric::UserRejoin(UserRejoinMetric {
                    reason: RejoinReason::EpochMismatch,
                    incremental_count: 2,
                    rejoin_time_ms: 1500,
                    success: true,
                }),
                ServiceMetric::UserJoinTime(UserJoinTimeMetric {
                    join_type: Some(JoinType::ExternalCommit),
                    room_join_time_ms: 1500,
                    mls_join_time_ms: 1000,
                    is_vp9_decode_supported: None,
                    is_vp9_encode_supported: None,
                }),
            ],
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: MetricsRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.metrics.len(), 2);
        assert_eq!(
            deserialized.metrics[0].metric_type(),
            MetricType::UserRejoin
        );
        assert_eq!(
            deserialized.metrics[1].metric_type(),
            MetricType::UserJoinTime
        );
    }
}
