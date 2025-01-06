use serde::{Deserialize, Serialize};
use aws_sdk_s3::{Client, Config, PutObjectRequest, Bytes};
use proxmox_schema::{api, BooleanSchema, StringSchema, Updater};

/// Schema for Cloud Backup Store name
pub const CLOUD_BACKUP_STORE_NAME_SCHEMA: Schema = StringSchema::new("Cloud Backup Store Name")
    .min_length(3)
    .max_length(64)
    .schema();

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
)]
#[allow(clippy::too_many_arguments)]
pub async fn upload_file(
    filename: web::Path<String>,
    file: web::Data<bytes::Bytes>,
    config: web::Json<CloudBackupStoreConfig>,
) -> Result<impl Responder, Error> {
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
}
