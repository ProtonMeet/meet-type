use mls_spec::{
    drafts::ratchet_tree_options::RatchetTreeOption, group::welcome::Welcome, messages::MlsMessage,
};
use serde::{Deserialize, Serialize};

/// Content types for real-time communication messages
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
#[repr(u8)]
pub enum RTCMessageInContent {
    SendCommit(MlsCommitInfo),
    SendProposalAndCommit(MlsProposalAndCommitInfo),
    Proposal(MlsProposalInfo),
    CommitUpdate(MlsCommitInfo),
    RemoveLeafNode(MlsRemoveLeafNodeInfo),
    LiveKitAdminChange(LiveKitAdminChangeInfo),
    Welcome(MlsWelcomeInfo),
}

/// Ratchet tree and group info bundle for MLS external commits
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct RatchetTreeAndGroupInfo {
    pub ratchet_tree: RatchetTreeOption,
    pub group_info: MlsMessage,
}

/// MLS commit information with optional welcome message
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsCommitInfo {
    pub room_id: Vec<u8>,
    pub welcome_message: Option<Welcome>,
    pub commit: MlsMessage,
}

/// MLS proposal and commit bundle
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsProposalAndCommitInfo {
    pub room_id: Vec<u8>,
    pub proposal: MlsMessage,
    pub commit: MlsMessage,
}

/// MLS welcome and ratchet tree bundle
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsWelcomeInfo {
    pub room_id: Vec<u8>,
    pub welcome: Welcome,
    pub ratchet_tree: RatchetTreeOption,
}

/// MLS proposal information
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsProposalInfo {
    pub room_id: Vec<u8>,
    pub proposal: MlsMessage,
}

/// Version enum for GroupInfo data format
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
    Serialize,
    Deserialize,
)]
#[repr(u32)]
pub enum GroupInfoVersion {
    V0 = 0,
    V1 = 1,
}

impl Default for GroupInfoVersion {
    fn default() -> Self {
        GroupInfoVersion::V1 // New data defaults to V1
    }
}

/// Error type for GroupInfo version conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupInfoVersionError;

impl std::fmt::Display for GroupInfoVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid group info version")
    }
}

impl std::error::Error for GroupInfoVersionError {}

impl TryFrom<u32> for GroupInfoVersion {
    type Error = GroupInfoVersionError;

    fn try_from(version: u32) -> Result<Self, Self::Error> {
        match version {
            0 => Ok(GroupInfoVersion::V0),
            1 => Ok(GroupInfoVersion::V1),
            _ => Err(GroupInfoVersionError),
        }
    }
}

/// Versioned wrapper for GroupInfo data stored in NATS
/// This allows backward compatibility with existing unversioned data
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct VersionedGroupInfoData {
    pub version: GroupInfoVersion,
    pub data: RatchetTreeAndGroupInfo,
}

/// Group info summary data, client can compare it with the local MLS group to see if it's up to date
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct GroupInfoSummaryData {
    pub epoch: u64,
    pub group_id: Vec<u8>,
}

/// Real-time communication message sent from client to server
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct RTCMessageIn {
    pub content: RTCMessageInContent,
}

/// Information for removing a leaf node from MLS tree
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsRemoveLeafNodeInfo {
    pub room_id: Vec<u8>,
    pub leaf_node_index: u32,
}

/// Information for changing LiveKit room admin
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct LiveKitAdminChangeInfo {
    pub room_id: Vec<u8>,
    pub participant_uid: Vec<u8>,
    pub participant_type: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_info_version_default() {
        assert_eq!(GroupInfoVersion::default(), GroupInfoVersion::V1);
    }
}
