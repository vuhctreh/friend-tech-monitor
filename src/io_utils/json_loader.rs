use std::collections::HashMap;
use std::io::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::io_utils::types::MonitorList;

pub fn load_monitor_list() -> HashMap<String, u64> {
    let data: String = fs::read_to_string("monitor.json").expect("ERROR: Could not read monitor.json.");

    serde_json::from_str(&*data).expect("ERROR: Could not parse JSON as not well-formatted.")
}

pub fn write_monitor_list(monitor_list: HashMap<String, u64>) -> Result<(), Error> {
    let file = File::create("monitor.json")?;

    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &monitor_list)?;

    writer.flush()?;

    Ok(())
}