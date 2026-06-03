//! Plain-old-data types that mirror the Qt `DeviceData` / `ChannelData`
//! classes. The on-disk format (read/written in `device_tree.txt`) is
//! preserved exactly so the Rust demo and the original Qt demo can
//! share a device list file.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProtocolType {
    #[default]
    Tcp = 0,
    Udp = 1,
    Mcast = 2,
    Rtp = 3,
    Rtsp = 4,
    Https = 5,
    Unknown = 99,
}

impl ProtocolType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
            Self::Mcast => "MULTICAST",
            Self::Rtp => "RTP",
            Self::Rtsp => "RTSP",
            Self::Https => "HTTPS",
            Self::Unknown => "?",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StreamType {
    #[default]
    Main = 0,
    Sub = 1,
    Third = 2,
    Trans = 3,
    Fourth = 4,
}

impl StreamType {
    pub fn label(self) -> &'static str {
        match self {
            Self::Main => "Main stream",
            Self::Sub => "Sub stream",
            Self::Third => "Third stream",
            Self::Trans => "Trans code",
            Self::Fourth => "Fourth stream",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelData {
    pub name: String,
    pub number: i32,
    pub protocol: ProtocolType,
    pub stream: StreamType,
    pub real_handle: i32,
    pub online: bool,
    #[serde(skip)]
    pub _field_idx: u8,
}

impl Default for ChannelData {
    fn default() -> Self {
        Self {
            name: String::new(),
            number: 0,
            protocol: ProtocolType::Tcp,
            stream: StreamType::Main,
            real_handle: -1,
            online: false,
            _field_idx: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceData {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub secret_key: String,
    pub user_id: i32,
    pub multicast: String,
    pub zero_chan_num: u8,
    pub real_play_label: i32,
    pub channels: Vec<ChannelData>,
    #[serde(skip)]
    pub _field_idx: u8,
}

impl Default for DeviceData {
    fn default() -> Self {
        Self {
            name: String::new(),
            ip: String::new(),
            port: 0,          // 0 = not set yet (not 8000!)
            user: String::new(), // empty = not set
            password: String::new(),
            secret_key: String::new(),
            user_id: -1,
            multicast: String::new(),
            zero_chan_num: 0,
            real_play_label: 0,
            channels: Vec::new(),
            _field_idx: 0,
        }
    }
}

impl DeviceData {
    pub fn is_logged_in(&self) -> bool { self.user_id > 0 }
    pub fn channel_count(&self) -> usize { self.channels.len() }
    pub fn effective_port(&self) -> u16 { if self.port == 0 { 8000 } else { self.port } }
    pub fn effective_user(&self) -> &str { if self.user.is_empty() { "admin" } else { &self.user } }
}
