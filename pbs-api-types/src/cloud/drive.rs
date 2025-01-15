use serde::{Deserialize, Serialize};
use proxmox_schema::api;
use reqwest::Client as ReqwestClient;
use reqwest::multipart::{Form, Part};
use serde_json::json;

#[api(
    properties: {
        name: {
            schema: DRIVE_NAME_SCHEMA,
        },
        config: {
            type: CloudBackupStoreConfig,
        },
        info: {
            type: OptionalDeviceIdentification,
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
/// Cloud backup store
pub struct CloudBackupStore {
    #[updater(skip)]
    pub name: String,
    #[serde(flatten)]
    pub config: CloudBackupStoreConfig,
    #[serde(flatten)]
    pub info: OptionalDeviceIdentification,
    /// Indicates if the cloud storage is connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
}

#[api(
    method = "POST",
    path = "/upload/",
)]
#[allow(clippy::too_many_arguments)]
pub async fn upload_file(
    &self,
    filename: web::Path<String>,
    file: web::Data<bytes::Bytes>,
    config: web::Json<CloudBackupStoreConfig>,
) -> Result<impl Responder, Error> {
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
}
