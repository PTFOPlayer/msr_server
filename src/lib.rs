#![feature(proc_macro_hygiene, decl_macro)]

pub mod data_getters;
mod misc;

pub use data_getters::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DataToJson {
    cpu: CpuCore,
    memory: Memory,
}

fn process_data(voltage: *mut f64, package_power: *mut f64, time_mul: i32) -> DataToJson {
    let (vendor, name) = non_c_name_and_vendor();
    let cs = sys_utils(time_mul);
    let (cpu, mem) = unsafe { cs.produce_final_data(*voltage, *package_power, vendor, name) };
    return DataToJson { cpu, memory: mem };
}

#[no_mangle]
pub extern "C" fn print_json_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = serde_json::to_string(&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    } else {
        println!("error serializing data")
    };
}

#[no_mangle]
pub extern "C" fn print_toml_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    if let Ok(serialized) = toml::to_string(&&process_data(voltage, package_power, time_mul)) {
        println!("{}", serialized);
    } else {
        println!("error serializing data")
    };
}

use rocket::{get, routes, State};
struct DataStruct(*mut f64, *mut f64, i32);

unsafe impl Send for DataStruct{}
unsafe impl Sync for DataStruct{}

#[get("/")]
fn full_data(data: State<DataStruct>) -> String {
    let result = process_data(data.0, data.1, data.2);
    if let Ok(serialized) = serde_json::to_string(&result) {
        return serialized;
    } else {
        return "error occured on server".to_string();
    }
}

#[get("/cpu")]
fn cpu_data(data: State<DataStruct>) -> String {
    let result = process_data(data.0, data.1, data.2).cpu;
    if let Ok(serialized) = serde_json::to_string(&result) {
        return serialized;
    } else {
        return "error occured on server".to_string();   
    }
}

#[get("/memory")]
fn memory_data(data: State<DataStruct>) -> String {
    let result = process_data(data.0, data.1, data.2).memory;
    if let Ok(serialized) = serde_json::to_string(&result) {
        return serialized;
    } else {
        return "error occured on server".to_string();   
    }
}

#[no_mangle]
pub extern "C" fn server_rs(voltage: *mut f64, package_power: *mut f64, time_mul: i32) {
    println!("{:?}", misc::module_parser::load_modules());
    rocket::ignite().mount("/", routes![full_data, cpu_data, memory_data]).manage(DataStruct(voltage, package_power, time_mul)).launch();
}
 