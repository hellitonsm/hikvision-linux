//! ManageDevice submenu entries — these all show a dialog and call a
//! single SDK function. The skeleton provides one struct per Qt class
//! so that the wiring is one-to-one with the original code.

use crate::public::module::{Module, ModuleGroup};

macro_rules! manage_module {
    ($name:ident, $id:literal, $label:literal) => {
        #[derive(Debug, Default)]
        pub struct $name;

        impl $name {
            pub fn new() -> Self { Self }
        }

        impl Module for $name {
            fn id(&self) -> &'static str { $id }
            fn label(&self) -> &'static str { $label }
            fn group(&self) -> ModuleGroup { ModuleGroup::Manage }
        }
    };
}

manage_module!(Timing, "timing", "Remote timing");
manage_module!(RemoteUpdate, "update", "Remote update");
manage_module!(FormatDisk, "format", "Format disk");
manage_module!(RebootDevice, "reboot", "Reboot device");
manage_module!(ShutdownDevice, "shutdown", "Shutdown device");
manage_module!(LogSearch, "log_search", "Log search");
manage_module!(DeviceState, "device_state", "Device state");
manage_module!(ResetDevice, "reset", "Restore defaults");
