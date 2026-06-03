//! Persistent storage for the device tree. Mirrors the on-disk format
//! used by the original Qt demo (`device_tree.txt`):
//!
//! ```text
//! <device>
//! name
//! ip
//! port
//! user
//! password
//! secret_key
//! <channel>
//! name
//! number
//! protocol
//! stream
//! </channel>
//! ...
//! </device>
//! ```

use std::fs;
use std::io;
use std::path::Path;

use super::data::{ChannelData, DeviceData, ProtocolType, StreamType};

const FILE_NAME: &str = "device_tree.txt";

pub fn load(path: &Path) -> Vec<DeviceData> {
    let data = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Vec::new(),
        Err(e) => {
            log::warn!("Could not read {}: {e}", path.display());
            return Vec::new();
        }
    };
    parse(&data)
}

pub fn save(path: &Path, devices: &[DeviceData]) -> io::Result<()> {
    let mut s = String::new();
    for dev in devices {
        s.push_str("<device>\n");
        s.push_str(&dev.name);
        s.push('\n');
        s.push_str(&dev.ip);
        s.push('\n');
        s.push_str(&dev.port.to_string());
        s.push('\n');
        s.push_str(&dev.user);
        s.push('\n');
        s.push_str(&dev.password);
        s.push('\n');
        s.push_str(&dev.secret_key);
        s.push('\n');
        for ch in &dev.channels {
            s.push_str("<channel>\n");
            s.push_str(&ch.name);
            s.push('\n');
            s.push_str(&ch.number.to_string());
            s.push('\n');
            s.push_str(&(ch.protocol as i32).to_string());
            s.push('\n');
            s.push_str(&(ch.stream as i32).to_string());
            s.push('\n');
            s.push_str("</channel>\n");
        }
        s.push_str("</device>\n");
    }
    fs::write(path, s)
}

pub fn default_path() -> std::path::PathBuf {
    Path::new(FILE_NAME).to_path_buf()
}

fn parse(text: &str) -> Vec<DeviceData> {
    let mut devices = Vec::new();
    let mut cur_device: Option<DeviceData> = None;
    let mut cur_channel: Option<ChannelData> = None;
    let mut in_channel = false;

    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        match line {
            "<device>" => {
                cur_device = Some(DeviceData::default());
                in_channel = false;
            }
            "</device>" => {
                if let Some(d) = cur_device.take() {
                    devices.push(d);
                }
                in_channel = false;
            }
            "<channel>" => {
                cur_channel = Some(ChannelData::default());
                in_channel = true;
            }
            "</channel>" => {
                if let (Some(c), Some(d)) = (cur_channel.take(), cur_device.as_mut()) {
                    d.channels.push(c);
                }
                in_channel = false;
            }
            _ => {
                if in_channel {
                    if let Some(c) = cur_channel.as_mut() {
                        apply_channel_line(c, line);
                    }
                } else if let Some(d) = cur_device.as_mut() {
                    apply_device_line(d, line);
                }
            }
        }
    }
    devices
}

fn apply_device_line(d: &mut DeviceData, line: &str) {
    match d._field_idx {
        0 => d.name = line.to_string(),
        1 => d.ip = line.to_string(),
        2 => d.port = line.parse().unwrap_or(8000),
        3 => d.user = line.to_string(),
        4 => d.password = line.to_string(),
        5 => d.secret_key = line.to_string(),
        _ => {},
    }
    d._field_idx += 1;
}

fn apply_channel_line(c: &mut ChannelData, line: &str) {
    match c._field_idx {
        0 => c.name = line.to_string(),
        1 => c.number = line.parse().unwrap_or(0),
        2 => c.protocol = num_to_protocol(line.parse().unwrap_or(0)),
        3 => c.stream = num_to_stream(line.parse().unwrap_or(0)),
        _ => {},
    }
    c._field_idx += 1;
}

fn num_to_protocol(n: i32) -> ProtocolType {
    match n {
        0 => ProtocolType::Tcp,
        1 => ProtocolType::Udp,
        2 => ProtocolType::Mcast,
        3 => ProtocolType::Rtp,
        4 => ProtocolType::Rtsp,
        5 => ProtocolType::Https,
        _ => ProtocolType::Unknown,
    }
}

fn num_to_stream(n: i32) -> StreamType {
    match n {
        0 => StreamType::Main,
        1 => StreamType::Sub,
        2 => StreamType::Third,
        3 => StreamType::Trans,
        4 => StreamType::Fourth,
        _ => StreamType::Main,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let mut dev = DeviceData::default();
        dev.name = "Cam-1".into();
        dev.ip = "10.0.0.1".into();
        dev.port = 8000;
        dev.user = "admin".into();
        dev.password = "pass12345".into();
        dev.channels.push(ChannelData {
            name: "ch1".into(),
            number: 1,
            ..Default::default()
        });
        let mut buf = std::env::temp_dir();
        buf.push("hcnetsdk_rust_test_tree.txt");
        save(&buf, &[dev.clone()]).unwrap();
        let loaded = load(&buf);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].name, "Cam-1");
        assert_eq!(loaded[0].channels[0].name, "ch1");
        let _ = fs::remove_file(buf);
    }
}
