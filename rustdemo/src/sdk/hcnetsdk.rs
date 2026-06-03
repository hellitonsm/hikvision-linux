//! FFI bindings to Hikvision's HCNetSDK (`libhcnetsdk`).
//!
//! These are *focused* bindings: only the symbols the original Qt demo
//! actually uses are declared. Full `bindgen` of `HCNetSDK.h` would
//! generate tens of thousands of lines, most of which we never touch.
//!
//! Struct layouts here match the C definitions in `includeCn/HCNetSDK.h`
//! (Linux 64-bit build). All `BOOL` in the C header becomes `c_int` here
//! (per the Linux definition in HCNetSDK.h itself).

#![allow(non_snake_case, non_camel_case_types, dead_code, clippy::all)]

use libc::{c_char, c_int, c_uint, c_uchar, c_void};

// ---------------------------------------------------------------------------
// Primitive aliases matching the Linux HCNetSDK.h definitions.
// ---------------------------------------------------------------------------
pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type WORD = u16;
pub type DWORD = c_uint;
pub type LONG = c_int;
pub type UINT = c_uint;
pub type LPVOID = *mut c_void;
pub type HANDLE = LPVOID;
pub type LPDWORD = *mut DWORD;
// On Linux the Hikvision SDK uses `unsigned int` for HWND (4 bytes),
// NOT a pointer.  We must match that layout exactly.
#[cfg(target_os = "linux")]
pub type HWND = u32;
#[cfg(not(target_os = "linux"))]
pub type HWND = *mut c_void;

// ---------------------------------------------------------------------------
// Constants used by the demo.
// ---------------------------------------------------------------------------
pub const BOOL_TRUE: BOOL = 1;
pub const BOOL_FALSE: BOOL = 0;
pub const NET_DVR_NOERROR: c_int = 0;

// Return values
pub const NET_DVR_USERID_INVALID: LONG = -1;
pub const NET_DVR_CHANNEL_ERROR: c_int = 4;
pub const NET_DVR_NETWORK_FAIL_CONNECT: c_int = 7;
pub const NET_DVR_LOGIN_ERROR_PASSWORD: c_int = 1;
pub const NET_DVR_PASSWORD_FORMAT_ERROR: c_int = 26;
pub const NET_DVR_ERROR_DEVICE_NOT_ACTIVATED: c_int = 250;

// Alarm / exception command codes
pub const COMM_ALARM: c_int = 0x1100;
pub const COMM_ALARM_V30: c_int = 0x4000;
pub const COMM_TRADEINFO: c_int = 0x1500;
pub const COMM_IPCCFG: c_int = 0x4002;

pub const EXCEPTION_EXCHANGE: c_uint = 0x8000;
pub const EXCEPTION_AUDIOEXCHANGE: c_uint = 0x8001;
pub const EXCEPTION_ALARM: c_uint = 0x8002;
pub const EXCEPTION_ALARMRECONNECT: c_uint = 0x8003;
pub const ALARM_RECONNECTSUCCESS: c_uint = 0x8004;
pub const EXCEPTION_SERIAL: c_uint = 0x8005;
pub const EXCEPTION_PREVIEW: c_uint = 0x8006;
pub const EXCEPTION_RECONNECT: c_uint = 0x8007;
pub const PREVIEW_RECONNECTSUCCESS: c_uint = 0x8008;
pub const EXCEPTION_SERIAL_RECONNECT: c_uint = 0x8009;
pub const SERIAL_RECONNECTSUCCESS: c_uint = 0x800A;
pub const EXCEPTION_VOICETALK: c_uint = 0x800B;
pub const EXCEPTION_VOICETALK_RECONNECT: c_uint = 0x800C;
pub const VOICETALK_RECONNECTSUCCESS: c_uint = 0x800D;

// String length constants
pub const NAME_LEN: usize = 32;
pub const SERIALNO_LEN: usize = 48;
pub const MACADDR_LEN: usize = 6;
pub const NET_DVR_DEV_ADDRESS_MAX_LEN: usize = 129;
pub const NET_DVR_LOGIN_USERNAME_MAX_LEN: usize = 64;
pub const NET_DVR_LOGIN_PASSWD_MAX_LEN: usize = 64;
pub const NET_DVR_SECRET_KEY_LEN: usize = 128;

