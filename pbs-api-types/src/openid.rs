use serde::{Deserialize, Serialize};

use proxmox_schema::{api, ApiStringFormat, ArraySchema, Schema, StringSchema, Updater};

use super::{
    CLOUD_SAFE_ID_FORMAT, CLOUD_SAFE_ID_REGEX, CLOUD_REALM_ID_SCHEMA, SINGLE_LINE_COMMENT_SCHEMA,
};

pub const OPENID_SCOPE_FORMAT: ApiStringFormat = ApiStringFormat::Pattern(&CLOUD_SAFE_ID_REGEX);

pub const OPENID_SCOPE_SCHEMA: Schema = StringSchema::new("Cloud OpenID Scope Name.")
    .format(&OPENID_SCOPE_FORMAT)
    .schema();

pub const OPENID_SCOPE_ARRAY_SCHEMA: Schema =
    ArraySchema::new("Array of Cloud OpenID Scopes.", &OPENID_SCOPE_SCHEMA).schema();

pub const OPENID_SCOPE_LIST_FORMAT: ApiStringFormat =
    ApiStringFormat::PropertyString(&OPENID_SCOPE_ARRAY_SCHEMA);

pub const OPENID_DEFAULT_SCOPE_LIST: &str = "email profile";
pub const OPENID_SCOPE_LIST_SCHEMA: Schema = StringSchema::new("Cloud OpenID Scope List")
    .format(&OPENID_SCOPE_LIST_FORMAT)
    .default(OPENID_DEFAULT_SCOPE_LIST)
    .schema();

pub const OPENID_ACR_FORMAT: ApiStringFormat = ApiStringFormat::Pattern(&CLOUD_SAFE_ID_REGEX);

pub const OPENID_ACR_SCHEMA: Schema =
    StringSchema::new("Cloud OpenID Authentication Context Class Reference.")
        .format(&OPENID_SCOPE_FORMAT)
        .schema();

pub const OPENID_ACR_ARRAY_SCHEMA: Schema =
    ArraySchema::new("Array of Cloud OpenID ACRs.", &OPENID_ACR_SCHEMA).schema();

pub const OPENID_ACR_LIST_FORMAT: ApiStringFormat =
    ApiStringFormat::PropertyString(&OPENID_ACR_ARRAY_SCHEMA);

pub const OPENID_ACR_LIST_SCHEMA: Schema = StringSchema::new("Cloud OpenID ACR List")
    .format(&OPENID_ACR_LIST_FORMAT)
    .schema();

pub const OPENID_USERNAME_CLAIM_SCHEMA: Schema = StringSchema::new(
    "Use the value of this attribute/claim as the unique user name. It \
    is up to the identity provider to guarantee the uniqueness. The \
    OpenID specification only guarantees that Subject ('sub') is \
    unique. Also, ensure that the user cannot change this \
    attribute independently.",
)
.max_length(64)
.min_length(1)
.format(&CLOUD_SAFE_ID_FORMAT)
.schema();

#[api(
    properties: {
        realm: {
            schema: CLOUD_REALM_ID_SCHEMA,
        },
        "client-key": {
            optional: true,
        },
        "scopes": {
            schema: OPENID_SCOPE_LIST_SCHEMA,
            optional: true,
        },
        "acr-values": {
            schema: OPENID_ACR_LIST_SCHEMA,
            optional: true,
        },
        prompt: {
            description: "Cloud OpenID Prompt",
            type: String,
            format: &CLOUD_SAFE_ID_FORMAT,
            optional: true,
        },
        comment: {
            optional: true,
            schema: SINGLE_LINE_COMMENT_SCHEMA,
        },
        autocreate: {
            optional: true,
            default: true,
        },
        "username-claim": {
            schema: OPENID_USERNAME_CLAIM_SCHEMA,
            optional: true,
        },
    },
)]
#[derive(Serialize, Deserialize, Updater)]
#[serde(rename_all = "kebab-case")]
/// Cloud OpenID configuration properties.
pub struct CloudOpenIdConfig {
    #[updater(skip)]
    pub realm: String,
    /// OpenID Issuer URL
    pub issuer_url: String,
    /// OpenID Client ID
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acr_values: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// OpenID Client Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Automatically create users if they do not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocreate: Option<bool>,
    #[updater(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_claim: Option<String>,
}
