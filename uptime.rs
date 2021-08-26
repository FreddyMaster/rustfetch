use std::{env, io};
use sysinfo::{RefreshKind, System, SystemExt};

pub fn duration(uptime: u64) -> (String, String, String) {
    let sys = System::new_with_specifics(RefreshKind::new().with_cpu().with_memory());
    let uptime = sys.uptime();
    let days = if uptime > 86400 {
        let days_pre = uptime / 60 / 60 / 24;
        days_pre.to_string() + "d"
    } else {
        "".to_string()
    };
    let hours = if uptime > 3600 {
        let hours_pre = (uptime / 60 / 60) % 24;
        hours_pre.to_string() + "h"
    } else {
        "".to_string()
    };
    let minutes = if uptime > 60 {
        let minutes_pre = (uptime / 60) % 60;
        minutes_pre.to_string() + "m"
    } else {
        "".to_string()
    };
    (days, hours, minutes)
}
