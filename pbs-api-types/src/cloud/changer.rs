use serde::{Deserialize, Serialize};

use proxmox_schema::{
    api, ApiStringFormat, ArraySchema, IntegerSchema, Schema, StringSchema, Updater,
};

use crate::{
    OptionalBackupSpecification, PROXMOX_SAFE_ID_FORMAT,
};

pub const BUCKET_NAME_SCHEMA: Schema = StringSchema::new("Bucket Name")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .schema();

pub const CLOUD_PROVIDER_SCHEMA: Schema = StringSchema::new("Cloud Provider")
    .format(&ApiStringFormat::PropertyString(
        "^(aws|azure|gcp)$",
    ))
    .schema();

pub const ACCESS_KEY_SCHEMA: Schema = StringSchema::new("Access Key ID")
    .format(&ApiStringFormat::PropertyString(
        "^[0-9a-zA-Z]{16,64}$",
    ))
    .schema();

pub const SECRET_KEY_SCHEMA: Schema = StringSchema::new("Secret Access Key")
    .format(&ApiStringFormat::PropertyString(
        "^.{16,512}$",
    ))
    .schema();

pub const REGION_SCHEMA: Schema = StringSchema::new("Region")
    .format(&ApiStringFormat::PropertyString(
        "^[a-zA-Z0-9-_]{2,64}$",
    ))
    .schema();

pub const CONTAINER_NAME_SCHEMA: Schema = StringSchema::new("Container Name")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .schema();

pub const EXPORT_DATA_SCHEMA: Schema = StringSchema::new(
    "A list of blob names, comma separated, to be exported from the container.",
)
.format(&ApiStringFormat::PropertyString(
    "^[a-zA-Z0-9-_./~`!@#$%^&*()-_=+{}\\[\\]:\"'|;'<>,?/]{1,512}$",
))
.schema();

/// Cloud Backup definition
#[api(
    properties: {
        config: {
            type: CloudBackupConfig,
        }
    }
)]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup
pub struct CloudBackup {
    #[updater(skip)]
    pub config: CloudBackupConfig,
}

/// Cloud Backup Configuration
#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup Config
pub struct CloudBackupConfig {
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub bucket_name: String,
    pub container_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data: Option<String>,
}
