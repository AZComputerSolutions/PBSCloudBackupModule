use std::fmt;

use serde::{Deserialize, Serialize};

use proxmox_schema::*;

use crate::{
    CIDR_FORMAT, CIDR_V4_FORMAT, CIDR_V6_FORMAT, IP_FORMAT, IP_V4_FORMAT, IP_V6_FORMAT,
    PROXMOX_SAFE_ID_REGEX,
};

pub const NETWORK_INTERFACE_FORMAT: ApiStringFormat =
    ApiStringFormat::Pattern(&PROXMOX_SAFE_ID_REGEX);

pub const IP_V4_SCHEMA: Schema = StringSchema::new("Cloud IPv4 address.")
    .format(&IP_V4_FORMAT)
    .max_length(15)
    .schema();

pub const IP_V6_SCHEMA: Schema = StringSchema::new("Cloud IPv6 address.")
    .format(&IP_V6_FORMAT)
    .max_length(39)
    .schema();

pub const CIDR_V4_SCHEMA: Schema = StringSchema::new("Cloud IPv4 CIDR block.")
    .format(&CIDR_V4_FORMAT)
    .max_length(18)
    .schema();

pub const CIDR_V6_SCHEMA: Schema = StringSchema::new("Cloud IPv6 CIDR block.")
    .format(&CIDR_V6_FORMAT)
    .max_length(43)
    .schema();

#[api()]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Interface configuration method
pub enum CloudNetworkConfigMethod {
    /// Manually configured network
    Manual,
    /// Dynamic IP allocation (Cloud DHCP equivalent)
    Dynamic,
    /// Use predefined cloud network subnet
    Subnet,
    /// Cloud-specific loopback
    Loopback,
}

#[api()]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[repr(u8)]
/// Virtual Network Interface Types
pub enum CloudNetworkInterfaceType {
    /// Cloud-specific virtual network adapter
    VirtualAdapter,
    /// Public-facing network interface
    PublicInterface,
    /// Private internal cloud network
    PrivateInterface,
    /// Loopback
    Loopback,
    /// Unknown interface type
    Unknown,
}

#[api(
    properties: {
        name: {
            schema: NETWORK_INTERFACE_FORMAT,
        },
        "type": {
            type: CloudNetworkInterfaceType,
        },
        method: {
            type: CloudNetworkConfigMethod,
            optional: true,
        },
        cidr: {
            schema: CIDR_V4_SCHEMA,
            optional: true,
        },
        cidr6: {
            schema: CIDR_V6_SCHEMA,
            optional: true,
        },
        gateway: {
            schema: IP_V4_SCHEMA,
            optional: true,
        },
        gateway6: {
            schema: IP_V6_SCHEMA,
            optional: true,
        },
        subnet_id: {
            description: "Cloud Subnet ID",
            type: String,
            optional: true,
        },
        tags: {
            description: "Cloud-specific metadata tags",
            type: Array,
            items: {
                description: "Metadata tag",
                type: String,
            },
        },
        comments: {
            description: "Comments (may span multiple lines)",
            type: String,
            optional: true,
        },
    }
)]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Cloud Network Interface configuration
pub struct CloudInterface {
    /// Interface name
    pub name: String,
    /// Interface type
    #[serde(rename = "type")]
    pub interface_type: CloudNetworkInterfaceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<CloudNetworkConfigMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv4 CIDR
    pub cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6 CIDR
    pub cidr6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv4 Gateway
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// IPv6 Gateway
    pub gateway6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Cloud Subnet ID
    pub subnet_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

impl CloudInterface {
    pub fn new(name: String) -> Self {
        Self {
            name,
            interface_type: CloudNetworkInterfaceType::Unknown,
            method: None,
            cidr: None,
            cidr6: None,
            gateway: None,
            gateway6: None,
            subnet_id: None,
            tags: Vec::new(),
            comments: None,
        }
    }
}
