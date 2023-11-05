use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sysinfo::*;

use crate::{main_data::cpuid_data::CPUID, CacheData, CACHE_DATA, NAME_VENDOR, TIME_MUL};

lazy_static! {
    pub static ref CORE_STAT: CoreStat = sys_utils();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoreStat {
    pub voltage: f64,
    pub package_power: f64,
    pub vendor: String,
    pub name: String,
    pub freq: u64,
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
    pub fn update(mut self, voltage: f64, package_power: f64) -> Self {
        self.voltage = voltage;
        self.package_power = package_power;

        let mut sys = System::new_all();

        sys.refresh_all();

        std::thread::sleep(std::time::Duration::from_millis(1000 / TIME_MUL as u64));

        sys.refresh_all();

        let comp = sys
            .components()
            .into_iter()
            .filter(|x| x.label().contains("coretemp"));

        let i_i = comp.clone().count();
        let s_temp = comp.map(|x| x.temperature()).sum::<f32>();

        self.temperature = s_temp / i_i as f32;

        self.mem_free = sys.available_memory();
        self.mem_used = sys.used_memory();

        self.per_core_freq = sys
            .cpus()
            .iter()
            .map(|c| c.frequency())
            .collect::<Vec<u64>>();

        let gci = sys.global_cpu_info();

        self.freq = gci.frequency();
        self.util = gci.cpu_usage() as f64;

        self
    }
}

pub fn sys_utils() -> CoreStat {
    let mut sys = System::new_all();

    sys.refresh_all();

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

    let comp = sys
        .components()
        .into_iter()
        .filter(|x| x.label().contains("coretemp"));

    let i_i = comp.clone().count();
    let s_temp = comp.map(|x| x.temperature()).sum::<f32>();

    let temperature = s_temp / i_i as f32;

    let per_core_freq = sys
        .cpus()
        .iter()
        .map(|c| c.frequency())
        .collect::<Vec<u64>>();

    let cache = CACHE_DATA.clone();
    CoreStat {
        freq: sys.global_cpu_info().frequency(),
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
