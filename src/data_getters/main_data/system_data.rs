use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sysinfo::*;

use crate::{main_data::cpuid_data::CPUID, CacheData, CACHE_DATA, NAME_VENDOR, TIME_MUL};

lazy_static! {
    pub static ref CORE_STAT: CoreStat = sys_utils();
    pub static ref SYS: Arc<Mutex<System>> = Arc::new(Mutex::new(System::new_all()));
    pub static ref COMP: Arc<Mutex<Components>> =
        Arc::new(Mutex::new(Components::new_with_refreshed_list()));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoreStat {
    pub voltage: f64,
    pub package_power: f64,
    pub vendor: String,
    pub name: String,
    pub util: f64,
    pub threads: i32,
    pub cores: i32,
    pub temperature: f32,
    pub per_core_freq: Vec<u64>,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub cache: Vec<CacheData>,
}

impl CoreStat {
    pub fn update(mut self, voltage: f64, package_power: f64) -> Result<Self, String> {
        self.voltage = voltage;
        self.package_power = package_power;

        let mut sys = match SYS.lock() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };
        sys.refresh_cpu();
        sys.refresh_memory();

        let mut comp = match COMP.lock() {
            Ok(res) => res,
            Err(err) => return Err(err.to_string()),
        };

        comp.refresh();
        let temp = comp.into_iter().find(|x| x.label().contains("coretemp"));

        self.temperature = match temp {
            Some(res) => res.temperature(),
            None => 0.,
        };

        self.mem_free = sys.available_memory();
        self.mem_used = sys.used_memory();

        self.per_core_freq = sys
            .cpus()
            .iter()
            .map(|c| c.frequency())
            .collect::<Vec<u64>>();
        self.per_core_freq.shrink_to_fit();

        let gci = sys.global_cpu_info();
        self.util = gci.cpu_usage() as f64;

        Ok(self)
    }
}

pub fn sys_utils() -> CoreStat {
    let mut sys = System::new_all();

    std::thread::sleep(std::time::Duration::from_millis(1000 / TIME_MUL as u64));

    sys.refresh_all();

    let t = sys.cpus().len() as i32;
    let ht = match CPUID.get_feature_info() {
        Some(val) => val.has_htt(),
        None => false,
    };

    let c = match sys.physical_core_count() {
        Some(res) => res as i32,
        None => {
            if ht {
                t / 2
            } else {
                t
            }
        }
    };

    let mut comp = COMP.lock().unwrap();
    comp.refresh();
    let temp = comp.into_iter().find(|x| x.label().contains("coretemp"));

    let temperature = match temp {
        Some(res) => res.temperature(),
        None => 0.,
    };

    let per_core_freq = sys
        .cpus()
        .iter()
        .map(|c| c.frequency())
        .collect::<Vec<u64>>();

    let cache = CACHE_DATA.clone();
    CoreStat {
        util: sys.global_cpu_info().cpu_usage() as f64,
        threads: t,
        cores: c,
        temperature,
        mem_total: sys.total_memory(),
        mem_free: sys.available_memory(),
        mem_used: sys.used_memory(),
        per_core_freq,
        cache,
        vendor: NAME_VENDOR.0.clone(),
        name: NAME_VENDOR.1.clone(),
        voltage: 0.0,
        package_power: 0.0,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Drive {
    name: String,
    mount_point: String,
    file_sys: String,
    kind: String,
    removable: bool,
    space: u64,
    available_space: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Drives {
    drives: Vec<Drive>,
}

pub fn get_drives() -> Drives {
    let disks = Disks::new_with_refreshed_list();

    let mut v = vec![];
    for d in disks.into_iter() {
        v.push(Drive {
            name: d.name().to_string_lossy().into(),
            mount_point: d.mount_point().to_string_lossy().into(),
            file_sys: d.file_system().to_string_lossy().into(),
            kind: format!("{:?}", d.kind()),
            removable: d.is_removable(),
            space: d.total_space(),
            available_space: d.available_space(),
        });
    }

    Drives { drives: v }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    host_name: String,
    boot_time: u64,
    distro_id: String,
    kernel_version: String,
    os_version: String,
}

pub fn get_system() -> SystemInfo {
    let default = "".to_owned();

    SystemInfo {
        host_name: System::name().map_or(default.clone(), |res| res),
        boot_time: System::boot_time(),
        distro_id: System::distribution_id(),
        kernel_version: System::kernel_version().map_or(default.clone(), |res| res),
        os_version: System::long_os_version().map_or(default, |res| res),
    }
}
