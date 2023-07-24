pub mod data_getters;
pub use data_getters::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataToJson {
    cpu: CpuCore,
    memory: Memory,
}

#[no_mangle]
pub extern "C" fn print_json_rs(voltage: f64, package_power: f64, time_mul: i32) {

    let (vendor, name) = non_c_name_and_vendor();

    let cs = non_c_sys_utils(time_mul);

    let (cpu, mem) = cs.split(voltage, package_power, vendor, name);

    if let Ok(serialized) = serde_json::to_string(&DataToJson {
        cpu,
        memory: mem
    }) {
        println!("{}", serialized);
    };
}