// ---------------------------------------------------------------------------
// Structures used by the demo. Layout must match the C definitions exactly.
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct NET_DVR_ALARMINFO {
    pub dwAlarmType: DWORD,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_ALARMINFO_V30 {
    pub dwAlarmType: DWORD,
    pub dwAlarmInputNumber: DWORD,
    pub byAlarmOutputNumber: [BYTE; 96],
    pub byAlarmRelatedVideo: [BYTE; 16],
    pub byChannel: [BYTE; 16],
    pub byDiskNumber: [BYTE; 16],
}
impl Default for NET_DVR_ALARMINFO_V30 {
    fn default() -> Self { unsafe { std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_ALARMER {
    pub byUserIDValid: BYTE,
    pub bySerialValid: BYTE,
    pub byVersionValid: BYTE,
    pub byDeviceNameValid: BYTE,
    pub byMacAddrValid: BYTE,
    pub byLinkPortValid: BYTE,
    pub byDeviceIPValid: BYTE,
    pub bySocketIPValid: BYTE,
    pub lUserID: LONG,
    pub sSerialNumber: [BYTE; SERIALNO_LEN],
    pub dwDeviceVersion: DWORD,
    pub sDeviceName: [BYTE; NAME_LEN],
    pub byMacAddr: [BYTE; MACADDR_LEN],
    pub wLinkPort: WORD,
    pub sDeviceIP: [BYTE; 128],
    pub sSocketIP: [BYTE; 128],
    pub byIpProtocol: BYTE,
    pub byIPv6Valid: [BYTE; 129],
    pub byRes2: [BYTE; 255],
}
impl Default for NET_DVR_ALARMER {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_LOCAL_IP_CHAN_INFO {
    pub sIP: [BYTE; 16],
    pub wDVRPort: WORD,
}
impl Default for NET_DVR_LOCAL_IP_CHAN_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_IPCHANINFO {
    pub byEnable: BYTE,
    pub byIPID: BYTE,
    pub byRes1: [BYTE; 2],
    pub sIP: [BYTE; 16],
    pub wDVRPort: WORD,
    pub sIPEx: [BYTE; 32],
    pub byRes: [BYTE; 2],
}
impl Default for NET_DVR_IPCHANINFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_DEVICEINFO_V30 {
    pub sSerialNumber: [BYTE; SERIALNO_LEN],
    pub byAlarmInPortNum: BYTE,
    pub byAlarmOutPortNum: BYTE,
    pub byDiskNum: BYTE,
    pub byDVRType: BYTE,
    pub byChanNum: BYTE,
    pub byStartChan: BYTE,
    pub byAudioChanNum: BYTE,
    pub byIPChanNum: BYTE,
    pub byZeroChanNum: BYTE,
    pub byMainProto: BYTE,
    pub bySubProto: BYTE,
    pub bySupport: BYTE,
    pub bySupport1: BYTE,
    pub bySupport2: BYTE,
    pub wDevType: WORD,
    pub bySupport3: BYTE,
    pub byMultiStreamProto: BYTE,
    pub byStartDChan: BYTE,
    pub byStartDTalkChan: BYTE,
    pub byHighDChanNum: BYTE,
    pub bySupport4: BYTE,
    pub byLanguageType: BYTE,
    pub byVoiceInChanNum: BYTE,
    pub byStartVoiceInChanNo: BYTE,
    pub byRes3: [BYTE; 2],
    pub byMirrorChanNum: BYTE,
    pub wStartMirrorChanNo: WORD,
    pub byRes4: [BYTE; 2],
    pub byZeroSubChanNum: BYTE,
    pub byRes: [BYTE; 183],
}
impl Default for NET_DVR_DEVICEINFO_V30 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_DEVICEINFO_V40 {
    pub struDeviceV30: NET_DVR_DEVICEINFO_V30,
    pub bySupportLock: BYTE,
    pub byRetryLoginTime: BYTE,
    pub byPasswordLen: BYTE,
    pub byRes1: BYTE,
    pub wLockTime: WORD,
    pub byRes2: [BYTE; 128],
}
impl Default for NET_DVR_DEVICEINFO_V40 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_USER_LOGIN_INFO {
    pub sDeviceAddress: [BYTE; NET_DVR_DEV_ADDRESS_MAX_LEN],
    pub byUseTransport: BYTE,
    pub wPort: WORD,
    pub sUserName: [BYTE; NET_DVR_LOGIN_USERNAME_MAX_LEN],
    pub sPassword: [BYTE; NET_DVR_LOGIN_PASSWD_MAX_LEN],
    pub byLoginMode: BYTE,
    pub byHttps: BYTE,
    pub byDeviceType: BYTE,
    pub byLoginClientType: BYTE,
    pub byProxyType: BYTE,
    pub sUserIP: [BYTE; 129],
    pub szDomain: [c_char; 256],
    pub bUseAsynLogin: BOOL,
    pub byRes2: [BYTE; 125],
}
impl Default for NET_DVR_USER_LOGIN_INFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NET_DVR_PREVIEWINFO {
    pub lChannel: LONG,
    pub dwStreamType: DWORD,
    pub dwLinkMode: DWORD,
    pub hPlayWnd: HWND,
    pub bBlocked: BOOL,
    pub bPassbackRecord: BOOL,
    pub byPreviewMode: BYTE,
    pub byStreamID: [BYTE; 32],
    pub byProtoType: BYTE,
    pub byRes1: BYTE,
    pub byVideoCodingType: BYTE,
    pub dwDisplayBufNum: DWORD,
    pub byNPQMode: BYTE,
    pub byRes: [BYTE; 215],
}
impl Default for NET_DVR_PREVIEWINFO {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// ---------------------------------------------------------------------------
// Callback function pointer types.
// ---------------------------------------------------------------------------

pub type fExceptionCallBack =
    Option<unsafe extern "C" fn(dwType: DWORD, lUserID: LONG, lHandle: LONG, pUser: *mut c_void)>;

pub type fMessCallBack_V30 = Option<
    unsafe extern "C" fn(
        lCommand: LONG,
        pAlarmer: *mut NET_DVR_ALARMER,
        pAlarmInfo: *mut c_char,
        dwBufLen: DWORD,
        pUser: *mut c_void,
    ),
>;

pub type fMessCallBackEx_V51 = Option<
    unsafe extern "C" fn(
        lCommand: LONG,
        pAlarmer: *mut NET_DVR_ALARMER,
        pAlarmInfo: *mut c_char,
        dwBufLen: DWORD,
        pUser: *mut c_void,
    ),
>;

// ---------------------------------------------------------------------------
// Extern "C" declarations. Loaded via libloading; see sdk/loader.rs.
// ---------------------------------------------------------------------------

extern "C" {
    pub fn NET_DVR_Init() -> BOOL;
    pub fn NET_DVR_Cleanup();
    pub fn NET_DVR_Login_V40(
        pLoginInfo: *const NET_DVR_USER_LOGIN_INFO,
        pDeviceInfo: *const NET_DVR_DEVICEINFO_V40,
    ) -> LONG;
    pub fn NET_DVR_Logout_V30(lUserID: LONG) -> BOOL;
    pub fn NET_DVR_GetLastError() -> DWORD;
    pub fn NET_DVR_GetSDKBuildVersion() -> DWORD;
    pub fn NET_DVR_GetSDKVersion() -> DWORD;
    pub fn NET_DVR_SetLogToFile(nLogLevel: DWORD, strLogDir: *const c_char, bAutoDel: BOOL) -> BOOL;
    pub fn NET_DVR_SetLogPrint(bLogPrint: BOOL) -> BOOL;
    pub fn NET_DVR_SetConnectTime(dwWaitTime: DWORD, dwTryTimes: DWORD) -> BOOL;
    pub fn NET_DVR_SetExceptionCallBack_V30(
        nMessage: DWORD,
        hWnd: HWND,
        fExceptionCallBack: fExceptionCallBack,
        pUser: *mut c_void,
    );
    pub fn NET_DVR_SetDVRMessageCallBack_V30(
        fMessageCallBack: fMessCallBack_V30,
        pUser: *mut c_void,
    );
    pub fn NET_DVR_SetDVRMessageCallBack_V51(
        nMessage: LONG,
        fMessageCallBack: fMessCallBackEx_V51,
        pUser: *mut c_void,
    );
    pub fn NET_DVR_StartListen_V30(
        sLocalIP: *const c_char,
        wLocalPort: WORD,
        fDataCallBack: fMessCallBackEx_V51,
        pUserData: *mut c_void,
    ) -> LONG;
    pub fn NET_DVR_StopListen_V30(lHandle: LONG) -> BOOL;
    pub fn NET_DVR_RealPlay_V40(
        lUserID: LONG,
        pPreviewInfo: *const NET_DVR_PREVIEWINFO,
        fRealDataCallBack_V30: *mut c_void,
        pUser: *mut c_void,
    ) -> LONG;
    pub fn NET_DVR_StopRealPlay(lRealHandle: LONG) -> BOOL;
    pub fn NET_DVR_SetSDKSecretKey(lUserID: LONG, sSecretKey: *const c_char) -> BOOL;
}

// ---------------------------------------------------------------------------
// libPlayCtrl (PlayM4) bindings — frame decoder & renderer.
// ---------------------------------------------------------------------------

pub type PLAYM4_HWND = *mut c_void;
pub type PLAYM4_HDC = *mut c_void;

pub const T_YV12: c_int = 3;
pub const T_UYVY: c_int = 1;
pub const T_RGB32: c_int = 7;
pub const T_AUDIO16: c_int = 101;
pub const T_AUDIO8: c_int = 100;

pub const STREAME_REALTIME: c_uint = 0;
pub const STREAME_FILE: c_uint = 1;

pub const PLAYM4_NOERROR: c_int = 0;
pub const PLAYM4_DEC_VIDEO_ERROR: c_int = 4;
pub const PLAYM4_DEC_AUDIO_ERROR: c_int = 5;
pub const PLAYM4_ALLOC_MEMORY_ERROR: c_int = 6;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FRAME_INFO {
    pub nWidth: c_int,
    pub nHeight: c_int,
    pub nStamp: c_int,
    pub nType: c_int,
    pub nFrameRate: c_int,
    pub dwFrameNum: c_uint,
}

/// Decoded frame callback. Called by libPlayCtrl from its internal
/// thread for every decoded video frame. The buffer is in the
/// format specified by `pFrameInfo.nType` (e.g. T_YV12).
pub type pfnDecCBFun = Option<
    unsafe extern "C" fn(
        nPort: c_int,
        pBuf: *mut c_char,
        nSize: c_int,
        pFrameInfo: *const FRAME_INFO,
        nReserved1: *mut c_void,
        nReserved2: c_int,
    ),
>;

/// Raw real-time stream data callback for `NET_DVR_RealPlay_V40`.
pub type fRealDataCallBack_V30 = Option<
    unsafe extern "C" fn(
        lRealHandle: LONG,
        dwDataType: DWORD,
        pBuffer: *mut c_uchar,
        dwBufSize: DWORD,
        pUser: *mut c_void,
    ),
>;

extern "C" {
    pub fn PlayM4_GetSdkVersion() -> c_uint;
    pub fn PlayM4_GetPort(nPort: *mut c_int) -> c_int;
    pub fn PlayM4_FreePort(nPort: c_int) -> c_int;
    pub fn PlayM4_SetStreamOpenMode(nPort: c_int, nMode: c_uint) -> c_int;
    pub fn PlayM4_OpenStream(
        nPort: c_int,
        pFileHeadBuf: *mut c_uchar,
        nSize: c_uint,
        nBufPoolSize: c_uint,
    ) -> c_int;
    pub fn PlayM4_CloseStream(nPort: c_int) -> c_int;
    pub fn PlayM4_InputData(nPort: c_int, pBuf: *mut c_uchar, nSize: c_uint) -> c_int;
    pub fn PlayM4_Play(
        nPort: c_int,
        hWnd: PLAYM4_HWND,
    ) -> c_int;
    pub fn PlayM4_Stop(nPort: c_int) -> c_int;
    pub fn PlayM4_Pause(nPort: c_int, nPause: c_uint) -> c_int;
    pub fn PlayM4_SetDecCallBack(
        nPort: c_int,
        DecCBFun: pfnDecCBFun,
    ) -> c_int;
    pub fn PlayM4_SetDecCallBackMend(
        nPort: c_int,
        DecCBFun: pfnDecCBFun,
        nUser: *mut c_void,
    ) -> c_int;
    pub fn PlayM4_GetPictureSize(
        nPort: c_int,
        pWidth: *mut c_int,
        pHeight: *mut c_int,
    ) -> c_int;
    pub fn PlayM4_GetLastError(nPort: c_int) -> c_uint;
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Read a NUL-terminated byte string (in a C-style `BYTE` array) into a
/// Rust `String`, dropping any trailing garbage and decoding as GB18030 /
/// Latin1 fallback (the SDK stores mixed Chinese/ASCII text in some fields).
pub fn c_bytes_to_string(buf: &[u8]) -> String {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..len]).trim().to_string()
}
