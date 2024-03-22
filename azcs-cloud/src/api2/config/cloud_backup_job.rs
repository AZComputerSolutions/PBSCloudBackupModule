use ::serde::{Deserialize, Serialize};
use anyhow::Error;
use hex::FromHex;
use serde_json::Value;

use proxmox_router::{http_bail, Permission, Router, RpcEnvironment};
use proxmox_schema::{api, param_bail};

use pbs_api_types::{
    Authid, CloudBackupJobConfig, CloudBackupJobConfigUpdater, JOB_ID_SCHEMA, PRIV_CLOUD_AUDIT,
    PRIV_CLOUD_MODIFY, PROXMOX_CONFIG_DIGEST_SCHEMA,
};

use pbs_config::CachedUserInfo;

#[api(
    input: {
        properties: {},
    },
    returns: {
        description: "List configured jobs.",
        type: Array,
        items: { type: CloudBackupJobConfig },
    },
    access: {
        description: "List configured cloud jobs filtered by Cloud.Audit privileges",
        permission: &Permission::Anybody,
    },
)]
/// List all cloud backup jobs
pub fn list_cloud_backup_jobs(
    _param: Value,
    rpcenv: &mut dyn RpcEnvironment,
) -> Result<Vec<CloudBackupJobConfig>, Error> {
    let auth_id: Authid = rpcenv.get_auth_id().unwrap().parse()?;
    let user_info = CachedUserInfo::new()?;

    let (config, digest) = pbs_config::cloud_job::config()?;

    let list = config.convert_to_typed_array::<CloudBackupJobConfig>("backup")?;

    let list = list
        .into_iter()
        .filter(|job| {
            let privs = user_info.lookup_privs(&auth_id, &["cloud", "job", &job.id]);
            privs & PRIV_CLOUD_AUDIT != 0
        })
        .collect();

    rpcenv["digest"] = hex::encode(digest).into();

    Ok(list)
}

#[api(
    protected: true,
    input: {
        properties: {
            job: {
                type: CloudBackupJobConfig,
                flatten: true,
            },
        },
    },
    access: {
        permission: &Permission::Privilege(&["cloud", "job"], PRIV_CLOUD_MODIFY, false),
    },
)]
/// Create a new cloud backup job.
pub fn create_cloud_backup_job(
    job: CloudBackupJobConfig,
    _rpcenv: &mut dyn RpcEnvironment,
) -> Result<(), Error> {
    let _lock = pbs_config::cloud_job::lock()?;

    let (mut config, _digest) = pbs_config::cloud_job::config()?;

    if config.sections.get(&job.id).is_some() {
        param_bail!("id", "job '{}' already exists.", job.id);
    }

    config.set_data(&job.id, "backup", &job)?;

    pbs_config::cloud_job::save_config(&config)?;

    crate::server::jobstate::create_state_file("cloud-backup-job", &job.id)?;

    Ok(())
}

#[api(
   input: {
        properties: {
            id: {
                schema: JOB_ID_SCHEMA,
            },
        },
    },
    returns: { type: CloudBackupJobConfig },
    access: {
        permission: &Permission::Privilege(&["cloud", "job", "{id}"], PRIV_CLOUD_AUDIT, false),
    },
)]
/// Read a cloud backup job configuration.
pub fn read_cloud_backup_job(
    id: String,
    rpcenv: &mut dyn RpcEnvironment,
) -> Result<CloudBackupJobConfig, Error> {
    let (config, digest) = pbs_config::cloud_job::config()?;

    let job = config.lookup("backup", &id)?;

    rpcenv["digest"] = hex::encode(digest).into();

    Ok(job)
}

#[api()]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// Deletable property name
pub enum DeletableProperty {
    /// Delete the comment property.
    Comment,
    /// Delete the job schedule.
    Schedule,
    /// Delete the eject-media property
    EjectMedia,
    /// Delete the export-media-set property
    ExportMediaSet,
    /// Delete the 'latest-only' property
    LatestOnly,
    /// Delete the 'notify-user' property
    NotifyUser,
    /// Delete the 'group_filter' property
    GroupFilter,
    /// Delete the 'max-depth' property
    MaxDepth,
    /// Delete the 'ns' property
    Ns,
}

