#[derive(Debug, PartialEq, Eq, Clone)]
/// Media location
pub enum MediaLocation {
    /// Ready for use (inside storage service)
    Online(String),
    /// Local available, but need to be synchronized
    Offline,
    /// Media is inside a Vault
    Vault(String),
    /// Media is stored in a Cloud account
    Cloud(String),
}

// ...

impl std::str::FromStr for MediaLocation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "offline" {
            return Ok(MediaLocation::Offline);
        }
        if s.starts_with("online-") {
            return Ok(MediaLocation::Online(
                s.strip_prefix("online-").unwrap().to_string(),
            ));
        }
        if s.starts_with("vault-") {
            return Ok(MediaLocation::Vault(
                s.strip_prefix("vault-").unwrap().to_string(),
            ));
        }
        if s.starts_with("cloud-") {
            return Ok(MediaLocation::Cloud(
                s.strip_prefix("cloud-").unwrap().to_string(),
            ));
        }

        bail!("MediaLocation parse error");
    }
}

// ...

impl std::fmt::Display for MediaLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaLocation::Offline => {
                write!(f, "offline")
            }
            MediaLocation::Online(storage_service) => {
                write!(f, "online-{}", storage_service)
            }
            MediaLocation::Vault(vault) => {
                write!(f, "vault-{}", vault)
            }
            MediaLocation::Cloud(cloud_service) => {
                write!(f, "cloud-{}", cloud_service)
            }
        }
    }
}
