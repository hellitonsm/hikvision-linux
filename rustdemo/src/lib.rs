//! HCNetSDK Qt-style demo reimplemented in Rust + Slint.
//!
//! Module layout mirrors the original C++ project under `src/`:
//!
//! * `mainwindow::devicetree`  →  device tree on the left + add-node /
//!   device-attr / channel-attr dialogs
//! * `mainwindow::logalarm`    →  log / alarm tables at the bottom
//! * `realplay`                →  preview / live stream page
//! * `playback`                →  remote playback page
//! * `para_config`             →  parameter configuration tabs
//! * `managedevice`            →  manage sub-menu (timing, update, ...)
//! * `otherfunc`               →  other sub-menu (intercom, audio, ...)
//! * `exit_module`             →  exit dialog
//! * `sdk`                     →  FFI bindings + loader + safe façade
//! * `public`                  →  shared `Module` trait

pub mod mainwindow;
pub mod para_config;
pub mod managedevice;
pub mod otherfunc;
pub mod playback;
pub mod realplay;
pub mod exit_module;
pub mod sdk;
pub mod public;
