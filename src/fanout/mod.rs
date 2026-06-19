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
    SendProposalAndCommit(Box<MlsProposalAndCommitInfo>),
    Proposal(MlsProposalInfo),
    CommitUpdate(MlsCommitInfo),
    RemoveLeafNode(MlsRemoveLeafNodeInfo),
    LiveKitAdminChange(LiveKitAdminChangeInfo),
    Welcome(MlsWelcomeInfo),
    JoinRequest(JoinRequestInfo),
    JoinDecision(JoinDecisionInfo),
    JoinRequestRemoved(JoinRequestRemovedInfo),
    /// Host-sealed PSK + Welcome for an STT-style agent join.
    /// Delivered only to the agent connection.
    AgentBundle(MlsAgentBundleInfo),
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

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct JoinRequestInfo {
    pub request_id: Vec<u8>,
    pub participant_uid: Vec<u8>,
    pub encrypted_key_package: Vec<u8>,
    pub expires_at: u64,
}

/// Outcome of a join request. Correlates with `JoinRequestInfo::request_id`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct JoinDecisionInfo {
    pub request_id: Vec<u8>,
    pub status: JoinDecisionStatus,
}

/// Status of a join request decision.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
#[repr(u8)]
pub enum JoinDecisionStatus {
    Admitted = 0,
    Rejected = 1,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct JoinRequestRemovedInfo {
    pub participant_uid: Vec<u8>,
}

/// Error type for join decision status conversion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JoinDecisionStatusError;

impl std::fmt::Display for JoinDecisionStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid join decision status")
    }
}

impl std::error::Error for JoinDecisionStatusError {}

impl TryFrom<u8> for JoinDecisionStatus {
    type Error = JoinDecisionStatusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(JoinDecisionStatus::Admitted),
            1 => Ok(JoinDecisionStatus::Rejected),
            _ => Err(JoinDecisionStatusError),
        }
    }
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
#[derive(Default)]
pub enum GroupInfoVersion {
    V0 = 0,
    #[default]
    V1 = 1,
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

/// Bundle delivered to an STT-style agent so it can join the MLS group.
///
/// The agent's join is PSK-bound (RFC 9420 §8.4): a meeting host derives
/// `psk = HKDF(meeting_password, "stt-agent", ...)`, HPKE-seals it to the
/// agent's KeyPackage HPKE public key, then commits `PreSharedKey { psk_id }`
/// + `Add { agent_kp }` in a single commit. The bundle carries both pieces
/// the agent needs:
///   - `encrypted_psk`: HPKE ciphertext only the agent's HPKE secret key can
///     open.
///   - `welcome`: standard MLS Welcome built from the commit that admitted
///     the agent.
///
/// `psk_id` is fixed across all agent joins (`"agent-psk-id"`) — kept as a
/// field rather than a constant so a future per-agent rotation strategy
/// can vary it without breaking the wire format.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    tls_codec::TlsSize,
    tls_codec::TlsDeserialize,
    tls_codec::TlsSerialize,
)]
pub struct MlsAgentBundleInfo {
    pub room_id: Vec<u8>,
    pub psk_id: Vec<u8>,
    pub encrypted_psk: Vec<u8>,
    pub welcome: Welcome,
    /// Tree state at the joining epoch. Embedded here (matching the human
    /// `MlsWelcomeInfo` shape) so the agent can `join_group(welcome, tree)`
    /// directly without an extra `GET /v1/groupInfo` round-trip.
    pub ratchet_tree: RatchetTreeOption,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tls_codec::{Deserialize as _, Serialize as _};

    fn sample_join_request_info() -> JoinRequestInfo {
        JoinRequestInfo {
            request_id: b"req-123".to_vec(),
            participant_uid: b"user-456".to_vec(),
            encrypted_key_package: vec![1, 2, 3, 4],
            expires_at: 1_700_000_000,
        }
    }

    fn sample_join_decision_info(status: JoinDecisionStatus) -> JoinDecisionInfo {
        JoinDecisionInfo {
            request_id: b"req-123".to_vec(),
            status,
        }
    }

    #[test]
    fn test_group_info_version_default() {
        assert_eq!(GroupInfoVersion::default(), GroupInfoVersion::V1);
    }

    #[test]
    fn test_join_request_info_tls_roundtrip() {
        let info = sample_join_request_info();

        let mut bytes = Vec::new();
        info.tls_serialize(&mut bytes).unwrap();

        let decoded = JoinRequestInfo::tls_deserialize(&mut bytes.as_slice()).unwrap();
        assert_eq!(info, decoded);
    }

