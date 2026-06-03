//! Configuration (parameter) sub-windows.
//!
//! The Qt demo has 10+ parameter tabs: network, channel, user, exception,
//! serial, alarm, ATM, IP, hard-disk, etc. They all funnel through
//! `CConfigureParams::loadRemoteParams()` / `setRemoteParams()`. We
//! provide a single placeholder page that explains where each tab will
//! land.

use crate::public::module::{Module, ModuleGroup};

#[derive(Debug, Default)]
pub struct Configure;

impl Configure {
    pub fn new() -> Self { Self }

    pub fn tabs() -> &'static [&'static str] {
        &[
            "Network", "Channel", "User", "Exception", "Serial",
            "Alarm", "ATM", "IP", "Hard disk", "Public functions",
        ]
    }
}

impl Module for Configure {
    fn id(&self) -> &'static str { "configure" }
    fn label(&self) -> &'static str { "Configure" }
    fn group(&self) -> ModuleGroup { ModuleGroup::Configure }
}
