use serde::{Deserialize, Serialize};

use proxmox_schema::{ api, ApiStringFormat, ArraySchema, IntegerSchema, Schema, StringSchema, Updater, };

use crate::{ OptionalBackupSpecification, PROXMOX_SAFE_ID_FORMAT, };

pub const BUCKET_NAME_SCHEMA: Schema = StringSchema::new("Bucket Name") .format(&PROXMOX_SAFE_ID_FORMAT) .min_length(3) .max_length(64) .schema();

pub const CLOUD_ PROVIDER_SCHEMA: Schema = StringSchema::new("Cloud Provider") .format(&ApiStringFormat::PropertyString( "^(aws|azure|gcp)$", )) .schema();

pub const ACCESS_KEY_SCHEMA: Schema = StringSchema::new("Access Key ID") .format(&ApiStringFormat::PropertyString( "^[0-9a-fA-F]{32}$", )) .schema();

pub const SECRET_KEY_SCHEMA: Schema = StringSchema::new("Secret Access Key") .format(&ApiStringFormat::PropertyString( "^.{16,512}$", )) .schema();

pub const REGION_SCHEMA: Schema = StringSchema::new("Region") .format(&ApiStringFormat::PropertyString( "^[a-zA-Z0-9-_]{2,64}$", )) .schema();

pub const CONTAINER_NAME_SCHEMA: Schema = StringSchema::new("Container Name") .format(&PROXMOX_SAFE_ID_FORMAT) .min_length(3) .max_length(64) .schema();

pub const EXPORT_DATA_SCHEMA: Schema = StringSchema::new( "A list of blob names, comma separated, to be exported from the container.", ) .format(&ApiStringFormat::PropertyString( "^[a-zA-Z0-9-_./~`!@#$%^&*()-_=+\{}\[\]:"'\|;'",<>/? ]{1,512}$", )) .schema();

// Definition of the CloudBackup type #[api( properties: { config: { type: CloudBackup, } } )] #[derive(Serialize, Deserialize, Updater)] #[serde(rename_all = "kebab-case")] /// Cloud Backup pub struct CloudBackup { #[updater(skip)] pub config: CloudBackupConfig, }

// Configuration of the CloudBackup type #[api()] #[derive(Serialize, Deserialize)] #[serde(rename_all = "kebab-case")] /// Cloud Backup Config pub struct CloudBackupConfig { pub provider: String, pub access_key_id: Option<String>, pub secret_access_key: Option<String>, pub region: Option<String>, pub bucket_name: String, pub container_name: String, pub blob_prefix: Option<String>, pub export_data: Option<String>, }