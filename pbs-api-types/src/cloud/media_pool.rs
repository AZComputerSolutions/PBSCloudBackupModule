//! Types for cloud media pool API
//!
//! Note: Both MediaSetPolicy and RetentionPolicy are complex enums,
//! so we cannot use them directly for the API. Instead, we represent
//! them as String.

use std::str::FromStr;

use anyhow::Error;
use serde::{Deserialize, Serialize};

use proxmox_schema::{api, ApiStringFormat, Schema, StringSchema, Updater};

use proxmox_time::{CalendarEvent, TimeSpan};

use crate::{
    PROXMOX_SAFE_ID_FORMAT, SINGLE_LINE_COMMENT_FORMAT, SINGLE_LINE_COMMENT_SCHEMA,
};

pub const CLOUD_MEDIA_POOL_NAME_SCHEMA: Schema = StringSchema::new("Cloud media pool name.")
    .format(&PROXMOX_SAFE_ID_FORMAT)
    .min_length(2)
    .max_length(32)
    .schema();

pub const CLOUD_MEDIA_SET_NAMING_TEMPLATE_SCHEMA: Schema = StringSchema::new(
    "Cloud media set naming template (may contain strftime() time format specifications).",
)
.format(&SINGLE_LINE_COMMENT_FORMAT)
.min_length(2)
.max_length(64)
.schema();

pub const CLOUD_MEDIA_SET_ALLOCATION_POLICY_FORMAT: ApiStringFormat = ApiStringFormat::VerifyFn(|s| {
    MediaSetPolicy::from_str(s)?;
    Ok(())
});

pub const CLOUD_MEDIA_SET_ALLOCATION_POLICY_SCHEMA: Schema =
    StringSchema::new("Cloud media set allocation policy ('continue', 'always', or a calendar event).")
        .format(&CLOUD_MEDIA_SET_ALLOCATION_POLICY_FORMAT)
        .schema();

/// Media set allocation policy for cloud storage
pub enum MediaSetPolicy {
    /// Try to use the current media set
    ContinueCurrent,
    /// Each backup job creates a new media set
    AlwaysCreate,
    /// Create a new set when the specified CalendarEvent triggers
    CreateAt(CalendarEvent),
}

impl std::str::FromStr for MediaSetPolicy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "continue" {
            return Ok(MediaSetPolicy::ContinueCurrent);
        }
        if s == "always" {
            return Ok(MediaSetPolicy::AlwaysCreate);
        }

        let event = s.parse()?;

        Ok(MediaSetPolicy::CreateAt(event))
    }
}

pub const CLOUD_MEDIA_RETENTION_POLICY_FORMAT: ApiStringFormat = ApiStringFormat::VerifyFn(|s| {
    RetentionPolicy::from_str(s)?;
    Ok(())
});

pub const CLOUD_MEDIA_RETENTION_POLICY_SCHEMA: Schema =
    StringSchema::new("Cloud media retention policy ('overwrite', 'keep', or time span).")
        .format(&CLOUD_MEDIA_RETENTION_POLICY_FORMAT)
        .schema();

/// Media retention Policy for cloud storage
pub enum RetentionPolicy {
    /// Always overwrite media
    OverwriteAlways,
    /// Protect data for the timespan specified
    ProtectFor(TimeSpan),
    /// Never overwrite data
    KeepForever,
}

impl std::str::FromStr for RetentionPolicy {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "overwrite" {
            return Ok(RetentionPolicy::OverwriteAlways);
        }
        if s == "keep" {
            return Ok(RetentionPolicy::KeepForever);
        }

        let time_span = s.parse()?;

        Ok(RetentionPolicy::ProtectFor(time_span))
    }
}

#[api(
    properties: {
        name: {
            schema: CLOUD_MEDIA_POOL_NAME_SCHEMA,
        },
        allocation: {
            schema: CLOUD_MEDIA_SET_ALLOCATION_POLICY_SCHEMA,
            optional: true,
        },
        retention: {
            schema: CLOUD_MEDIA_RETENTION_POLICY_SCHEMA,
            optional: true,
        },
        template: {
            schema: CLOUD_MEDIA_SET_NAMING_TEMPLATE_SCHEMA,
            optional: true,
        },
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
    },
)]
#[derive(Serialize, Deserialize, Updater)]
/// Cloud media pool configuration
pub struct CloudMediaPoolConfig {
    /// The cloud pool name
    #[updater(skip)]
    pub name: String,
    /// Media Set allocation policy for cloud
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation: Option<String>,
    /// Media retention policy for cloud
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
    /// Media set naming template (default "%c")
    ///
    /// The template is UTF8 text, and can include strftime time
    /// format specifications.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
