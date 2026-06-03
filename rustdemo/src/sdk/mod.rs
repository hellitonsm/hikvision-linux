//! Public SDK façade. Wraps the FFI loader + callbacks and offers a
//! safe(ish) Rust API that the rest of the demo uses.

pub mod hcnetsdk;
pub mod loader;
pub mod callbacks;

use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};

use hcnetsdk::*;

pub use callbacks::{exception_callback, message_callback, EventQueue, SdkEvent};
use callbacks::{exception_thunk, message_thunk_v30};

/// Global SDK initialisation. Safe to call once at startup.
pub fn init() -> Result<(), String> {
    loader::handles().map_err(|e| e.clone())?;
    unsafe {
        if NET_DVR_Init() == 0 {
            return Err(format!(
                "NET_DVR_Init failed: err {}",
                NET_DVR_GetLastError()
            ));
        }
        // Write SDK logs to ./sdkLog (matches the Qt demo's behaviour).
        let dir = std::ffi::CString::new("sdkLog").unwrap();
        NET_DVR_SetLogToFile(3, dir.as_ptr(), 0);
        NET_DVR_SetLogPrint(1);
        NET_DVR_SetConnectTime(10_000, 1);
        NET_DVR_SetExceptionCallBack_V30(0, 0, Some(exception_thunk), std::ptr::null_mut());
    }
    Ok(())
}

/// Hook the alarm listener. Must be called once after `init()`.
pub fn start_alarm_listener(_queue: EventQueue) -> Result<i32, String> {
    // The queue's tx is already installed in the static by `new_queue`.
    let ip = std::ffi::CString::new("").unwrap();
    let l_handle = unsafe {
        NET_DVR_StartListen_V30(ip.as_ptr(), 7200, Some(message_thunk_v30), std::ptr::null_mut())
    };
    if l_handle < 0 {
        Err(format!(
            "NET_DVR_StartListen_V30 failed: err {}",
            unsafe { NET_DVR_GetLastError() }
        ))
    } else {
        Ok(l_handle)
    }
}

pub fn cleanup() {
    unsafe {
        NET_DVR_Cleanup();
    }
}

pub fn sdk_version() -> String {
    let v = unsafe { NET_DVR_GetSDKBuildVersion() };
    format!(
        "HCNetSDK V{}.{}.{}.{}",
        (v >> 24) & 0xff,
        (v >> 16) & 0xff,
        (v >> 8) & 0xff,
        v & 0xff
    )
}

// ---------------------------------------------------------------------------
// Login helper.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub serial: String,
    pub by_chan_num: u8,
    pub by_ip_chan_num: u8,
    pub by_start_chan: u8,
    pub by_zero_chan_num: u8,
}

impl DeviceInfo {
    pub fn from_v40(d: &NET_DVR_DEVICEINFO_V40) -> Self {
        Self {
            serial: c_bytes_to_string(&d.struDeviceV30.sSerialNumber),
            by_chan_num: d.struDeviceV30.byChanNum,
            by_ip_chan_num: d.struDeviceV30.byIPChanNum,
            by_start_chan: d.struDeviceV30.byStartChan,
            by_zero_chan_num: d.struDeviceV30.byZeroChanNum,
        }
    }
}

pub fn login(
    ip: &str,
    port: u16,
    user: &str,
    password: &str,
    secret_key: &str,
) -> Result<(i32, DeviceInfo), String> {
    unsafe {
        let mut info: NET_DVR_USER_LOGIN_INFO = Default::default();
        let ip_bytes = ip.as_bytes();
        let n = ip_bytes.len().min(NET_DVR_DEV_ADDRESS_MAX_LEN - 1);
        info.sDeviceAddress[..n].copy_from_slice(&ip_bytes[..n]);
        info.wPort = port;
        let user_bytes = user.as_bytes();
        let n = user_bytes.len().min(NET_DVR_LOGIN_USERNAME_MAX_LEN - 1);
        info.sUserName[..n].copy_from_slice(&user_bytes[..n]);
        let pw_bytes = password.as_bytes();
        let n = pw_bytes.len().min(NET_DVR_LOGIN_PASSWD_MAX_LEN - 1);
        info.sPassword[..n].copy_from_slice(&pw_bytes[..n]);
        info.bUseAsynLogin = 0;

        let dev_v40: NET_DVR_DEVICEINFO_V40 = Default::default();
        let user_id = NET_DVR_Login_V40(&info, &dev_v40);
        if user_id < 0 {
            return Err(format!(
                "Login failed: error code {}",
                NET_DVR_GetLastError()
            ));
        }

        if !secret_key.is_empty() {
            let key = std::ffi::CString::new(secret_key).unwrap();
            NET_DVR_SetSDKSecretKey(user_id, key.as_ptr());
        }

        Ok((user_id, DeviceInfo::from_v40(&dev_v40)))
    }
}

pub fn logout(user_id: i32) -> bool {
    unsafe { NET_DVR_Logout_V30(user_id) != 0 }
}

pub fn last_error() -> u32 {
    unsafe { NET_DVR_GetLastError() }
}

// ---------------------------------------------------------------------------
// Last user-id (used by the main window to keep quick reference).
// ---------------------------------------------------------------------------
static LAST_USER_ID: AtomicI32 = AtomicI32::new(-1);

pub fn set_last_user(id: i32) {
    LAST_USER_ID.store(id, Ordering::SeqCst);
}

pub fn last_user() -> i32 {
    LAST_USER_ID.load(Ordering::SeqCst)
}

// ---------------------------------------------------------------------------
// Path helpers (preserve the on-disk layout used by the original demo).
// ---------------------------------------------------------------------------

pub fn sdk_log_dir() -> &'static Path {
    Path::new("sdkLog")
}
