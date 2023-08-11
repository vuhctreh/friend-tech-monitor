use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MonitorList {
    pub monitor: Vec<String>,
}