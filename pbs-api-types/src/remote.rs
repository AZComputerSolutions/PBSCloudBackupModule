use serde::{Deserialize, Serialize};

use super::*;
use proxmox_schema::*;

// Updated password schema for cloud backup
pub const CLOUD_PASSWORD_SCHEMA: Schema =
    StringSchema::new("Password or auth token for cloud service.")
        .format(&PASSWORD_FORMAT)
        .min_length(1)
        .max_length(1024)
        .schema();

pub const CLOUD_PASSWORD_BASE64_SCHEMA: Schema =
    StringSchema::new("Password or auth token for cloud service (stored as base64 string).")
        .format(&PASSWORD_FORMAT)
        .min_length(1)
        .max_length(1024)
        .schema();

pub const CLOUD_ID_SCHEMA: Schema = StringSchema::new("Cloud Backup ID.")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(32)
    .schema();

#[api(
    properties: {
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
        service_url: {
            schema: URL_SCHEMA,
        },
        region: {
            optional: true,
            description: "The region for the cloud service (if applicable).",
            type: String,
        },
        "auth-id": {
            type: Authid,
        },
        fingerprint: {
            optional: true,
            schema: CERT_FINGERPRINT_SHA256_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize, Updater, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup configuration properties.
pub struct CloudConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub service_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    pub auth_id: Authid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[api(
    properties: {
        name: {
            schema: CLOUD_ID_SCHEMA,
        },
        config: {
            type: CloudConfig,
        },
        password: {
            schema: CLOUD_PASSWORD_BASE64_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup properties.
pub struct CloudBackup {
    pub name: String,
    // Note: The stored password is base64 encoded
    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[serde(with = "proxmox_serde::string_as_base64")]
    pub password: String,
    #[serde(flatten)]
    pub config: CloudConfig,
}

#[api(
    properties: {
        name: {
            schema: CLOUD_ID_SCHEMA,
        },
        config: {
            type: CloudConfig,
        },
    },
)]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup properties without password.
pub struct CloudBackupWithoutPassword {
    pub name: String,
    #[serde(flatten)]
    pub config: CloudConfig,
}
