pub mod data_getters;

pub use data_getters::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataToJson {
    cpu: CpuCore,
    memory: Memory,
}

fn process_data(voltage: *mut f64, package_power: *mut f64, time_mul: i32) -> DataToJson {
    let (vendor, name) = non_c_name_and_vendor();
    let cs = non_c_sys_utils(time_mul);
    let (cpu, mem) = unsafe { cs.split(*voltage, *package_power, vendor, name) };
    return DataToJson { cpu, memory: mem };
}

#[no_mangle]
pub extern "C" fn print_json_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = serde_json::to_string(&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    };
}

#[no_mangle]
pub extern "C" fn print_toml_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = toml::to_string(&&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    };
}

#[no_mangle]
pub extern "C" fn toml_to_file_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    use std::io::Write;
    loop {
        if let Ok(serialized) = toml::to_string(&&process_data(voltage, package_power, time_mul)) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .append(false)
                .truncate(true)
                .open("/msr_data.toml")
                .unwrap();
            _ = file.write(serialized.as_bytes());
            file.flush().unwrap();
        }
    }
}
