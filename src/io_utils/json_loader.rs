use std::fs;
use crate::io_utils::types::MonitorList;

pub fn load_monitor_list() -> MonitorList {
    let data: String = fs::read_to_string("monitor.json").expect("ERROR: Could not read monitor.json.");

    serde_json::from_str(&*data).expect("ERROR: Could not parse JSON as not well-formatted.")
}