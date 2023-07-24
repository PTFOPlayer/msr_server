use sysinfo::*;

use crate::cpuid_data::CPUID;

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
        s_temp = 1.;
    }

    let per_core_freq = sys
        .cpus()
        .iter()
        .map(|c| c.frequency())
        .collect::<Vec<u64>>();

    let res = CoreStat {
        freq: sys.global_cpu_info().frequency(),
        util: sys.global_cpu_info().cpu_usage() as f64,
        threads: t,
        cores: c,
        temperature: s_temp / i_i as f32,
        mem_total: sys.total_memory(),
        mem_free: sys.free_memory(),
        mem_used: sys.used_memory(),
        per_core_freq: per_core_freq.into_boxed_slice().as_mut_ptr(),
    };

    return res;
}
