pub mod data;
pub mod storage;

pub use data::{ChannelData, DeviceData, ProtocolType, StreamType};
pub use storage::{default_path, load, save};

/// One node in the visible device tree. Mirrors `TreeItem` from the Qt
/// demo: it can be a device (with children = channels) or the synthetic
/// root "Devices" node.
#[derive(Debug, Clone)]
pub enum TreeNode {
    Root,
    Device {
        device_index: usize,
        name: String,
        online: bool,
    },
    Channel {
        device_index: usize,
        channel_index: usize,
        name: String,
        online: bool,
    },
}

impl TreeNode {
    pub fn label(&self) -> String {
        match self {
            Self::Root => "Devices".to_string(),
            Self::Device { name, online, .. } => {
                let suffix = if *online { "" } else { " (offline)" };
                format!("{}{}", name, suffix)
            }
            Self::Channel { name, online, .. } => {
                let suffix = if *online { " [playing]" } else { "" };
                format!("{}{}", name, suffix)
            }
        }
    }

    pub fn device_index(&self) -> Option<usize> {
        match self {
            Self::Device { device_index, .. } | Self::Channel { device_index, .. } => {
                Some(*device_index)
            }
            _ => None,
        }
    }

    pub fn channel_index(&self) -> Option<usize> {
        match self {
            Self::Channel { channel_index, .. } => Some(*channel_index),
            _ => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Channel { .. })
    }
}

/// Build a flat depth-first view of the tree from a list of devices.
/// Used by Slint's `TreeView` model.
pub fn build_tree(devices: &[DeviceData]) -> Vec<(usize, TreeNode)> {
    let mut out: Vec<(usize, TreeNode)> = Vec::new();
    out.push((0, TreeNode::Root));
    for (di, dev) in devices.iter().enumerate() {
        out.push((
            1,
            TreeNode::Device {
                device_index: di,
                name: dev.name.clone(),
                online: dev.is_logged_in(),
            },
        ));
        for (ci, ch) in dev.channels.iter().enumerate() {
            out.push((
                2,
                TreeNode::Channel {
                    device_index: di,
                    channel_index: ci,
                    name: format!("Channel {}", ch.number),
                    online: ch.real_handle >= 0,
                },
            ));
        }
    }
    out
}
