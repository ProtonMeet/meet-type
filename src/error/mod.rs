use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ServerError {
    InvalidToken = 4000,
    CheckDeviceClock = 4001,
    TokenExpired = 4002,
}
