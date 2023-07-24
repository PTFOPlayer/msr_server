use std::f32::NAN;

use serde::{Deserialize, Serialize};
use sysinfo::*;

use crate::cpuid_data::CPUID;

/// Deprecated
#[repr(C)]
pub struct CoreStat {
    freq: u64,
    util: f64,
    threads: i32,
    cores: i32,
    temperature: f32,
    mem_total: u64,
    mem_free: u64,
    mem_used: u64,
    per_core_freq: *mut u64,
}

#[no_mangle]
#[deprecated]
pub extern "C" fn get_sys_utils_rs(time_mul: i32) -> CoreStat {
    let mut sys = System::new_all();

    sys.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(1000 / time_mul as u64));

    sys.refresh_all();

    let t = sys.cpus().len() as i32;
    let ht = match CPUID.get_feature_info() {
        Some(val) => val.has_htt(),
        None => false,
    };

    let mut c = t;
    if ht {
        c = c / 2;
    }

    let mut s_temp = 0.;
    let mut i_i = 0;
    for i in sys.components() {
        if i.label().contains("coretemp") {
            s_temp += i.temperature();
            i_i += 1;
        }
    }

    if i_i == 0 {
        i_i = 1;
        s_temp = NAN;
    }

    let mut per_core_freq = sys
        .cpus()
        .iter()
        .map(|c| {
            c.frequency()
        })
        .collect::<Vec<u64>>();

    per_core_freq.shrink_to_fit();
    let pcf_ptr = per_core_freq.into_boxed_slice().as_mut_ptr();

    let res = CoreStat {
        freq: sys.global_cpu_info().frequency(),
        util: sys.global_cpu_info().cpu_usage() as f64,
        threads: t,
        cores: c,
        temperature: s_temp / i_i as f32,
        mem_total: sys.total_memory(),
        mem_free: sys.free_memory(),
        mem_used: sys.used_memory(),
        per_core_freq: pcf_ptr,
    };

    std::mem::forget(pcf_ptr);

    return res;
}

pub struct CoreStatRs {
    pub freq: u64,
    pub util: f64,
    pub threads: i32,
    pub cores: i32,
    pub temperature: f32,
    pub per_core_freq: Vec<u64>,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
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
}

impl CoreStatRs {
    pub fn split(
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
            },
            Memory {
                mem_total: self.mem_total / 1024 / 1024,
                mem_free: self.mem_free / 1024 / 1024,
                mem_used: self.mem_used / 1024 / 1024,
            },
        );
    }
}

pub fn non_c_sys_utils(time_mul: i32) -> CoreStatRs {
    let mut sys = System::new_all();

    sys.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(1000 / time_mul as u64));

    sys.refresh_all();

    let t = sys.cpus().len() as i32;
    let ht = match CPUID.get_feature_info() {
        Some(val) => val.has_htt(),
        None => false,
    };

    let mut c = t;
    if ht {
        c = c / 2;
    }

    let mut s_temp = 0.;
    let mut i_i = 0;
    for i in sys.components() {
        if i.label().contains("coretemp") {
            s_temp += i.temperature();
            i_i += 1;
        }
    }

    if i_i == 0 {
        i_i = 1;
        s_temp = 1.;
    }

    let temperature = s_temp / i_i as f32;

    let per_core_freq = sys
        .cpus()
        .iter()
        .map(|c| {
            c.frequency()
        })
        .collect::<Vec<u64>>();

    let res = CoreStatRs {
        freq: sys.global_cpu_info().frequency(),
        util: sys.global_cpu_info().cpu_usage() as f64,
        threads: t,
        cores: c,
        temperature,
        mem_total: sys.total_memory(),
        mem_free: sys.available_memory(),
        mem_used: sys.used_memory(),
        per_core_freq,
    };

    return res;
}