    #[test]
    fn test_rtc_message_in_join_request_tls_roundtrip() {
        let msg = RTCMessageIn {
            content: RTCMessageInContent::JoinRequest(sample_join_request_info()),
        };

        let mut bytes = Vec::new();
        msg.tls_serialize(&mut bytes).unwrap();

        let decoded = RTCMessageIn::tls_deserialize(&mut bytes.as_slice()).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_rtc_message_in_content_join_request_discriminant() {
        let content = RTCMessageInContent::JoinRequest(JoinRequestInfo {
            request_id: vec![],
            participant_uid: vec![],
            encrypted_key_package: vec![],
            expires_at: 0,
        });

        let bytes = content.tls_serialize_detached().unwrap();
        assert_eq!(bytes[0], 7);
    }

    #[test]
    fn test_join_decision_status_try_from() {
        assert_eq!(
            JoinDecisionStatus::try_from(0),
            Ok(JoinDecisionStatus::Admitted)
        );
        assert_eq!(
            JoinDecisionStatus::try_from(1),
            Ok(JoinDecisionStatus::Rejected)
        );
        assert!(JoinDecisionStatus::try_from(99).is_err());
    }

    #[test]
    fn test_join_decision_info_tls_roundtrip() {
        for status in [JoinDecisionStatus::Admitted, JoinDecisionStatus::Rejected] {
            let info = sample_join_decision_info(status);

            let mut bytes = Vec::new();
            info.tls_serialize(&mut bytes).unwrap();

            let decoded = JoinDecisionInfo::tls_deserialize(&mut bytes.as_slice()).unwrap();
            assert_eq!(info, decoded);
        }
    }

    #[test]
    fn test_rtc_message_in_join_decision_tls_roundtrip() {
        let msg = RTCMessageIn {
            content: RTCMessageInContent::JoinDecision(sample_join_decision_info(
                JoinDecisionStatus::Admitted,
            )),
        };

        let mut bytes = Vec::new();
        msg.tls_serialize(&mut bytes).unwrap();

        let decoded = RTCMessageIn::tls_deserialize(&mut bytes.as_slice()).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_rtc_message_in_content_join_decision_discriminant() {
        let content = RTCMessageInContent::JoinDecision(JoinDecisionInfo {
            request_id: vec![],
            status: JoinDecisionStatus::Rejected,
        });

        let bytes = content.tls_serialize_detached().unwrap();
        assert_eq!(bytes[0], 8);
    }

    fn sample_welcome() -> Welcome {
        // Minimal Welcome — empty secrets, empty encrypted_group_info, default
        // ciphersuite. The agent-bundle round-trip test only exercises the
        // tls_codec wrapper, not Welcome semantics.
        Welcome {
            cipher_suite: Default::default(),
            secrets: Vec::new(),
            encrypted_group_info: Default::default(),
        }
    }

    fn sample_agent_bundle_info() -> MlsAgentBundleInfo {
        MlsAgentBundleInfo {
            room_id: b"room-789".to_vec(),
            psk_id: b"agent-psk-id".to_vec(),
            encrypted_psk: vec![9, 8, 7, 6, 5],
            welcome: sample_welcome(),
            ratchet_tree: RatchetTreeOption::OutOfBand,
        }
    }

    #[test]
    fn test_agent_bundle_info_tls_roundtrip() {
        let info = sample_agent_bundle_info();
        let mut bytes = Vec::new();
        info.tls_serialize(&mut bytes).unwrap();
        let decoded = MlsAgentBundleInfo::tls_deserialize(&mut bytes.as_slice()).unwrap();
        assert_eq!(info, decoded);
    }

    #[test]
    fn test_rtc_message_in_agent_bundle_tls_roundtrip() {
        let msg = RTCMessageIn {
            content: RTCMessageInContent::AgentBundle(sample_agent_bundle_info()),
        };
        let mut bytes = Vec::new();
        msg.tls_serialize(&mut bytes).unwrap();
        let decoded = RTCMessageIn::tls_deserialize(&mut bytes.as_slice()).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_rtc_message_in_content_agent_bundle_discriminant() {
        let content = RTCMessageInContent::AgentBundle(sample_agent_bundle_info());
        let bytes = content.tls_serialize_detached().unwrap();
        // AgentBundle is the 11th variant (0-indexed: 10) — after JoinRequestRemoved(9).
        assert_eq!(bytes[0], 10);
    }

    #[test]
    fn join_request_removed_round_trip() {
        let info = JoinRequestRemovedInfo {
            participant_uid: b"user-456".to_vec(),
        };
        let content = RTCMessageInContent::JoinRequestRemoved(info.clone());
        let mut buf = Vec::new();
        content.tls_serialize(&mut buf).unwrap();
        let decoded = RTCMessageInContent::tls_deserialize(&mut &buf[..]).unwrap();
        assert_eq!(decoded, content);
    }
}
