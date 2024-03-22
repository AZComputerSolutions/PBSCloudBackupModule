//! Types for cloud backup API

use serde::{Deserialize, Serialize};

use proxmox_schema::{api, const_regex, ApiStringFormat, Schema, StringSchema};
use proxmox_uuid::Uuid;

use crate::{BackupType, BACKUP_ID_SCHEMA, FINGERPRINT_SHA256_FORMAT};

const_regex! {
    pub CLOUD_RESTORE_SNAPSHOT_REGEX = concat!(r"^", PROXMOX_SAFE_ID_REGEX_STR!(), r":(?:", BACKUP_NS_PATH_RE!(),")?", SNAPSHOT_PATH_REGEX_STR!(), r"$");
}

pub const CLOUD_RESTORE_SNAPSHOT_FORMAT: ApiStringFormat =
    ApiStringFormat::Pattern(&CLOUD_RESTORE_SNAPSHOT_REGEX);

pub const CLOUD_ENCRYPTION_KEY_FINGERPRINT_SCHEMA: Schema =
    StringSchema::new("Cloud encryption key fingerprint (sha256).")
        .format(&FINGERPRINT_SHA256_FORMAT)
        .schema();

pub const CLOUD_RESTORE_SNAPSHOT_SCHEMA: Schema =
    StringSchema::new("A snapshot in the format: 'store:[ns/namespace/...]type/id/time")
        .format(&CLOUD_RESTORE_SNAPSHOT_FORMAT)
        .type_text("store:[ns/namespace/...]type/id/time")
        .schema();

pub struct CloudContentListFilter {
    pub label_text: Option<String>,
    pub backup_type: Option<BackupType>,
    pub backup_id: Option<String>,
}
