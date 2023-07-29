use lazy_static::lazy_static;
use raw_cpuid::CpuId;

lazy_static! {
    pub static ref CPUID: CpuId = CpuId::new();
}

pub fn name_and_vendor() -> (String, String) {
    let vs = CPUID.get_vendor_info();
    let bs = CPUID.get_processor_brand_string();
    return match (vs, bs) {
        (Some(res_v), Some(res_b)) => (res_v.to_string(), res_b.as_str().to_owned()),
        _ => ("none".to_owned(), "none".to_owned()),
    };
}
