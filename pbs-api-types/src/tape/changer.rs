//! Types for cloud backup API

use serde::{Deserialize, Serialize};

use proxmox_schema::{
    api, ApiStringFormat, ArraySchema, IntegerSchema, Schema, StringSchema, Updater,
};

use crate::{OptionalDeviceIdentification, PROXMOX_SAFE_ID_FORMAT};

pub const BACKUP_NAME_SCHEMA: Schema = StringSchema::new("Cloud Backup Identifier.")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(32)
    .schema();

pub const CLOUD_STORAGE_PATH_SCHEMA: Schema =
    StringSchema::new("Path or URL to the cloud storage (e.g. 's3://bucket-name/path').").schema();

pub const OBJECT_LABEL_SCHEMA: Schema = StringSchema::new("Object Label/Identifier.")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(2)
    .max_length(32)
    .schema();

pub const BACKUP_SLOT_ARRAY_SCHEMA: Schema = ArraySchema::new(
    "Backup slot list.",
    &IntegerSchema::new("Slot number").minimum(1).schema(),
)
.schema();

pub const EXPORT_OBJECT_LIST_SCHEMA: Schema = StringSchema::new(
    "A list of backup slots, comma-separated. These slots are reserved for specific usage in the cloud.",
)
.format(&ApiStringFormat::PropertyString(&BACKUP_SLOT_ARRAY_SCHEMA))
.schema();

#[api(
    properties: {
        name: {
            schema: BACKUP_NAME_SCHEMA,
        },
        path: {
            schema: CLOUD_STORAGE_PATH_SCHEMA,
        },
        "export-slots": {
            schema: EXPORT_OBJECT_LIST_SCHEMA,
            optional: true,
        },
        "auto-eject": {
            optional: true,
            default: false,
        }
    },
)]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud backup configuration
pub struct CloudBackupConfig {
    #[updater(skip)]
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_slots: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set to true, data is ejected after backup completion
    pub auto_eject: Option<bool>,
}

#[api(
    properties: {
        config: {
            type: CloudBackupConfig,
        },
        info: {
            type: OptionalDeviceIdentification,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud backup configuration with optional identification attributes
pub struct CloudBackupListEntry {
    #[serde(flatten)]
    pub config: CloudBackupConfig,
    #[serde(flatten)]
    pub info: OptionalDeviceIdentification,
}

#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Object Kind
pub enum CloudObjectKind {
    /// Storage bucket
    Bucket,
    /// Individual file/object
    Object,
    /// Import/Export slots
    ImportExport,
}

#[api(
    properties: {
        "object-kind": {
            type: CloudObjectKind,
        },
        "label-text": {
            schema: OBJECT_LABEL_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Object Entry
pub struct CloudObjectEntry {
    pub object_kind: CloudObjectKind,
    /// The ID of the object or slot
    pub object_id: u64,
    /// The object label (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_text: Option<String>,
    /// The slot the object was retrieved from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loaded_slot: Option<u64>,
    /// The current state of the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
