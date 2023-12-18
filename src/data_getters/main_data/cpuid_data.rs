use lazy_static::lazy_static;
use raw_cpuid::{CpuId, CpuIdReaderNative};
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CPUID: CpuId<CpuIdReaderNative> = CpuId::new();
    pub static ref NAME_VENDOR: (String, String) = name_and_vendor();
    pub static ref CACHE_DATA: Vec<CacheData> = get_cache();
}

pub fn name_and_vendor() -> (String, String) {
    let vs = CPUID.get_vendor_info();
    let bs = CPUID.get_processor_brand_string();
    return match (vs, bs) {
        (Some(res_v), Some(res_b)) => (res_v.to_string(), res_b.as_str().to_owned()),
        _ => ("none".to_owned(), "none".to_owned()),
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheData {
    pub size: i64,
    pub level: u8,
    pub cache_type: String,
}

pub fn get_cache() -> Vec<CacheData> {
    
    match CPUID.get_cache_parameters() {
        Some(res) => res
            .map(|c| {
                let size = c.associativity()
                    * c.physical_line_partitions()
                    * c.coherency_line_size()
                    * c.sets();
                let size = size as i64;
                let level = c.level();
                let cache_type = c.cache_type().to_string();

                CacheData {
                    size,
                    level,
                    cache_type,
                }
            })
            .collect::<Vec<CacheData>>(),
        None => {
            vec![]
        }
    }
}
