#[derive(Debug, PartialEq, Eq, Clone)]
/// Media location
pub enum MediaLocation {
    /// Ready for use (inside tape library)
    Online(String),
    /// Local available, but need to be mounted (insert into tape
    /// drive)
    Offline,
    /// Media is inside a Vault
    Vault(String),
    /// Media is stored in a Cloud account
    Cloud,
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
        if s == "cloud" {
            return Ok(MediaLocation::Cloud);
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
            MediaLocation::Online(changer) => {
                write!(f, "online-{}", changer)
            }
            MediaLocation::Vault(vault) => {
                write!(f, "vault-{}", vault)
            }
            MediaLocation::Cloud => {
                write!(f, "cloud")
            }
        }
    }
}