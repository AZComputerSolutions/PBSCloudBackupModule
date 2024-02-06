//! Cloud Backup Management

use anyhow::Error;
use serde_json::Value;

use proxmox_router::{list_subdirs_api_method, Router, SubdirMap};
use proxmox_schema::api;

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
    Ok("hello world".to_string())
}

const SUBDIRS: SubdirMap = &[
    ("cloud_hello", &Router::new().get(&API_METHOD_CLOUD_HELLO)),
];

pub const ROUTER: Router = Router::new()
    .get(&list_subdirs_api_method!(SUBDIRS))
    .subdirs(SUBDIRS);