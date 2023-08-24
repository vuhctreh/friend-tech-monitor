//! I/O operations for JSON.

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use eyre::{eyre, Result};

/// Read from monitor.json file.
pub fn load_monitor_list() -> Result<HashMap<String, u64>> {
    let data: String = fs::read_to_string("monitor.json")?;

    let map = serde_json::from_str(&*data);

    match map {
        Ok(m) => Ok(m),
        Err(e) => Err(eyre!(e))
    }
}

/// Write to monitor.json file.
pub fn write_monitor_list(monitor_list: HashMap<String, u64>) -> Result<()> {
    let file = File::create("monitor.json")?;

    let mut writer = BufWriter::new(file);

    serde_json::to_writer(&mut writer, &monitor_list)?;

    writer.flush()?;

    Ok(())
}