#[api(
    protected: true,
    input: {
        properties: {
            id: {
                schema: JOB_ID_SCHEMA,
            },
            update: {
                type: CloudBackupJobConfigUpdater,
                flatten: true,
            },
            delete: {
                description: "List of properties to delete.",
                type: Array,
                optional: true,
                items: {
                    type: DeletableProperty,
                }
            },
            digest: {
                optional: true,
                schema: PROXMOX_CONFIG_DIGEST_SCHEMA,
            },
        },
    },
    access: {
        permission: &Permission::Privilege(&["cloud", "job", "{id}"], PRIV_CLOUD_MODIFY, false),
    },
)]
/// Update the cloud backup job
pub fn update_cloud_backup_job(
    id: String,
    update: CloudBackupJobConfigUpdater,
    delete: Option<Vec<DeletableProperty>>,
    digest: Option<String>,
) -> Result<(), Error> {
    let _lock = pbs_config::cloud_job::lock()?;

    let (mut config, expected_digest) = pbs_config::cloud_job::config()?;

    let mut data: CloudBackupJobConfig = config.lookup("backup", &id)?;

    if let Some(ref digest) = digest {
        let digest = <[u8; 32]>::from_hex(digest)?;
        crate::tools::detect_modified_configuration_file(&digest, &expected_digest)?;
    }

    if let Some(delete) = delete {
        for delete_prop in delete {
            match delete_prop {
                DeletableProperty::EjectMedia => {
                    data.setup.eject_media = None;
                }
                DeletableProperty::ExportMediaSet => {
                    data.setup.export_media_set = None;
                }
                DeletableProperty::LatestOnly => {
                    data.setup.latest_only = None;
                }
                DeletableProperty::NotifyUser => {
                    data.setup.notify_user = None;
                }
                DeletableProperty::Schedule => {
                    data.schedule = None;
                }
                DeletableProperty::Comment => {
                    data.comment = None;
                }
                DeletableProperty::GroupFilter => {
                    data.setup.group_filter = None;
                }
                DeletableProperty::MaxDepth => {
                    data.setup.max_depth = None;
                }
                DeletableProperty::Ns => {
                    data.setup.ns = None;
                }
            }
        }
    }

    if let Some(store) = update.setup.store {
        data.setup.store = store;
    }
    if let Some(pool) = update.setup.pool {
        data.setup.pool = pool;
    }
    if let Some(drive) = update.setup.drive {
        data.setup.drive = drive;
    }

    if update.setup.eject_media.is_some() {
        data.setup.eject_media = update.setup.eject_media;
    };
    if update.setup.export_media_set.is_some() {
        data.setup.export_media_set = update.setup.export_media_set;
    }
    if update.setup.latest_only.is_some() {
        data.setup.latest_only = update.setup.latest_only;
    }
    if update.setup.notify_user.is_some() {
        data.setup.notify_user = update.setup.notify_user;
    }
    if update.setup.group_filter.is_some() {
        data.setup.group_filter = update.setup.group_filter;
    }
    if update.setup.ns.is_some() {
        data.setup.ns = update.setup.ns;
    }
    if update.setup.max_depth.is_some() {
        data.setup.max_depth = update.setup.max_depth;
    }

    let schedule_changed = data.schedule != update.schedule;
    if update.schedule.is_some() {
        data.schedule = update.schedule;
    }

    if let Some(comment) = update.comment {
        let comment = comment.trim();
        if comment.is_empty() {
            data.comment = None;
        } else {
            data.comment = Some(comment.to_string());
        }
    }

    config.set_data(&id, "backup", &data)?;

    pbs_config::cloud_job::save_config(&config)?;

    if schedule_changed {
        crate::server::jobstate::update_job_last_run_time("cloud-backup-job", &id)?;
    }

    Ok(())
}

#[api(
    protected: true,
    input: {
        properties: {
            id: {
                schema: JOB_ID_SCHEMA,
            },
            digest: {
                optional: true,
                schema: PROXMOX_CONFIG_DIGEST_SCHEMA,
            },
        },
    },
    access: {
        permission: &Permission::Privilege(&["cloud", "job", "{id}"], PRIV_CLOUD_MODIFY, false),
    },
)]
/// Remove a cloud backup job configuration
pub fn delete_cloud_backup_job(
    id: String,
    digest: Option<String>,
    _rpcenv: &mut dyn RpcEnvironment,
) -> Result<(), Error> {
    let _lock = pbs_config::cloud_job::lock()?;

    let (mut config, expected_digest) = pbs_config::cloud_job::config()?;

    if let Some(ref digest) = digest {
        let digest = <[u8; 32]>::from_hex(digest)?;
        crate::tools::detect_modified_configuration_file(&digest, &expected_digest)?;
    }

    match config.lookup::<CloudBackupJobConfig>("backup", &id) {
        Ok(_job) => {
            config.sections.remove(&id);
        }
        Err(_) => {
            http_bail!(NOT_FOUND, "job '{}' does not exist.", id)
        }
    };

    pbs_config::cloud_job::save_config(&config)?;

    crate::server::jobstate::remove_state_file("cloud-backup-job", &id)?;

    Ok(())
}

const ITEM_ROUTER: Router = Router::new()
    .get(&API_METHOD_READ_CLOUD_BACKUP_JOB)
    .put(&API_METHOD_UPDATE_CLOUD_BACKUP_JOB)
    .delete(&API_METHOD_DELETE_CLOUD_BACKUP_JOB);

pub const ROUTER: Router = Router::new()
    .get(&API_METHOD_LIST_CLOUD_BACKUP_JOBS)
    .post(&API_METHOD_CREATE_CLOUD_BACKUP_JOB)
    .match_all("id", &ITEM_ROUTER);
