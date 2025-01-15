use serde::{Deserialize, Serialize};
<<<<<<< HEAD
use proxmox_schema::api;
use reqwest::Client as ReqwestClient;
use reqwest::multipart::{Form, Part};
use serde_json::json;
=======
use aws_sdk_s3::{Client, Config, PutObjectRequest, Bytes};
use proxmox_schema::{api, BooleanSchema, StringSchema, Updater};

/// Schema for Cloud Backup Store name
pub const CLOUD_BACKUP_STORE_NAME_SCHEMA: Schema = StringSchema::new("Cloud Backup Store Name")
    .min_length(3)
    .max_length(64)
    .schema();
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6

#[api(
    properties: {
        name: {
            schema: CLOUD_BACKUP_STORE_NAME_SCHEMA,
        },
        config: {
            type: CloudBackupStoreConfig,
        },
        info: {
            type: OptionalCloudDeviceIdentification,
        },
        connected: {
            schema: BooleanSchema,
            optional: true,
            default: false,
        },
    }
)]
#[derive(Serialize, Deserialize, Updater, Clone)]
#[serde(rename_all = "kebab-case")]
/// Cloud Backup Store
pub struct CloudBackupStore {
    #[updater(skip)]
    pub name: String,
    #[serde(flatten)]
    pub config: CloudBackupStoreConfig,
    #[serde(flatten)]
    pub info: OptionalCloudDeviceIdentification,
    /// Indicates if the cloud storage is connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

<<<<<<< HEAD
#[api(
    method = "POST",
    path = "/upload/",
=======
/// Configuration for the Cloud Backup Store
#[api()]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CloudBackupStoreConfig {
    /// The name of the bucket or container
    pub container_name: String,
    /// The region of the cloud service
    pub region: String,
    /// Cloud service endpoint (optional, for custom S3 endpoints)
    pub service_endpoint: Option<String>,
    /// Access key for the cloud storage
    pub access_key: String,
    /// Secret key for the cloud storage
    pub secret_key: String,
}

#[api(
    method = "POST",
    path = "/upload/{filename}",
    description = "Upload a file to the cloud backup store."
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6
)]
#[allow(clippy::too_many_arguments)]
pub async fn upload_file(
    filename: web::Path<String>,
    file: web::Data<bytes::Bytes>,
    config: web::Json<CloudBackupStoreConfig>,
) -> Result<impl Responder, Error> {
<<<<<<< HEAD
    // Create a new Sia API client instance
    let sia_client = SiaClient::new(config.sia_api_url.clone(), config.sia_api_key.clone().unwrap());

    // Convert the file bytes to a Vec
    let file_content = file.into_inner().to_vec();

    // Attempt to upload the file to Sia
    match sia_client.upload_file(filename.as_str(), file_content).await {
        Ok(_) => Ok(Json(json!({ "message": "File uploaded successfully" }))),
        Err(err) => Err(Error::from(err)),
    }
}

#[derive(Clone)]
pub struct SiaClient {
    pub client: ReqwestClient,
    pub base_url: String,
    pub api_key: String,
}

impl SiaClient {
    // Initialize the Sia client
    pub fn new(base_url: String, api_key: String) -> SiaClient {
        let client = ReqwestClient::new();
        SiaClient { client, base_url, api_key }
    }

    // Example: Upload a file to Sia
    pub async fn upload_file(&self, filename: &str, file_content: Vec<u8>) -> Result<(), reqwest::Error> {
        let url = format!("{}/upload", self.base_url);
        
        let form = Form::new()
            .part("file", Part::bytes(file_content)
                .file_name(filename)
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
=======
    // Create S3 client with provided configuration
    let s3_config = aws_sdk_s3::Config::builder()
        .region(aws_sdk_s3::Region::new(config.region.clone()))
        .credentials_provider(aws_sdk_s3::Credentials::from_keys(
            config.access_key.clone(),
            config.secret_key.clone(),
            None,
        ))
        .endpoint_url(config.service_endpoint.clone().unwrap_or_default())
        .build();
    let s3_client = aws_sdk_s3::Client::from_conf(s3_config);

    // Prepare the file for upload
    let file = Bytes::from(file.into_inner());

    // Build the request
    let req = PutObjectRequest::builder()
        .bucket(config.container_name.clone())
        .key(filename.to_string())
        .body(file.into())
        .acl("private")
        .build();

    // Upload the file
    let resp = s3_client.put_object(req).await?;

    Ok(web::Json(resp))
>>>>>>> c5486c7693dec02d9e75fc2b811164ac694c4be6
}
