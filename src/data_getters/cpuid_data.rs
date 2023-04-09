use std::ffi::{c_char, CString};

use lazy_static::lazy_static;
use raw_cpuid::{CpuId}; 

lazy_static!{
    pub static ref CPUID: CpuId = CpuId::new();
}
//static cpuid: CpuId = CpuId::new();

#[no_mangle]
pub extern "C" fn get_cpu_vendor_rs() -> *const c_char {
    //let cpuid = CpuId::new();
    let s = {
        let this = CPUID.get_vendor_info();
        match this {
            Some(val) => val.to_string(),
            None => "none".to_string(),
        }
    }
    .to_string();
    let cs = CString::new(s);
    return cs.unwrap().into_raw();
}

#[no_mangle]
pub extern "C" fn get_cpu_name_rs() -> *const c_char {
    //let cpuid = CpuId::new();
    let bs = CPUID.get_processor_brand_string();
    match bs {
        Some(res) => {
            let s = res.as_str();
            let cs = CString::new(s);
            return cs.unwrap().into_raw();
        }
        None => return CString::new("none").unwrap().into_raw(),
    }
}
