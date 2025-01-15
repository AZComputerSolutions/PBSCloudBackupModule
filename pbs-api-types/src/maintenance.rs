use anyhow::{bail, Error};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use proxmox_schema::{api, const_regex, ApiStringFormat, Schema, StringSchema};

const_regex! {
    pub MAINTENANCE_MESSAGE_REGEX = r"^[[:^cntrl:]]*$";
}

pub const MAINTENANCE_MESSAGE_FORMAT: ApiStringFormat =
    ApiStringFormat::Pattern(&MAINTENANCE_MESSAGE_REGEX);

pub const MAINTENANCE_MESSAGE_SCHEMA: Schema =
    StringSchema::new("Message describing the reason for the maintenance.")
        .format(&MAINTENANCE_MESSAGE_FORMAT)
        .max_length(64)
        .schema();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Operation requirements, used when checking for maintenance mode in cloud backups.
pub enum Operation {
    /// For any read operation, such as backup restoration or metadata fetching.
    Read,
    /// For any write/delete operation, such as creating or deleting backups.
    Write,
    /// For any operation on in-memory state, like checking cloud storage status.
    /// 
    /// NOTE: This does not involve performing IO operations.
    Lookup,
}

#[api]
#[derive(Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
/// Maintenance type for cloud backup systems.
pub enum MaintenanceType {
    /// Only read operations are allowed.
    ReadOnly,
    /// Neither read nor write operations are allowed.
    Offline,
    /// The cloud storage bucket is being deleted or reconfigured.
    Delete,
}
serde_plain::derive_display_from_serialize!(MaintenanceType);
serde_plain::derive_fromstr_from_deserialize!(MaintenanceType);

#[api(
    properties: {
        type: {
            type: MaintenanceType,
        },
        message: {
            optional: true,
            schema: MAINTENANCE_MESSAGE_SCHEMA,
        }
    },
    default_key: "type",
)]
#[derive(Deserialize, Serialize)]
/// Maintenance mode for cloud backups.
pub struct MaintenanceMode {
    /// Type of maintenance ("read-only" or "offline").
    #[serde(rename = "type")]
    ty: MaintenanceType,

    /// Reason for maintenance.
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl MaintenanceMode {
    /// Checks the current maintenance mode against an attempted operation.
    pub fn check(&self, operation: Option<Operation>) -> Result<(), Error> {
        if self.ty == MaintenanceType::Delete {
            bail!("cloud storage bucket is being deleted");
        }

        let message = percent_encoding::percent_decode_str(self.message.as_deref().unwrap_or(""))
            .decode_utf8()
            .unwrap_or(Cow::Borrowed(""));

        if let Some(Operation::Lookup) = operation {
            return Ok(());
        } else if self.ty == MaintenanceType::Offline {
            bail!("offline maintenance mode: {}", message);
        } else if self.ty == MaintenanceType::ReadOnly {
            if let Some(Operation::Write) = operation {
                bail!("read-only maintenance mode: {}", message);
            }
        }
        Ok(())
    }
}
