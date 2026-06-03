//! PlayBack module — remote playback of recordings stored on the device.
//!
//! Skeleton mirroring `src/PlayBack/playback.cpp`. The actual file
//! search (`NET_DVR_FindFile_V30`) and stream download require more
//! FFI surface from the SDK; those call sites are stubbed.

use crate::public::module::{Module, ModuleGroup};

#[derive(Debug, Default)]
pub struct PlayBack;

impl PlayBack {
    pub fn new() -> Self { Self }
}

impl Module for PlayBack {
    fn id(&self) -> &'static str { "playback" }
    fn label(&self) -> &'static str { "Playback" }
    fn group(&self) -> ModuleGroup { ModuleGroup::PlayBack }
}
