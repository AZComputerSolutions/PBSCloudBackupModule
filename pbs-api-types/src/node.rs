use std::ffi::OsStr;

use proxmox_schema::*;
use serde::{Deserialize, Serialize};

#[api]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
/// Cloud memory usage counters
pub struct CloudMemoryCounters {
    /// Total memory (in bytes)
    pub total: u64,
    /// Used memory (in bytes)
    pub used: u64,
    /// Free memory (in bytes)
    pub free: u64,
}

#[api]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
/// Cloud swap usage counters
pub struct CloudSwapCounters {
    /// Total swap (in bytes)
    pub total: u64,
    /// Used swap (in bytes)
    pub used: u64,
    /// Free swap (in bytes)
    pub free: u64,
}

#[api]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
/// Contains general cloud node information such as instance ID
pub struct CloudNodeInformation {
    /// The instance ID
    pub instance_id: String,
    /// The availability zone
    pub availability_zone: String,
    /// The cloud provider
    pub provider: String,
}

#[api]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
/// The current kernel version (output of `uname`)
pub struct KernelVersionInformation {
    /// The system name
    pub sysname: String,
    /// The kernel release number
    pub release: String,
    /// The kernel version
    pub version: String,
    /// The machine architecture
    pub machine: String,
}

impl KernelVersionInformation {
    pub fn from_uname_parts(
        sysname: &OsStr,
        release: &OsStr,
        version: &OsStr,
        machine: &OsStr,
    ) -> Self {
        KernelVersionInformation {
            sysname: sysname.to_str().map(String::from).unwrap_or_default(),
            release: release.to_str().map(String::from).unwrap_or_default(),
            version: version.to_str().map(String::from).unwrap_or_default(),
            machine: machine.to_str().map(String::from).unwrap_or_default(),
        }
    }

    pub fn get_legacy(&self) -> String {
        format!("{} {} {}", self.sysname, self.release, self.version)
    }
}

#[api]
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
/// Information about the CPU in a cloud environment
pub struct CloudCpuInformation {
    /// The CPU model
    pub model: String,
    /// The number of virtual CPUs (vCPUs)
    pub vcpus: usize,
}

#[api(
    properties: {
        memory: {
            type: CloudMemoryCounters,
        },
        swap: {
            type: CloudSwapCounters,
        },
        loadavg: {
            type: Array,
            items: {
                type: Number,
                description: "the load",
            }
        },
        cpuinfo: {
            type: CloudCpuInformation,
        },
        info: {
            type: CloudNodeInformation,
        }
    },
)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
/// The Cloud Node status
pub struct CloudNodeStatus {
    pub memory: CloudMemoryCounters,
    pub swap: CloudSwapCounters,
    /// The current uptime of the instance (in seconds).
    pub uptime: u64,
    /// Load for 1, 5, and 15 minutes.
    pub loadavg: [f64; 3],
    /// The current kernel version (NEW struct type).
    pub current_kernel: KernelVersionInformation,
    /// Total CPU usage since the last query.
    pub cpu: f64,
    /// The CPU information.
    pub cpuinfo: CloudCpuInformation,
    /// General instance information.
    pub info: CloudNodeInformation,
}
