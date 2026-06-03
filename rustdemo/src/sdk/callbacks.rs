//! Bridge between C-style SDK callbacks and Rust channels.
//!
//! The HCNetSDK passes raw function pointers around. We funnel those calls
//! into an `mpsc::Sender` and store the `Receiver` behind a `Mutex` so the
//! main Slint thread can poll events and update the UI.

use std::ffi::c_void;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::sdk::hcnetsdk::{
    c_bytes_to_string, fExceptionCallBack, fMessCallBackEx_V51, NET_DVR_ALARMER,
    NET_DVR_ALARMINFO, NET_DVR_ALARMINFO_V30, COMM_ALARM, COMM_ALARM_V30,
    EXCEPTION_AUDIOEXCHANGE, EXCEPTION_ALARM, EXCEPTION_ALARMRECONNECT, EXCEPTION_EXCHANGE,
    EXCEPTION_PREVIEW, EXCEPTION_RECONNECT, EXCEPTION_SERIAL, EXCEPTION_VOICETALK,
    EXCEPTION_VOICETALK_RECONNECT, PREVIEW_RECONNECTSUCCESS, ALARM_RECONNECTSUCCESS,
    VOICETALK_RECONNECTSUCCESS,
};

/// One event coming from the SDK. Polled by the main thread.
#[derive(Debug, Clone)]
pub enum SdkEvent {
    Alarm {
        kind: AlarmKind,
        device_ip: String,
        device_name: String,
    },
    Exception {
        kind: ExceptionKind,
        device_ip: String,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum AlarmKind {
    SignalInput,
    DiskFull,
    SignalLost,
    MotionDetect,
    DiskFormat,
    DiskReadWriteErr,
    NetDisconnect,
    IpConflict,
    IllegalAccess,
    VideoSignalAbnormal,
    RecordAbnormal,
    Unknown(u32),
}

impl AlarmKind {
    pub fn from_v30(t: u32) -> Self {
        match t {
            0 => Self::SignalInput,
            1 => Self::DiskFull,
            2 => Self::SignalLost,
            3 => Self::MotionDetect,
            4 => Self::DiskFormat,
            5 => Self::DiskReadWriteErr,
            6 => Self::NetDisconnect,
            7 => Self::IpConflict,
            8 => Self::IllegalAccess,
            9 => Self::VideoSignalAbnormal,
            10 => Self::RecordAbnormal,
            other => Self::Unknown(other),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::SignalInput => "Signal input alarm",
            Self::DiskFull => "Hard disk full",
            Self::SignalLost => "Signal lost",
            Self::MotionDetect => "Motion detect",
            Self::DiskFormat => "Hard disk format",
            Self::DiskReadWriteErr => "Disk R/W error",
            Self::NetDisconnect => "Network disconnect",
            Self::IpConflict => "IP conflict",
            Self::IllegalAccess => "Illegal access",
            Self::VideoSignalAbnormal => "Video signal abnormal",
            Self::RecordAbnormal => "Record abnormal",
            Self::Unknown(_) => "Unknown alarm",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExceptionKind {
    Network,
    AudioExchange,
    Alarm,
    AlarmReconnect,
    AlarmReconnectSuccess,
    Serial,
    Preview,
    PreviewReconnect,
    PreviewReconnectSuccess,
    SerialReconnect,
    SerialReconnectSuccess,
    VoiceTalk,
    VoiceTalkReconnect,
    VoiceTalkReconnectSuccess,
    Other(u32),
}

impl ExceptionKind {
    pub fn from_code(c: u32) -> Self {
        match c {
            EXCEPTION_EXCHANGE => Self::Network,
            EXCEPTION_AUDIOEXCHANGE => Self::AudioExchange,
            EXCEPTION_ALARM => Self::Alarm,
            EXCEPTION_ALARMRECONNECT => Self::AlarmReconnect,
            ALARM_RECONNECTSUCCESS => Self::AlarmReconnectSuccess,
            EXCEPTION_SERIAL => Self::Serial,
            EXCEPTION_PREVIEW => Self::Preview,
            EXCEPTION_RECONNECT => Self::PreviewReconnect,
            PREVIEW_RECONNECTSUCCESS => Self::PreviewReconnectSuccess,
            0x8009 => Self::SerialReconnect,
            0x800A => Self::SerialReconnectSuccess,
            EXCEPTION_VOICETALK => Self::VoiceTalk,
            EXCEPTION_VOICETALK_RECONNECT => Self::VoiceTalkReconnect,
            VOICETALK_RECONNECTSUCCESS => Self::VoiceTalkReconnectSuccess,
            other => Self::Other(other),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Network => "Network exception",
            Self::AudioExchange => "Audio exchange exception",
            Self::Alarm => "Alarm exception",
            Self::AlarmReconnect => "Alarm reconnecting",
            Self::AlarmReconnectSuccess => "Alarm reconnect success",
            Self::Serial => "Serial exception",
            Self::Preview => "Preview exception",
            Self::PreviewReconnect => "Preview reconnecting",
            Self::PreviewReconnectSuccess => "Preview reconnect success",
            Self::SerialReconnect => "Serial reconnecting",
            Self::SerialReconnectSuccess => "Serial reconnect success",
            Self::VoiceTalk => "Voice talk exception",
            Self::VoiceTalkReconnect => "Voice talk reconnecting",
            Self::VoiceTalkReconnectSuccess => "Voice talk reconnect success",
            Self::Other(_) => "Unknown exception",
        }
    }
}

/// Holder that hands out `&Receiver` via a `MutexGuard`. The receiver
/// itself isn't `Clone`, so we keep a single instance behind a mutex and
/// let the UI thread borrow it briefly during each tick.
struct RxHolder {
    rx: parking_lot::Mutex<Receiver<SdkEvent>>,
}

static HOLDER: once_cell::sync::OnceCell<RxHolder> = once_cell::sync::OnceCell::new();
static TX: once_cell::sync::OnceCell<Sender<SdkEvent>> = once_cell::sync::OnceCell::new();

/// Create a fresh queue and install it globally. The returned struct is
/// mainly for ergonomics — the `Receiver` is also reachable via
/// `receiver()`.
pub struct EventQueue {
    pub tx: Sender<SdkEvent>,
}

pub fn new_queue() -> EventQueue {
    let (tx, rx) = channel();
    let _ = HOLDER.set(RxHolder { rx: parking_lot::Mutex::new(rx) });
    let _ = TX.set(tx.clone());
    EventQueue { tx }
}

pub fn sender() -> Option<Sender<SdkEvent>> {
    TX.get().cloned()
}

pub fn receiver() -> Option<parking_lot::MutexGuard<'static, Receiver<SdkEvent>>> {
    HOLDER.get().map(|h| h.rx.lock())
}

pub fn exception_callback() -> fExceptionCallBack {
    Some(exception_thunk)
}

pub fn message_callback() -> fMessCallBackEx_V51 {
    Some(message_thunk_v30)
}

// ---------------------------------------------------------------------------
// C-callable thunks. They push the decoded event into the global queue.
// ---------------------------------------------------------------------------

pub unsafe extern "C" fn exception_thunk(
    dw_type: u32,
    _l_user_id: i32,
    _l_handle: i32,
    _p_user: *mut c_void,
) {
    let kind = ExceptionKind::from_code(dw_type);
    if let Some(tx) = sender() {
        let _ = tx.send(SdkEvent::Exception {
            kind,
            device_ip: String::new(),
        });
    }
}

pub unsafe extern "C" fn message_thunk_v30(
    l_command: i32,
    p_alarmer: *mut NET_DVR_ALARMER,
    p_alarm_info: *mut std::os::raw::c_char,
    _dw_buf_len: u32,
    _p_user: *mut c_void,
) {
    let device_ip = if !p_alarmer.is_null() {
        c_bytes_to_string(&(*p_alarmer).sDeviceIP)
    } else {
        String::new()
    };
    let device_name = if !p_alarmer.is_null() {
        c_bytes_to_string(&(*p_alarmer).sDeviceName)
    } else {
        String::new()
    };

    if l_command == COMM_ALARM || l_command == COMM_ALARM_V30 {
        if p_alarm_info.is_null() {
            return;
        }
        let alarm_type = if l_command == COMM_ALARM_V30 {
            (*(p_alarm_info as *const NET_DVR_ALARMINFO_V30)).dwAlarmType
        } else {
            (*(p_alarm_info as *const NET_DVR_ALARMINFO)).dwAlarmType
        };
        if let Some(tx) = sender() {
            let _ = tx.send(SdkEvent::Alarm {
                kind: AlarmKind::from_v30(alarm_type),
                device_ip,
                device_name,
            });
        }
    } else if l_command as u32 >= EXCEPTION_EXCHANGE {
        if let Some(tx) = sender() {
            let _ = tx.send(SdkEvent::Exception {
                kind: ExceptionKind::from_code(l_command as u32),
                device_ip,
            });
        }
    }
}
