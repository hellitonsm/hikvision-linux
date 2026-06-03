//! Log + alarm table. Two parallel ring buffers: one for SDK interface
//! calls (log table) and one for device alarms (alarm table).
//!
//! Kept in a plain `Vec` capped at MAX_ROWS so the UI stays snappy.

use chrono::Local;
use serde::Serialize;

pub const MAX_ROWS: usize = 1024;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub time: String,
    pub interface: String,
    pub device_ip: String,
    pub info: String,
    pub success: bool,
}

impl LogEntry {
    pub fn success(interface: &str, device_ip: &str, info: &str) -> Self {
        Self {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            interface: interface.to_string(),
            device_ip: device_ip.to_string(),
            info: info.to_string(),
            success: true,
        }
    }

    pub fn failure(interface: &str, device_ip: &str, info: &str) -> Self {
        Self {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            interface: interface.to_string(),
            device_ip: device_ip.to_string(),
            info: info.to_string(),
            success: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AlarmEntry {
    pub time: String,
    pub alarm_type: String,
    pub device: String,
    pub info: String,
}

impl AlarmEntry {
    pub fn from(alarm_type: &str, device_ip: &str, device_name: &str) -> Self {
        Self {
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            alarm_type: alarm_type.to_string(),
            device: format!("{}_{}", device_ip, device_name),
            info: alarm_type.to_string(),
        }
    }
}

#[derive(Default)]
pub struct LogAlarmState {
    pub logs: Vec<LogEntry>,
    pub alarms: Vec<AlarmEntry>,
    pub show_log: bool,
}

impl LogAlarmState {
    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
        if self.logs.len() > MAX_ROWS {
            let drop = self.logs.len() - MAX_ROWS;
            self.logs.drain(0..drop);
        }
    }

    pub fn add_alarm(&mut self, entry: AlarmEntry) {
        self.alarms.push(entry);
        if self.alarms.len() > MAX_ROWS {
            let drop = self.alarms.len() - MAX_ROWS;
            self.alarms.drain(0..drop);
        }
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    pub fn clear_alarms(&mut self) {
        self.alarms.clear();
    }
}
