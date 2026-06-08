use std::sync::atomic::{AtomicI32, Ordering};

use crate::public::module::{Module, ModuleGroup};
use crate::sdk;
use crate::sdk::hcnetsdk::*;
pub mod x11wnd;

/// Uma sessão individual de preview (uma câmera = uma janela X11 = um RealPlay).
#[derive(Debug)]
pub struct RealPlaySession {
    pub user_id: i32,
    pub channel: i32,
    pub real_handle: i32,
    pub preview_active: bool,
    preview_wnd: Option<x11wnd::PreviewWindow>,
}

impl RealPlaySession {
    pub fn new(
        user_id: i32,
        channel: i32,
        stream_type: u32,
        link_mode: u32,
    ) -> Result<Self, String> {
        if user_id <= 0 {
            return Err("not logged in".into());
        }

        let wnd = x11wnd::PreviewWindow::new()
            .ok_or_else(|| "failed to create X11 preview window".to_string())?;
        let hwnd = wnd.window_id();

        let _ = wnd.show();

        let mut preview_info: NET_DVR_PREVIEWINFO = Default::default();
        preview_info.lChannel = channel; // Canal 0 = Canal Zero
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

        sdk::set_last_user(user_id);

        Ok(Self {
            user_id,
            channel,
            real_handle,
            preview_active: true,
            preview_wnd: Some(wnd),
        })
    }

    pub fn stop(&mut self) {
        if self.real_handle >= 0 {
            unsafe { NET_DVR_StopRealPlay(self.real_handle); }
            self.real_handle = -1;
        }
        self.preview_wnd = None;
        self.preview_active = false;
    }

    /// Processa eventos X11 da janela. Retorna false se o usuário fechou a janela.
    pub fn poll_window_events(&mut self) -> bool {
        if let Some(ref mut wnd) = self.preview_wnd {
            if !wnd.poll_events() {
                self.stop();
                return false;
            }
        }
        true
    }
}

impl Drop for RealPlaySession {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Gerenciador de múltiplas sessões de preview.
///
/// Cada câmera aberta cria uma `RealPlaySession` independente.
/// Fechar uma janela X11 não afeta as outras.
pub struct RealPlay {
    sessions: Vec<RealPlaySession>,
}

impl Default for RealPlay {
    fn default() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }
}

impl RealPlay {
    pub fn new() -> Self { Self::default() }

    /// Inicia preview de um canal. Adiciona uma nova sessão sem fechar as existentes.
    ///
    /// `channel = 0` ativa o **Canal Zero** (stream multiplexado).
    pub fn start(&mut self, user_id: i32, channel: i32, stream_type: u32, link_mode: u32) -> Result<(), String> {
        // Verifica se já existe sessão para este user_id+channel
        if let Some(existing) = self.sessions.iter_mut().find(|s| s.user_id == user_id && s.channel == channel) {
            existing.stop();
        }

        let session = RealPlaySession::new(user_id, channel, stream_type, link_mode)?;
        let handle = session.real_handle;
        self.sessions.push(session);

        ACTIVE_REAL_HANDLE.store(handle, Ordering::SeqCst);
        ACTIVE_USER_ID.store(user_id, Ordering::SeqCst);

        Ok(())
    }

    /// Para o preview de um canal específico. Remove a sessão correspondente.
    pub fn stop_channel(&mut self, user_id: i32, channel: i32) {
        if let Some(pos) = self.sessions.iter().position(|s| s.user_id == user_id && s.channel == channel) {
            self.sessions.remove(pos);
        }
        if self.sessions.is_empty() {
            ACTIVE_REAL_HANDLE.store(-1, Ordering::SeqCst);
        }
    }

    /// Para todas as sessões.
    pub fn stop_all(&mut self) {
        self.sessions.clear();
        ACTIVE_REAL_HANDLE.store(-1, Ordering::SeqCst);
    }

    /// Processa eventos X11 de todas as janelas. Remove sessões cuja janela foi fechada.
    pub fn poll_window_events(&mut self) {
        // Marcar sessões mortas (janela fechada pelo WM)
        let mut i = 0;
        while i < self.sessions.len() {
            if !self.sessions[i].poll_window_events() {
                self.sessions.remove(i);
            } else {
                i += 1;
            }
        }
        if self.sessions.is_empty() {
            ACTIVE_REAL_HANDLE.store(-1, Ordering::SeqCst);
        }
    }

    /// Número de sessões ativas.
    pub fn active_count(&self) -> usize {
        self.sessions.len()
    }

    /// Verdadeiro se existe ao menos uma sessão ativa.
    pub fn preview_active(&self) -> bool {
        !self.sessions.is_empty()
    }

    /// Retorna true se o canal dado já tem uma sessão ativa.
    pub fn is_channel_active(&self, user_id: i32, channel: i32) -> bool {
        self.sessions.iter().any(|s| s.user_id == user_id && s.channel == channel && s.preview_active)
    }

    /// Itera sobre as sessões ativas (para atualizar tree, logs, etc).
    pub fn sessions(&self) -> &[RealPlaySession] {
        &self.sessions
    }
}

impl Module for RealPlay {
    fn id(&self) -> &'static str { "realplay" }
    fn label(&self) -> &'static str { "Preview" }
    fn group(&self) -> ModuleGroup { ModuleGroup::RealPlay }
}

impl Drop for RealPlay {
    fn drop(&mut self) { self.stop_all(); }
}

pub fn active_real_handle() -> i32 { ACTIVE_REAL_HANDLE.load(Ordering::SeqCst) }

static ACTIVE_REAL_HANDLE: AtomicI32 = AtomicI32::new(-1);
static ACTIVE_USER_ID: AtomicI32 = AtomicI32::new(-1);
