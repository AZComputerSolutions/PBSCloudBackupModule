use ::serde::{Deserialize, Serialize};

use proxmox_schema::{api, StringSchema, Schema};

/// Schema for optional vendor attribute
pub const VENDOR_SCHEMA: Schema = StringSchema::new("Vendor (autodetected)")
    .min_length(1)
    .max_length(64)
    .schema();

/// Schema for optional model attribute
pub const MODEL_SCHEMA: Schema = StringSchema::new("Model (autodetected)")
    .min_length(1)
    .max_length(64)
    .schema();

/// Schema for optional serial attribute
pub const SERIAL_SCHEMA: Schema = StringSchema::new("Serial number (autodetected)")
    .min_length(1)
    .max_length(128)
    .schema();

#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Optional Cloud Device Identification Attributes
pub struct OptionalCloudDeviceIdentification {
    /// Vendor (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    /// Model (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Serial number (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
}

#[api()]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Kind of cloud storage
pub enum CloudStorageKind {
    /// Object storage service (e.g., AWS S3, Azure Blob Storage)
    ObjectStorage,
    /// Block storage service
    BlockStorage,
}

#[api(
    properties: {
        kind: {
            type: CloudStorageKind,
        },
    },
)]
#[derive(Debug, Serialize, Deserialize)]
/// Cloud backup storage device information
pub struct CloudBackupDeviceInfo {
    pub kind: CloudStorageKind,
    /// Service endpoint URL for the cloud backup service
    pub service_endpoint: String,
    /// Access key for the cloud backup service
    pub access_key: String,
    /// Secret key for the cloud backup service
    pub secret_key: String,
    /// Bucket or container name for the cloud backup service
    pub container_name: String,
    /// Region for the cloud backup service
    pub region: String,
    /// Optional identification attributes
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub optional_identification: Option<OptionalCloudDeviceIdentification>,
}
