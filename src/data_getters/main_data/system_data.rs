use serde::{Deserialize, Serialize};
use sysinfo::*;

use crate::{get_cache, main_data::cpuid_data::CPUID, CacheData};

pub struct CoreStatTemporary {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CpuCore {
    pub vendor: String,
    pub name: String,
    pub freq: u64,
    pub util: f64,
    pub threads: i32,
    pub cores: i32,
    pub temperature: f32,
    pub voltage: f64,
    pub package_power: f64,
    pub per_core_freq: Vec<u64>,
    pub cache: Vec<CacheData>,
}

impl CoreStatTemporary {
    pub fn produce_final_data(
        &self,
        voltage: f64,
        package_power: f64,
        vendor: String,
        name: String,
    ) -> (CpuCore, Memory) {
        return (
            CpuCore {
                freq: self.freq,
                util: self.util,
                threads: self.threads,
                cores: self.cores,
                temperature: self.temperature,
                per_core_freq: self.per_core_freq.clone(),
                voltage,
                package_power,
                vendor,
                name,
                cache: self.cache.clone(),
            },
            Memory {
                mem_total: self.mem_total / 1024 / 1024,
                mem_free: self.mem_free / 1024 / 1024,
                mem_used: self.mem_used / 1024 / 1024,
            },
        );
    }
}

pub fn sys_utils(time_mul: i32) -> CoreStatTemporary {
    let mut sys = System::new_all();

    sys.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(1000 / time_mul as u64));

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

    let cache = get_cache();

    CoreStatTemporary {
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
    }
}
