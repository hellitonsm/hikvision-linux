//! Common skeleton for every Qt-style module subwindow.
//!
//! In the original C++ code, each subfeature is a `QDialog` or `QWidget`
//! with a `.ui` file. We mirror that with a Rust trait that exposes a
//! `name()`, a `description()`, and an `invoke()` hook (the menu action).
//! Implementations can either be real (calling the SDK) or a stub that
//! just logs and pops a placeholder dialog.

use std::fmt;

pub trait Module: fmt::Debug + Send + Sync {
    fn id(&self) -> &'static str;
    fn label(&self) -> &'static str;
    fn group(&self) -> ModuleGroup;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleGroup {
    RealPlay,
    PlayBack,
    Configure,
    Manage,
    Other,
    Exit,
    DeviceTree,
}

impl ModuleGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::RealPlay => "RealPlay",
            Self::PlayBack => "PlayBack",
            Self::Configure => "Configure",
            Self::Manage => "Manage",
            Self::Other => "Other",
            Self::Exit => "Exit",
            Self::DeviceTree => "DeviceTree",
        }
    }
}
