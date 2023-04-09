use sysinfo::*;

#[repr(C)]
pub struct core_stat{
    freq: u64,
    util: f64
}

#[no_mangle]
pub extern "C" fn get_cpu_utils_rs(time_mul: i32) -> core_stat {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    std::thread::sleep(std::time::Duration::from_millis(1000 / time_mul as u64));
    
    sys.refresh_all();
    
    let res = core_stat {
        freq: sys.global_cpu_info().frequency(),
        util: sys.global_cpu_info().cpu_usage() as f64
        
    };
    
    return res
}

#[no_mangle]
pub extern "C" fn get_cpu_threads_rs() -> i32 {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    sys.cpus().len() as i32
}

#[no_mangle]
pub extern "C" fn get_cpu_temp_rs() -> f32 {
    
    let mut sys = System::new_all();
    
    sys.refresh_all();

    let mut s_temp = 0.;
    let mut i_i = 0;
    for i in sys.components(){
        if i.label().contains("coretemp") {
            s_temp += i.temperature();
            i_i += 1;
        }
    }

    if i_i == 0 {
        return -1.0;
    }
    return s_temp / i_i as f32;
    
}

#[no_mangle]
pub extern "C" fn get_mem_total() -> u64 {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    return sys.total_memory();   
}

#[no_mangle]
pub extern "C" fn get_mem_free() -> u64 {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    return sys.free_memory();   
}

#[no_mangle]
pub extern "C" fn get_mem_used() -> u64 {
    let mut sys = System::new_all();
    
    sys.refresh_all();

    return sys.used_memory();   
}