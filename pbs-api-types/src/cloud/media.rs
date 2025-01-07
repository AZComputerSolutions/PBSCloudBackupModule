//! Types for cloud media management API
//!
//! This module defines types and schemas for managing cloud media.

use serde::{Deserialize, Serialize};

use proxmox_schema::*;
use proxmox_uuid::Uuid;

use crate::{MediaLocation, MediaStatus, UUID_FORMAT};

pub const CLOUD_MEDIA_SET_UUID_SCHEMA: Schema = StringSchema::new(
    "Cloud MediaSet UUID (The all-zero UUID reserves an empty media for a specific pool).",
)
.format(&UUID_FORMAT)
.schema();

pub const CLOUD_MEDIA_UUID_SCHEMA: Schema = StringSchema::new("Cloud Media UUID.")
    .format(&UUID_FORMAT)
    .schema();

#[api(
    properties: {
        "media-set-uuid": {
            schema: CLOUD_MEDIA_SET_UUID_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Media Set list entry
pub struct CloudMediaSetListEntry {
    /// Cloud media set name
    pub media_set_name: String,
    pub media_set_uuid: Uuid,
    /// Cloud MediaSet creation time stamp
    pub media_set_ctime: i64,
    /// Cloud Media Pool
    pub pool: String,
}

#[api(
    properties: {
        location: {
            type: MediaLocation,
        },
        status: {
            type: MediaStatus,
        },
        uuid: {
            schema: CLOUD_MEDIA_UUID_SCHEMA,
        },
        "media-set-uuid": {
            schema: CLOUD_MEDIA_SET_UUID_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Media list entry
pub struct CloudMediaListEntry {
    /// Cloud Media label text
    pub label_text: String,
    pub uuid: Uuid,
    /// Cloud Media creation time stamp
    pub ctime: i64,
    pub location: MediaLocation,
    pub status: MediaStatus,
    /// Expired flag
    pub expired: bool,
    /// Catalog status OK
    pub catalog: bool,
    /// Cloud Media set name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_set_uuid: Option<Uuid>,
    /// Cloud Media set seq_nr
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nr: Option<u64>,
    /// Cloud MediaSet creation time stamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_set_ctime: Option<i64>,
    /// Cloud Media Pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
}

#[api(
    properties: {
        uuid: {
            schema: CLOUD_MEDIA_UUID_SCHEMA,
        },
        "media-set-uuid": {
            schema: CLOUD_MEDIA_SET_UUID_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Media label info
pub struct CloudMediaIdFlat {
    /// Unique ID
    pub uuid: Uuid,
    /// Cloud Media label text
    pub label_text: String,
    /// Cloud Media creation time stamp
    pub ctime: i64,
    /// Cloud MediaSet Pool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_set_uuid: Option<Uuid>,
    /// Cloud MediaSet media sequence number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nr: Option<u64>,
    /// Cloud MediaSet Creation time stamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_set_ctime: Option<i64>,
    /// Encryption key fingerprint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key_fingerprint: Option<String>,
}

#[api(
    properties: {
        uuid: {
            schema: CLOUD_MEDIA_UUID_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Label with optional UUID
pub struct CloudLabelUuidMap {
    /// Cloud label text
    pub label_text: String,
    /// Associated UUID (if any)
    pub uuid: Option<Uuid>,
}

#[api(
    properties: {
        uuid: {
            schema: CLOUD_MEDIA_UUID_SCHEMA,
        },
        "media-set-uuid": {
            schema: CLOUD_MEDIA_SET_UUID_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
/// Cloud Media content list entry
pub struct CloudMediaContentEntry {
    /// Cloud Media label text
    pub label_text: String,
    /// Cloud Media UUID
    pub uuid: Uuid,
    /// Cloud Media set name
    pub media_set_name: String,
    /// Cloud Media set UUID
    pub media_set_uuid: Uuid,
    /// Cloud MediaSet creation time stamp
    pub media_set_ctime: i64,
    /// Cloud Media set seq_nr
    pub seq_nr: u64,
    /// Cloud Media Pool
    pub pool: String,
    /// Datastore Name
    pub store: String,
    /// Backup snapshot
    pub snapshot: String,
    /// Snapshot creation time (epoch)
    pub backup_time: i64,
}
