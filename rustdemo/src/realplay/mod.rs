use std::sync::atomic::{AtomicI32, Ordering};

use crate::public::module::{Module, ModuleGroup};
use crate::sdk;
use crate::sdk::hcnetsdk::*;
pub mod x11wnd;

#[derive(Debug)]
pub struct RealPlay {
    pub user_id: i32,
    pub channel: i32,
    pub real_handle: i32,
    pub preview_active: bool,
    preview_wnd: Option<x11wnd::PreviewWindow>,
}

impl Default for RealPlay {
    fn default() -> Self {
        Self {
            user_id: -1,
            channel: -1,
            real_handle: -1,
            preview_active: false,
            preview_wnd: None,
        }
    }
}

impl RealPlay {
    pub fn new() -> Self { Self::default() }

    pub fn start(&mut self, user_id: i32, channel: i32, stream_type: u32, link_mode: u32) -> Result<(), String> {
        if user_id <= 0 { return Err("not logged in".into()); }
        if self.preview_active { self.stop(); }

        let wnd = x11wnd::PreviewWindow::new()
            .ok_or_else(|| "failed to create X11 preview window".to_string())?;
        let hwnd = wnd.window_id();

        let _ = wnd.show();

        let mut preview_info: NET_DVR_PREVIEWINFO = Default::default();
        preview_info.lChannel = channel;
        preview_info.dwStreamType = stream_type;
        preview_info.dwLinkMode = link_mode;
        preview_info.bBlocked = 1;
        preview_info.dwDisplayBufNum = 1;
        preview_info.hPlayWnd = hwnd;

        let real_handle = unsafe {
            NET_DVR_RealPlay_V40(user_id, &preview_info, std::ptr::null_mut(), std::ptr::null_mut())
        };
        if real_handle < 0 {
            let _ = wnd.hide();
            return Err(format!(
                "NET_DVR_RealPlay_V40 failed: err {}",
                unsafe { NET_DVR_GetLastError() }
            ));
        }

        self.user_id = user_id;
        self.channel = channel;
        self.real_handle = real_handle;
        self.preview_wnd = Some(wnd);
        self.preview_active = true;
        ACTIVE_REAL_HANDLE.store(real_handle, Ordering::SeqCst);
        ACTIVE_USER_ID.store(user_id, Ordering::SeqCst);
        sdk::set_last_user(user_id);
        Ok(())
    }

    pub fn stop(&mut self) {
        if self.real_handle >= 0 {
            unsafe { NET_DVR_StopRealPlay(self.real_handle); }
            self.real_handle = -1;
        }
        // Dropping the PreviewWindow destroys the X11 window.
        self.preview_wnd = None;
        self.preview_active = false;
        ACTIVE_REAL_HANDLE.store(-1, Ordering::SeqCst);
    }

    pub fn poll_window_events(&mut self) -> bool {
        if let Some(ref mut wnd) = self.preview_wnd {
            if !wnd.poll_events() {
                // User closed the window via WM — stop the preview.
                self.stop();
                return false;
            }
        }
        true
    }
}

impl Module for RealPlay {
    fn id(&self) -> &'static str { "realplay" }
    fn label(&self) -> &'static str { "Preview" }
    fn group(&self) -> ModuleGroup { ModuleGroup::RealPlay }
}

impl Drop for RealPlay {
    fn drop(&mut self) { self.stop(); }
}

pub fn active_real_handle() -> i32 { ACTIVE_REAL_HANDLE.load(Ordering::SeqCst) }

static ACTIVE_REAL_HANDLE: AtomicI32 = AtomicI32::new(-1);
static ACTIVE_USER_ID: AtomicI32 = AtomicI32::new(-1);
