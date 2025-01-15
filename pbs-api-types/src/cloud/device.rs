<<<<<<< HEAD
use serde::{Deserialize, Serialize};
use proxmox_schema::api;
=======
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
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6

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
<<<<<<< HEAD
/// Kind of device
pub enum DeviceKind {
    /// Tape changer (Autoloader, Robot)
    Changer,
    /// Normal SCSI tape device
    Tape,
    /// Sia cloud storage device
    SiaBackup,
=======
/// Kind of cloud storage
pub enum CloudStorageKind {
    /// Object storage service (e.g., AWS S3, Azure Blob Storage)
    ObjectStorage,
    /// Block storage service
    BlockStorage,
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6
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
<<<<<<< HEAD
    /// API base URL for Sia backup service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sia_api_url: Option<String>,
    /// Sia API key for authentication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sia_api_key: Option<String>,
}

// New API client for Sia (added to integrate Sia API)
#[derive(Clone)]
pub struct SiaClient {
    pub client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
}

impl SiaClient {
    // Initialize the Sia client
    pub fn new(base_url: String, api_key: String) -> SiaClient {
        let client = reqwest::Client::new();
        SiaClient { client, base_url, api_key }
    }

    // Example: Upload a file to Sia
    pub async fn upload_file(&self, file_path: &str, destination: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}/upload", self.base_url);
        let file_content = std::fs::read(file_path).expect("Unable to read file");

        let form = reqwest::multipart::Form::new()
            .part("file", reqwest::multipart::Part::bytes(file_content)
                .file_name(file_path)
                .mime_str("application/octet-stream")
                .unwrap());

        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(reqwest::Error::new(reqwest::StatusCode::BAD_REQUEST, "Failed to upload file"))
        }
    }

    // Example: List files in a container
    pub async fn list_files(&self) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/files", self.base_url);

        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        if response.status().is_success() {
            let files: Vec<String> = response.json().await?;
            Ok(files)
        } else {
            Err(reqwest::Error::new(reqwest::StatusCode::BAD_REQUEST, "Failed to list files"))
        }
    }
}

// Example function to handle cloud backup via Sia
pub async fn handle_sia_backup(device_info: &CloudBackupDeviceInfo) -> Result<(), reqwest::Error> {
    if let Some(sia_url) = &device_info.sia_api_url {
        let sia_client = SiaClient::new(sia_url.clone(), device_info.sia_api_key.clone().unwrap());

        // Example: Upload file to Sia
        sia_client.upload_file("path/to/file.txt", "destination/path").await?;

        // Example: List files in the container
        let files = sia_client.list_files().await?;
        for file in files {
            println!("Found file: {}", file);
        }

        Ok(())
    } else {
        Err(reqwest::Error::new(reqwest::StatusCode::BAD_REQUEST, "Sia API URL is not provided"))
    }
=======
    /// Optional identification attributes
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub optional_identification: Option<OptionalCloudDeviceIdentification>,
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6
}
