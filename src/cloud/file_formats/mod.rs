

// openssl::sha::sha256(b"Proxmox Backup Catalog Archive v1.0")[0..8];
pub const PROXMOX_BACKUP_CATALOG_ARCHIVE_MAGIC_1_0: [u8; 8] =
    [183, 207, 199, 37, 158, 153, 30, 115];
// v1.1 introduced an optional, in-line namespace prefix for the snapshot field
// openssl::sha::sha256(b"Proxmox Backup Catalog Archive v1.1")[0..8];
pub const PROXMOX_BACKUP_CATALOG_ARCHIVE_MAGIC_1_1: [u8; 8] = [179, 236, 113, 240, 173, 236, 2, 96];
