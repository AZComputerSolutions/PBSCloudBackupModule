use serde::{Deserialize, Serialize};

use crate::{
    HOST_PORT_SCHEMA, HTTP_URL_SCHEMA, CLOUD_SAFE_ID_FORMAT, SINGLE_LINE_COMMENT_SCHEMA,
};
use proxmox_schema::{api, Schema, StringSchema, Updater};

pub const METRIC_SERVER_ID_SCHEMA: Schema = StringSchema::new("Metrics Server ID.")
    .format(&CLOUD_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(32)
    .schema();

pub const CLOUD_BUCKET_SCHEMA: Schema = StringSchema::new("Cloud Bucket.")
    .format(&CLOUD_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .default("cloud-backup")
    .schema();

pub const CLOUD_ORGANIZATION_SCHEMA: Schema = StringSchema::new("Cloud Organization.")
    .format(&CLOUD_SAFE_ID_FORMAT)
    .min_length(3)
    .max_length(64)
    .default("cloud-backup")
    .schema();

fn return_true() -> bool {
    true
}

fn is_true(b: &bool) -> bool {
    *b
}

#[api(
    properties: {
        name: {
            schema: METRIC_SERVER_ID_SCHEMA,
        },
        enable: {
            type: bool,
            optional: true,
            default: true,
        },
        endpoint: {
            schema: HOST_PORT_SCHEMA,
        },
        mtu: {
            type: u16,
            optional: true,
            default: 1500,
        },
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud Metrics Server (UDP)
pub struct CloudMetricsUdp {
    #[updater(skip)]
    pub name: String,
    #[serde(default = "return_true", skip_serializing_if = "is_true")]
    #[updater(serde(skip_serializing_if = "Option::is_none"))]
    /// Enables or disables the metrics server
    pub enable: bool,
    /// the endpoint + port
    pub endpoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The MTU
    pub mtu: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[api(
    properties: {
        name: {
            schema: METRIC_SERVER_ID_SCHEMA,
        },
        enable: {
            type: bool,
            optional: true,
            default: true,
        },
        url: {
            schema: HTTP_URL_SCHEMA,
        },
        token: {
            type: String,
            optional: true,
        },
        bucket: {
            schema: CLOUD_BUCKET_SCHEMA,
            optional: true,
        },
        organization: {
            schema: CLOUD_ORGANIZATION_SCHEMA,
            optional: true,
        },
        "max-body-size": {
            type: usize,
            optional: true,
            default: 50_000_000,
        },
        "verify-tls": {
            type: bool,
            optional: true,
            default: true,
        },
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud Metrics Server (HTTP(s))
pub struct CloudMetricsHttp {
    #[updater(skip)]
    pub name: String,
    #[serde(default = "return_true", skip_serializing_if = "is_true")]
    #[updater(serde(skip_serializing_if = "Option::is_none"))]
    /// Enables or disables the metrics server
    pub enable: bool,
    /// The base url of the cloud server
    pub url: String,
    /// The Optional Token
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The (optional) API token
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The (optional) maximum body size
    pub max_body_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If true, the certificate will be validated.
    pub verify_tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[api]
#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
/// Type of the cloud metrics server
pub enum MetricServerType {
    /// Cloud HTTP
    #[serde(rename = "cloud-http")]
    CloudHttp,
    /// Cloud UDP
    #[serde(rename = "cloud-udp")]
    CloudUdp,
}

#[api(
    properties: {
        name: {
            schema: METRIC_SERVER_ID_SCHEMA,
        },
        "type": {
            type: MetricServerType,
        },
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
    },
)]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
/// Basic information about a cloud metrics server
pub struct MetricServerInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: MetricServerType,
    /// Enables or disables the metrics server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    /// The target server
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
