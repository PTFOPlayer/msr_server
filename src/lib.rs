#![feature(proc_macro_hygiene, decl_macro)]

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
            drop(file);
        }
    }
}
use rocket::{get, routes, State};
struct DataStruct(*mut f64, *mut f64, i32);

unsafe impl Send for DataStruct{}
unsafe impl Sync for DataStruct{}

#[get("/")]
fn default_path<'a>(data: State<DataStruct>) -> String {
    let result = process_data(data.0, data.1, data.2);
    if let Ok(serialized) = serde_json::to_string(&result) {
        return serialized;
    } else {
        return "".to_string();
    }

}

#[no_mangle]
pub extern "C" fn server_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    rocket::ignite().mount("/", routes![default_path]).manage(DataStruct(voltage, package_power, time_mul)).launch();
}
 