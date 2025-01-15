//! Basic API types used by most of the cloud backup code.

use serde::{Deserialize, Serialize};

use proxmox_auth_api::{APITOKEN_ID_REGEX_STR, USER_ID_REGEX_STR};

pub mod common_regex;
pub mod percent_encoding;

use proxmox_schema::{
    api, const_regex, ApiStringFormat, ApiType, ArraySchema, ReturnType, Schema, StringSchema,
};
use proxmox_time::parse_daily_duration;

#[rustfmt::skip]
#[macro_export]
macro_rules! PROXMOX_SAFE_ID_REGEX_STR { () => { r"(?:[A-Za-z0-9_][A-Za-z0-9._\-]*)" }; }

#[rustfmt::skip]
#[macro_export]
macro_rules! BACKUP_ID_RE { () => (r"[A-Za-z0-9_][A-Za-z0-9._\-]*") }

#[rustfmt::skip]
#[macro_export]
macro_rules! BACKUP_TYPE_RE { () => (r"(?:cloud|host|vm|ct)") }

#[rustfmt::skip]
#[macro_export]
macro_rules! BACKUP_TIME_RE { () => (r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z") }

#[rustfmt::skip]
#[macro_export]
macro_rules! BACKUP_NS_RE {
    () => (
        concat!("(?:",
            "(?:", PROXMOX_SAFE_ID_REGEX_STR!(), r"/){0,7}", PROXMOX_SAFE_ID_REGEX_STR!(),
        ")?")
    );
}

#[rustfmt::skip]
#[macro_export]
macro_rules! BACKUP_NS_PATH_RE {
    () => (
        concat!(r"(?:ns/", PROXMOX_SAFE_ID_REGEX_STR!(), r"/){0,7}ns/", PROXMOX_SAFE_ID_REGEX_STR!(), r"/")
    );
}

#[rustfmt::skip]
#[macro_export]
macro_rules! SNAPSHOT_PATH_REGEX_STR {
    () => (
        concat!(
            r"(", BACKUP_TYPE_RE!(), ")/(", BACKUP_ID_RE!(), ")/(", BACKUP_TIME_RE!(), r")",
        )
    );
}

#[rustfmt::skip]
#[macro_export]
macro_rules! GROUP_OR_SNAPSHOT_PATH_REGEX_STR {
    () => {
        concat!(
            r"(", BACKUP_TYPE_RE!(), ")/(", BACKUP_ID_RE!(), ")(?:/(", BACKUP_TIME_RE!(), r"))?",
        )
    };
}

mod acl;
pub use acl::*;

mod datastore;
pub use datastore::*;

mod jobs;
pub use jobs::*;

mod key_derivation;
pub use key_derivation::{Kdf, KeyInfo};

mod maintenance;
pub use maintenance::*;

mod network;
pub use network::*;

mod node;
pub use node::*;

pub use proxmox_auth_api::types as userid;
pub use proxmox_auth_api::types::{Authid, Userid};
pub use proxmox_auth_api::types::{Realm, RealmRef};
pub use proxmox_auth_api::types::{Tokenname, TokennameRef};
pub use proxmox_auth_api::types::{Username, UsernameRef};
pub use proxmox_auth_api::types::{
    PROXMOX_GROUP_ID_SCHEMA, PROXMOX_TOKEN_ID_SCHEMA, PROXMOX_TOKEN_NAME_SCHEMA,
};

#[macro_use]
mod user;
pub use user::*;

pub use proxmox_schema::upid::*;

mod crypto;
pub use crypto::{bytes_as_fingerprint, CryptMode, Fingerprint};

pub mod file_restore;

mod openid;
pub use openid::*;

mod ldap;
pub use ldap::*;

mod remote;
pub use remote::*;

mod cloud;
pub use cloud::*;  // Cloud module that integrates with cloud storage APIs (e.g., S3, Google Cloud Storage).

mod tape;
pub use tape::*;  // This could be replaced with cloud backup-related implementations.

mod traffic_control;
pub use traffic_control::*;

mod zfs;
pub use zfs::*;

mod metrics;
pub use metrics::*;

#[rustfmt::skip]
#[macro_use]
mod local_macros {
    macro_rules! DNS_LABEL { () => (r"(?:[a-zA-Z0-9](?:[a-zA-Z0-9\-]*[a-zA-Z0-9])?)") }
    macro_rules! DNS_NAME { () => (concat!(r"(?:(?:", DNS_LABEL!() , r"\.)*", DNS_LABEL!(), ")")) }
    macro_rules! CIDR_V4_REGEX_STR { () => (concat!(r"(?:", IPV4RE!(), r"/\d{1,2})$")) }
    macro_rules! CIDR_V6_REGEX_STR { () => (concat!(r"(?:", IPV6RE!(), r"/\d{1,3})$")) }
    macro_rules! DNS_ALIAS_LABEL { () => (r"(?:[a-zA-Z0-9_](?:[a-zA-Z0-9\-]*[a-zA-Z0-9])?)") }
    macro_rules! DNS_ALIAS_NAME {
        () => (concat!(r"(?:(?:", DNS_ALIAS_LABEL!() , r"\.)*", DNS_ALIAS_LABEL!(), ")"))
    }
    macro_rules! PORT_REGEX_STR { () => (r"(?:[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])") }
}

const_regex! {
    pub IP_V4_REGEX = concat!(r"^", IPV4RE!(), r"$");
    pub IP_V6_REGEX = concat!(r"^", IPV6RE!(), r"$");
    pub IP_REGEX = concat!(r"^", IPRE!(), r"$");
    pub CIDR_V4_REGEX =  concat!(r"^", CIDR_V4_REGEX_STR!(), r"$");
    pub CIDR_V6_REGEX =  concat!(r"^", CIDR_V6_REGEX_STR!(), r"$");
    pub CIDR_REGEX =  concat!(r"^(?:", CIDR_V4_REGEX_STR!(), "|",  CIDR_V6_REGEX_STR!(), r")$");
    pub HOSTNAME_REGEX = r"^(?:[a-zA-Z0-9](?:[a-zA-Z0-9\-]*[a-zA-Z0-9])?)$";
    pub DNS_NAME_REGEX =  concat!(r"^", DNS_NAME!(), r"$");
    pub DNS_ALIAS_REGEX =  concat!(r"^", DNS_ALIAS_NAME!(), r"$");
    pub DNS_NAME_OR_IP_REGEX = concat!(r"^(?:", DNS_NAME!(), "|",  IPRE!(), r")$");
    pub HOST_PORT_REGEX = concat!(r"^(?:", DNS_NAME!(), "|", IPRE_BRACKET!(), "):", PORT_REGEX_STR!() ,"$");
    pub HTTP_URL_REGEX = concat!(r"^https?://(?:(?:(?:", DNS_NAME!(), "|", IPRE_BRACKET!(), ")(?::", PORT_REGEX_STR!() ,")?)|", IPV6RE!(),")(?:/[^\x00-\x1F\x7F]*)?$");

    pub SHA256_HEX_REGEX = r"^[a-f0-9]{64}$"; // fixme: define in common_regex ?

    pub PASSWORD_REGEX = r"^[[:^cntrl:]]*$"; // everything but control characters

    pub UUID_REGEX = r"^[0-9a-f]{8}(?:-[0-9a-f]{4}){3}-[0-9a-f]{12}$";

    pub SYSTEMD_DATETIME_REGEX = r"^\d{4}-\d{2}-\d{2}( \d{2}:\d{2}(:\d{2})?)?$"; // fixme: define in common_regex ?

    pub FINGERPRINT_SHA256_REGEX = r"^(?:[0-9a-fA-F][0-9a-fA-F])(?::[0-9a-fA-F][0-9a-fA-F]){31}$";

    pub OPENSSL_CIPHERS_REGEX = r"^[0-9A-Za-z_:, +!\-@=.]+$";

    pub PROXMOX_SAFE_ID_REGEX = concat!(r"^", PROXMOX_SAFE_ID_REGEX_STR!(), r"$");

    pub SINGLE_LINE_COMMENT_REGEX = r"^[[:^cntrl:]]*$";

    pub MULTI_LINE_COMMENT_REGEX = r"(?m)^([[:^cntrl:]]*)$";

    pub BACKUP_REPO_URL_REGEX = concat!(
        r"^^(?:(?:(",
        USER_ID_REGEX_STR!(), "|", APITOKEN_ID_REGEX_STR!(),
        ")@)?(",
        DNS_NAME!(), "|",  IPRE_BRACKET!(),
        "):)?(?:([0-9]{1,5}):)?(", PROXMOX_SAFE_ID_REGEX_STR!(), r")$"
    );

    pub BLOCKDEVICE_NAME_REGEX = r"^(?:(?:h|s|x?v)d[a-z]+)|(?:nvme\d+n\d+)$";
    pub BLOCKDEVICE_DISK_AND_PARTITION_NAME_REGEX = r"^(?:(?:h|s|x?v)d[a-z]+\d*)|(?:nvme\d+n\d+(p\d+)?)$";
    pub SUBSCRIPTION_KEY_REGEX = r"^([A-Za-z0-9]{4}-){7}[A-Za-z0-9]{4}$";
}

// Cloud Backup - Module to interact with cloud storage (AWS S3 example)
mod cloud {
    use rusoto_core::Region;
    use rusoto_s3::{S3, S3Client, PutObjectRequest, GetObjectRequest};
    
    pub fn upload_to_cloud(bucket: &str, key: &str, data: Vec<u8>) -> Result<(), String> {
        let s3_client = S3Client::new(Region::UsEast1);
        let put_request = PutObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            body: Some(data.into()),
            ..Default::default()
        };
        s3_client.put_object(put_request).sync().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn download_from_cloud(bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        let s3_client = S3Client::new(Region::UsEast1);
        let get_request = GetObjectRequest {
            bucket: bucket.to_string(),
            key: key.to_string(),
            ..Default::default()
        };
        let result = s3_client.get_object(get_request).sync().map_err(|e| e.to_string())?;
        result.body.map(|b| b.into_iter().collect()).map_err(|e| e.to_string())
    }
}

// Usage example
fn main() {
    // Example of uploading data to cloud (AWS S3)
    let data = b"Hello, Cloud Backup!".to_vec();
    match cloud::upload_to_cloud("my-backup-bucket", "backup/my-backup-key", data) {
        Ok(()) => println!("Backup uploaded successfully!"),
        Err(e) => println!("Failed to upload backup: {}", e),
    }

    // Example of downloading data from cloud (AWS S3)
    match cloud::download_from_cloud("my-backup-bucket", "backup/my-backup-key") {
        Ok(data) => println!("Backup downloaded: {:?}", String::from_utf8(data)),
        Err(e) => println!("Failed to download backup: {}", e),
    }
}
