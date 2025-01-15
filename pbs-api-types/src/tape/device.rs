use ::serde::{Deserialize, Serialize};

use proxmox_schema::api;

#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Optional Cloud Service Identification Attributes
pub struct OptionalCloudServiceIdentification {
    /// Service Name (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// Service Region (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Access Key (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// Account ID (autodetected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[api()]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Kind of cloud storage
pub enum CloudStorageKind {
    /// Object storage service
    ObjectStorage,
    /// File-based storage service
    FileStorage,
}

#[api(
    properties: {
        kind: {
            type: CloudStorageKind,
        },
    },
)]
#[derive(Debug, Serialize, Deserialize)]
/// Cloud storage information
pub struct CloudStorageInfo {
    pub kind: CloudStorageKind,
    /// URL to the cloud storage service endpoint
    pub endpoint_url: String,
    /// Access Key ID for authentication
    pub access_key_id: String,
    /// Secret Access Key for authentication
    pub secret_access_key: String,
    /// Service Name (autodetected)
    pub service_name: String,
    /// Storage Region
    pub region: String,
    /// Maximum allowed storage capacity (in GB)
    pub max_capacity_gb: u32,
    /// Current used storage capacity (in GB)
    pub used_capacity_gb: u32,
}
