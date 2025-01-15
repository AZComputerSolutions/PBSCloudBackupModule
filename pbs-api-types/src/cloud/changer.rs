use serde::{Deserialize, Serialize};
use proxmox_schema::{api, ApiStringFormat, ArraySchema, IntegerSchema, Schema, StringSchema, Updater};
use reqwest::Client;  // To interact with HTTP APIs

<<<<<<< HEAD
use crate::{OptionalBackupSpecification, PROXMOX_SAFE_ID_FORMAT};

pub const BUCKET_NAME_SCHEMA: Schema = StringSchema::new("Bucket Name")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .schema();

pub const CLOUD_PROVIDER_SCHEMA: Schema = StringSchema::new("Cloud Provider")
    .format(&ApiStringFormat::PropertyString("^(aws|azure|gcp|sia)$"))
    .schema();

pub const ACCESS_KEY_SCHEMA: Schema = StringSchema::new("Access Key ID")
    .format(&ApiStringFormat::PropertyString("^[0-9a-fA-F]{32}$"))
    .schema();

pub const SECRET_KEY_SCHEMA: Schema = StringSchema::new("Secret Access Key")
    .format(&ApiStringFormat::PropertyString("^.{16,512}$"))
    .schema();

pub const REGION_SCHEMA: Schema = StringSchema::new("Region")
    .format(&ApiStringFormat::PropertyString("^[a-zA-Z0-9-_]{2,64}$"))
    .schema();

pub const CONTAINER_NAME_SCHEMA: Schema = StringSchema::new("Container Name")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .schema();

pub const EXPORT_DATA_SCHEMA: Schema = StringSchema::new("A list of blob names, comma separated, to be exported from the container.")
    .format(&ApiStringFormat::PropertyString("^[a-zA-Z0-9-_./~`!@#$%^&*()-_=+\{}\[\]:'\"|;'",<>/? ]{1,512}$"))
    .schema();

// Define the CloudBackup type
#[api(properties: { config: { type: CloudBackup, } })]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup
pub struct CloudBackup {
    #[updater(skip)]
    pub config: CloudBackupConfig,
}

// Configuration for the CloudBackup type
#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup Config
pub struct CloudBackupConfig {
    pub provider: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub region: Option<String>,
    pub bucket_name: String,
    pub container_name: String,
    pub blob_prefix: Option<String>,
    pub export_data: Option<String>,
}

// New API client for Sia
#[derive(Clone)]
pub struct SiaClient {
    pub client: Client,
    pub base_url: String,
    pub api_key: String,
}

impl SiaClient {
    // Initialize the Sia client
    pub fn new(base_url: String, api_key: String) -> SiaClient {
        let client = Client::new();
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

// Function to handle Sia backup
pub async fn handle_sia_backup(config: &CloudBackupConfig) -> Result<(), reqwest::Error> {
    if config.provider == "sia" {
        let sia_client = SiaClient::new("https://sia-api.example.com".to_string(), config.access_key_id.clone().unwrap());

        // Example: Upload file to Sia
        sia_client.upload_file("path/to/file.txt", "destination/path").await?;

        // Example: List files in a container
        let files = sia_client.list_files().await?;
        for file in files {
            println!("Found file: {}", file);
        }

        Ok(())
    } else {
        Err(reqwest::Error::new(reqwest::StatusCode::BAD_REQUEST, "Unsupported provider"))
    }
=======
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
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6
}
