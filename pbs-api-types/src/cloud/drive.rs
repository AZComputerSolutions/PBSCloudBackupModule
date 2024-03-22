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

use aws_sdk_s3::{Client, Config, PutObjectRequest, Bytes, PutObjectOutput};

#[api(
    method = "POST",
    path = "/upload/{filename}",
)]
#[allow(clippy::too_many_arguments)]
pub async fn upload_file(
    &self,
    filename: web::Path<String>,
    file: web::Data<bytes::Bytes>,
    config: web::Json<CloudBackupStoreConfig>,
) -> Result<impl Responder, Error> {
    let s3_client = Client::from_conf(Config::builder().build());

    let file = Bytes::from(file.into_inner());

    let req = PutObjectRequest {
        body: Some(file),
        key: filename.as_str().to_owned(),
        bucket: config.bucket.to_owned(),
        acl: Some("private".to_owned()),
        ..Default::default()
    };

    let resp = s3_client.put_object(req).await?;

    Ok(Json(resp))
}