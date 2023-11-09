
use std::{
    fs::OpenOptions,
    io::{ Read, Seek, SeekFrom},
    path::Path,
};

use crate::TIME_MUL;


const MSR_PKG_ENERGY_STATUS: u32 = 0x611;
const MSR_POWER_UNIT: u32 = 0x606;
const MSR_VOLTAGE: u32 = 0x198;
const _MSR_TEMPERATURE_STATUS: u32 = 0x19c;
const _MSR_TEMPERATURE_TARGET: u32 = 0x1a2;
const _MSR_MISC_ENABLE: u32 = 0x1a0;

fn read_msr(core: u16, addr: u32) -> Result<u64, ()> {
    let path = format!("/dev/cpu/{}/msr", core);
    if !Path::new(&path).exists() {
        return Err(());
    }

    let mut file = OpenOptions::new().read(true).open(path).map_err(|_| ())?;

    file.seek(SeekFrom::Start(addr.into())).map_err(|_| ())?;

    let mut buff = [0u8; 8];
    file.read_exact(&mut buff).map_err(|_| ())?;

    Ok(u64::from_ne_bytes(buff))
}

pub fn get_voltage() -> f64 {
    let v = read_msr(0, MSR_VOLTAGE).unwrap();
    (v >> 32) as f64 / 8192.0
}

pub fn get_power() -> f64 {
    let energy_units_raw = read_msr(0, MSR_POWER_UNIT).unwrap();
    let energy_units = 0.5f64.powf(((energy_units_raw >> 8) & 0x1f) as f64);

    let pb = {
        let power_raw = read_msr(0, MSR_PKG_ENERGY_STATUS).unwrap();
        power_raw as f64 * energy_units
    };

    std::thread::sleep(std::time::Duration::from_millis(1000 / TIME_MUL as u64));

    let pa = {
        let power_raw = read_msr(0, MSR_PKG_ENERGY_STATUS).unwrap();
        power_raw as f64 * energy_units
    };

    (pa - pb) * TIME_MUL as f64
}
