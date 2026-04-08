# meet-type

Shared data types for meet-server and meet-client communication.

## Overview

This crate contains the shared type definitions that are used for communication between the meet server and client. By extracting these types into a separate crate, we ensure consistency and make it easy to share types across different components.

## Types Included

### MLS Message Types

- `MlsCommitInfo` - MLS commit with optional welcome message
- `MlsProposalInfo` - MLS proposal information
- `MlsProposalAndCommitInfo` - Bundle of proposal and commit
- `MlsRemoveLeafNodeInfo` - Information for removing a leaf node

### WebSocket Message Types

- `RTCMessageIn` - Real-time communication message from client
- `RTCMessageInContent` - Content types for RTC messages
- `JoinRoomMessage` / `JoinRoomResponse` - Room joining messages
- `LeaveRoomMessage` / `LeaveRoomResponse` - Room leaving messages
- `GroupInfoSummary` / `GroupInfoSummaryResponse` - Room groupInfo summary

### GroupInfo Types

- `VersionedGroupInfoData` - Versioned wrapper for group info
- `RatchetTreeAndGroupInfo` - Bundle of ratchet tree and group info
- `GroupInfoVersion` - Version enum for backward compatibility
- `GroupInfoSummaryData` - Summary of group info

### LiveKit Types

- `LiveKitAdminChangeInfo` - Admin change notification

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
meet-type = { path = "../meet-type" }
```

Then import the types you need:

```rust
use meet_type::{RTCMessageIn, MlsCommitInfo, JoinRoomMessage};
```

## Serialization

All types implement:

- `serde::Serialize` and `serde::Deserialize` for JSON
- `tls_codec` traits for TLS-style binary encoding (where applicable)

## Dependencies

- `serde` - JSON serialization
- `tls_codec` - TLS-style binary encoding
- `mls-spec` - MLS protocol types
