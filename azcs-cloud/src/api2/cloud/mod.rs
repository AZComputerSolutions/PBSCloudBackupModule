//! Cloud Backup Management

use anyhow::Error;
use serde_json::Value;

use proxmox_router::{list_subdirs_api_method, Router, SubdirMap};
use proxmox_schema::api;

pub mod backup;
pub mod restore;

#[api(
    input: {
        properties: {},
    },
    returns: {
        description: "Cloud hello world.",
        type: String,
    },
)]
/// Cloud Hello
pub fn cloud_hello(_param: Value) -> Result<String, Error> {
    Ok("api2/json/cloud/cloud-hello cloud-hello-world".to_string())
}

const SUBDIRS: SubdirMap = &[
    (
        "cloud-hello",
        &Router::new().get(&API_METHOD_CLOUD_HELLO),
    ),
    //("scan-drives", &Router::new().get(&API_METHOD_SCAN_DRIVES)),
];

pub const ROUTER: Router = Router::new()
    .get(&list_subdirs_api_method!(SUBDIRS))
    .subdirs(SUBDIRS);
