//! OtherFunc submenu entries.

use crate::public::module::{Module, ModuleGroup};

macro_rules! other_module {
    ($name:ident, $id:literal, $label:literal) => {
        #[derive(Debug, Default)]
        pub struct $name;
        impl $name { pub fn new() -> Self { Self } }
        impl Module for $name {
            fn id(&self) -> &'static str { $id }
            fn label(&self) -> &'static str { $label }
            fn group(&self) -> ModuleGroup { ModuleGroup::Other }
        }
    };
}

other_module!(Deployment, "deploy", "Arming / disarm");
other_module!(DevicePan, "device_pan", "Device panel simulation");
other_module!(ImportConfig, "import_export", "Import / export config");
other_module!(SerialTransfer, "serial", "Serial transparent channel");
other_module!(AudioBroadcast, "audio_broadcast", "Audio broadcast");
other_module!(AudioIntercom, "audio_intercom", "Audio intercom");
other_module!(AudioTransfer, "audio_transfer", "Audio transfer");
