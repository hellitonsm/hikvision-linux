use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use chrono::Local;
use slint::{ComponentHandle, ModelRc, SharedString, Timer, VecModel};
use hcnetsdk_rust_demo::mainwindow::devicetree::{self, DeviceData, ChannelData};
use hcnetsdk_rust_demo::mainwindow::logalarm::{AlarmEntry, LogAlarmState, LogEntry};
use hcnetsdk_rust_demo::realplay::RealPlay;
use hcnetsdk_rust_demo::sdk;
use hcnetsdk_rust_demo::sdk::callbacks::SdkEvent;

slint::include_modules!();
const TREE_PATH: &str = "device_tree.txt";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let sdk_ok = sdk::init().is_ok();
    let _listener = if sdk_ok { sdk::start_alarm_listener(sdk::callbacks::new_queue()).ok() } else { None };
    let devices = devicetree::load(std::path::Path::new(TREE_PATH));
    log::info!("Loaded {} devices", devices.len());

    let ui = MainWindow::new().unwrap();
    let add_dlg = AddNodeDialog::new().unwrap();
    let dev_dlg = DeviceAttrDialog::new().unwrap();
    let chan_dlg = ChannelAttrDialog::new().unwrap();
    let exit_dlg = ExitDialog::new().unwrap();

    let data = Rc::new(RefCell::new(GlobalData::new(devices, sdk_ok)));
    AppState::get(&ui).set_sdk_version(SharedString::from(data.borrow().sdk_version.clone()));
    AppState::get(&ui).set_player_version(SharedString::from(data.borrow().player_version.clone()));
    AppState::get(&ui).set_status_text(SharedString::from("Ready"));
    refresh_tree(&ui, &data);

    // RealPlay is a single persistent instance.
    let rp: Rc<RefCell<RealPlay>> = Rc::new(RefCell::new(RealPlay::new()));

    // Timer: clock + events + action poll + preview frame poll
    let t_data = data.clone();
    let t_ui = ui.as_weak();
    let t_add = add_dlg.as_weak();
    let t_dev = dev_dlg.as_weak();
    let t_chan = chan_dlg.as_weak();
    let t_rp = rp.clone();
    let timer = Timer::default();
    timer.start(slint::TimerMode::Repeated, Duration::from_millis(100), move || {
        let Some(ui) = t_ui.upgrade() else { return };
        AppState::get(&ui).set_status_text(SharedString::from(Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));
        if let Some(rx) = sdk::callbacks::receiver() {
            while let Ok(ev) = rx.try_recv() {
                if let SdkEvent::Alarm { kind, device_ip, device_name } = ev {
                    t_data.borrow_mut().log.add_alarm(AlarmEntry::from(kind.label(), &device_ip, &device_name));
                }
            }
        }
        check_action_flags(&ui, &t_data, &t_add, &t_dev, &t_chan, sdk_ok);

        // Poll X11 events on the preview window (handles WM close).
        t_rp.borrow_mut().poll_window_events();

        // Sync preview state
        let rp_b = t_rp.borrow();
        let active = rp_b.preview_active;
        drop(rp_b);
        if AppState::get(&ui).get_preview_active() != active {
            AppState::get(&ui).set_preview_active(active);
        }
    });

    // tree click
    let c_data = data.clone();
    let c_ui = ui.as_weak();
    ui.on_tree_clicked(move |idx: i32| {
        let Some(ui) = c_ui.upgrade() else { return };
        let (l, di, ci) = row_lookup(&c_data, idx);
        AppState::get(&ui).set_selected_tree_row(idx);
        AppState::get(&ui).set_selected_level(l);
        AppState::get(&ui).set_selected_device_index(di as i32);
        AppState::get(&ui).set_selected_channel_index(ci as i32);
    });

    // tree double-click
    let d_data = data.clone();
    let d_ui = ui.as_weak();
    let d_add = add_dlg.as_weak();
    let d_rp = rp.clone();
    ui.on_tree_double_clicked(move |idx: i32| {
        let ui = d_ui.unwrap();
        let (l, di, ci) = row_lookup(&d_data, idx);
        if l == 0 { if let Some(d) = d_add.upgrade() { let _ = d.show(); } }
        else if l == 1 { try_login(&ui, &d_data, di, sdk_ok); }
        else if l == 2 {
            let ch = activate_channel(&ui, &d_data, di, ci, sdk_ok);
            let uid = AppState::get(&ui).get_current_user_id();
            if uid > 0 && ch > 0 {
                AppState::get(&ui).set_current_page(1);
                let mut r = d_rp.borrow_mut();
                if let Err(e) = r.start(uid, ch, 0, 0) {
                    AppState::get(&ui).set_status_text(SharedString::from(format!("Preview: {e}")));
                }
            }
        }
    });

    // exit
    let e_data = data.clone();
    let e_show = exit_dlg.as_weak();
    let e_confirm = exit_dlg.as_weak();
    let e_cancel = exit_dlg.as_weak();
    ui.on_exit_clicked(move || { if let Some(d) = e_show.upgrade() { let _ = d.show(); } });
    exit_dlg.on_confirm(move || {
        if let Some(d) = e_confirm.upgrade() { let _ = d.hide(); }
        let devs = e_data.borrow();
        devicetree::save(std::path::Path::new(TREE_PATH), &devs.devices.borrow()).ok();
        drop(devs);
        sdk::cleanup(); std::process::exit(0);
    });
    exit_dlg.on_cancel(move || { if let Some(d) = e_cancel.upgrade() { let _ = d.hide(); } });

    // add device ok
    let a_data = data.clone();
    let a_ui = ui.as_weak();
    let a_win = add_dlg.as_weak();
    add_dlg.on_ok(move || {
        let Some(dlg) = a_win.upgrade() else { return };
        let ip = dlg.get_ip().to_string();
        let port = dlg.get_port().parse().unwrap_or(8000);
        let mut dev = DeviceData {
            name: if dlg.get_node_name().is_empty() { ip.clone() } else { dlg.get_node_name().to_string() },
            ip: ip.clone(), port, user: dlg.get_username().to_string(),
            password: dlg.get_password().to_string(), secret_key: dlg.get_secret_key().to_string(),
            ..Default::default()
        };
        let _ = dlg.hide();
        if sdk_ok {
    match sdk::login(&dev.ip, dev.effective_port(), dev.effective_user(), &dev.password, &dev.secret_key) {
                Ok((uid, info)) => {
                    for i in 0..info.by_chan_num as i32 {
                        dev.channels.push(ChannelData { name: format!("Camera{}", info.by_start_chan as i32 + i), number: info.by_start_chan as i32 + i, ..Default::default() });
                    }
                    for i in 0..info.by_ip_chan_num as i32 {
                        dev.channels.push(ChannelData { name: format!("IPCamera{}", i+1), number: i+33, ..Default::default() });
                    }
                    dev.zero_chan_num = info.by_zero_chan_num;
                    sdk::logout(uid);
                }
                Err(e) => {
                    a_data.borrow_mut().log.add_log(LogEntry::failure("AddDevice", &ip, &e));
                    let ui = a_ui.unwrap(); sync_log(&ui, &a_data);
                    AppState::get(&ui).set_status_text(SharedString::from(format!("Login fail: {e}")));
                    return;
                }
            }
        } else {
            for i in 0..4 { dev.channels.push(ChannelData { name: format!("Camera{}", i+1), number: i+1, ..Default::default() }); }
        }
        let mut guard = a_data.borrow_mut();
        let mut inner = guard.devices.borrow_mut();
        inner.push(dev);
        drop(inner);
        devicetree::save(std::path::Path::new(TREE_PATH), &guard.devices.borrow()).ok();
        drop(guard);
        a_data.borrow_mut().log.add_log(LogEntry::success("AddDevice", &ip, "ok"));
        let ui = a_ui.unwrap();
        refresh_tree(&ui, &a_data); sync_log(&ui, &a_data);
        AppState::get(&ui).set_status_text(SharedString::from(format!("Added: {ip}")));
    });

    // device/channel attr close
    let dw = dev_dlg.as_weak(); let cw = chan_dlg.as_weak();
    dev_dlg.on_close(move || { if let Some(d) = dw.upgrade() { let _ = d.hide(); } });
    chan_dlg.on_close(move || { if let Some(d) = cw.upgrade() { let _ = d.hide(); } });

    // realplay
    let rp_s = rp.clone(); let rp_s_data = data.clone(); let rp_s_ui = ui.as_weak();
    ui.on_realplay_start(move |channel: i32, stream: slint::SharedString, protocol: slint::SharedString| {
        let ui = rp_s_ui.unwrap();
        let mut g = rp_s.borrow_mut();
        let uid = AppState::get(&ui).get_current_user_id();
        let stream_type = match stream.as_str() {
            "Main stream" => 0u32,
            "Sub stream" => 1,
            "Third stream" => 2,
            "Trans code" => 3,
            "Fourth stream" => 4,
            _ => 0,
        };
        let link_mode = match protocol.as_str() {
            "TCP" => 0u32,
            "UDP" => 1,
            "MULTICAST" => 2,
            "RTP" => 3,
            "RTSP" => 4,
            "HTTPS" => 5,
            _ => 0,
        };
        match g.start(uid, channel, stream_type, link_mode) {
            Ok(()) => rp_s_data.borrow_mut().log.add_log(LogEntry::success("RealPlay", "—", &format!("ch={} stream={} proto={}", channel, stream, protocol))),
            Err(e) => {
                rp_s_data.borrow_mut().log.add_log(LogEntry::failure("RealPlay", "—", &e));
                AppState::get(&ui).set_status_text(SharedString::from(format!("Preview: {e}")));
            }
        }
    });
    let rp_t = rp.clone(); let rp_t_data = data.clone();
    ui.on_realplay_stop(move || {
        rp_t.borrow_mut().stop();
        rp_t_data.borrow_mut().log.add_log(LogEntry::success("RealPlay", "—", "stop"));
    });
    ui.on_realplay_snapshot(move || {
        log::info!("Snapshot requested (not yet implemented)");
    });
    let rp_p_data = data.clone(); let rp_p_ui = ui.as_weak();
    ui.on_ptz(move |c: i32| { rp_p_data.borrow_mut().log.add_log(LogEntry::success("PTZ", "—", &c.to_string())); sync_log(&rp_p_ui.unwrap(), &rp_p_data); });

    // log panel
    let cl_data = data.clone(); let cl_ui = ui.as_weak();
    let sv_data = data.clone(); let te_data = data.clone(); let te_ui = ui.as_weak();
    ui.on_clear_logs(move || { cl_data.borrow_mut().log.clear_logs(); sync_log(&cl_ui.unwrap(), &cl_data); });
    ui.on_save_logs(move || { let _ = std::fs::write("logs_export.json", serde_json::to_string_pretty(&sv_data.borrow().log.logs).unwrap_or_default()); });
    ui.on_test_event(move || { te_data.borrow_mut().log.add_log(LogEntry::success("TEST", "127.0.0.1", "ok")); sync_log(&te_ui.unwrap(), &te_data); });

    println!("HCNetSDK Rust Demo started");
    ui.run().unwrap();
    devicetree::save(std::path::Path::new(TREE_PATH), &data.borrow().devices.borrow()).ok();
    sdk::cleanup();
}

// ---- helpers ----

fn check_action_flags(ui: &MainWindow, data: &Rc<RefCell<GlobalData>>, add_w: &slint::Weak<AddNodeDialog>, dev_w: &slint::Weak<DeviceAttrDialog>, chan_w: &slint::Weak<ChannelAttrDialog>, sdk_ok: bool) {
    if AppState::get(ui).get_action_add_device() { AppState::get(ui).set_action_add_device(false); if let Some(d) = add_w.upgrade() { let _ = d.show(); } return; }
    if AppState::get(ui).get_action_login() { AppState::get(ui).set_action_login(false); let (l, di, _) = row_lookup(data, AppState::get(ui).get_selected_tree_row()); if l == 1 { try_login(ui, data, di, sdk_ok); } return; }
    if AppState::get(ui).get_action_logout() {
        AppState::get(ui).set_action_logout(false);
        let (l, di, _) = row_lookup(data, AppState::get(ui).get_selected_tree_row());
        if l != 1 { return; }
        let mut guard = data.borrow_mut();
        let mut ds = guard.devices.borrow_mut();
        let Some(d) = ds.get_mut(di) else { return };
        let ip = d.ip.clone();
        if d.user_id >= 0 { if sdk_ok { sdk::logout(d.user_id); } d.user_id = -1; }
        devicetree::save(std::path::Path::new(TREE_PATH), &ds).ok();
        drop(ds);
        drop(guard);
        data.borrow_mut().log.add_log(LogEntry::success("Logout", &ip, ""));
        refresh_tree(ui, data); sync_log(ui, data);
        return;
    }
    if AppState::get(ui).get_action_delete() {
        AppState::get(ui).set_action_delete(false);
        let (l, di, _) = row_lookup(data, AppState::get(ui).get_selected_tree_row());
        if l != 1 { return; }
        let mut guard = data.borrow_mut();
        let mut ds = guard.devices.borrow_mut();
        let ip = ds.get(di).map(|d| d.ip.clone()).unwrap_or_default();
        if let Some(d) = ds.get(di) { if d.user_id >= 0 && sdk_ok { sdk::logout(d.user_id); } }
        ds.remove(di);
        devicetree::save(std::path::Path::new(TREE_PATH), &ds).ok();
        drop(ds);
        drop(guard);
        data.borrow_mut().log.add_log(LogEntry::success("Delete", &ip, ""));
        refresh_tree(ui, data); sync_log(ui, data);
        AppState::get(ui).set_selected_tree_row(-1); AppState::get(ui).set_selected_level(-1);
        return;
    }
    if AppState::get(ui).get_action_device_attr() {
        AppState::get(ui).set_action_device_attr(false);
        let (l, di, _) = row_lookup(data, AppState::get(ui).get_selected_tree_row());
        if l != 1 { return; }
        let guard = data.borrow();
        let ds = guard.devices.borrow();
        if let Some(d) = ds.get(di) {
            if let Some(dlg) = dev_w.upgrade() {
                dlg.set_device_name(SharedString::from(&d.name)); dlg.set_ip(SharedString::from(&d.ip));
                dlg.set_port(SharedString::from(&d.port.to_string())); dlg.set_user_id(SharedString::from(&d.user_id.to_string()));
                dlg.set_status(SharedString::from(if d.is_logged_in(){"logged in"}else{"offline"}));
                dlg.set_chan_count(SharedString::from(&d.channels.len().to_string())); dlg.set_zero_chan(SharedString::from(&d.zero_chan_num.to_string()));
                let _ = dlg.show();
            }
        }
        return;
    }
    if AppState::get(ui).get_action_channel_attr() {
        AppState::get(ui).set_action_channel_attr(false);
        let (l, di, ci) = row_lookup(data, AppState::get(ui).get_selected_tree_row());
        if l != 2 { return; }
        let guard = data.borrow();
        let ds = guard.devices.borrow();
        if let Some(d) = ds.get(di) { if let Some(c) = d.channels.get(ci) {
            if let Some(dlg) = chan_w.upgrade() {
                dlg.set_channel_name(SharedString::from(&c.name)); dlg.set_channel_num(SharedString::from(&c.number.to_string()));
                dlg.set_protocol(SharedString::from(c.protocol.label())); dlg.set_stream(SharedString::from(c.stream.label()));
                dlg.set_real_handle(SharedString::from(&c.real_handle.to_string())); dlg.set_online(SharedString::from(if c.real_handle >= 0{"yes"}else{"no"}));
                let _ = dlg.show();
            }
        }}
    }
}

fn row_lookup(data: &Rc<RefCell<GlobalData>>, idx: i32) -> (i32, usize, usize) {
    let guard = data.borrow();
    let rows = guard.devices.borrow();
    let mut t = 1i32;
    for (di, d) in rows.iter().enumerate() {
        if t == idx { return (1, di, 0); } t += 1;
        for (ci, _) in d.channels.iter().enumerate() { if t == idx { return (2, di, ci); } t += 1; }
    }
    (0, 0, 0)
}

fn refresh_tree(ui: &MainWindow, data: &Rc<RefCell<GlobalData>>) {
    let guard = data.borrow();
    let devices = guard.devices.borrow();
    let mut v = Vec::new();
    v.push(DeviceNode { depth: 0, label: "Devices".into(), online: false, device_index: 0, channel_index: 0, is_channel: false, is_root: true });
    for (di, d) in devices.iter().enumerate() {
        v.push(DeviceNode { depth: 1, label: d.name.clone().into(), online: d.is_logged_in(), device_index: di as i32, channel_index: 0, is_channel: false, is_root: false });
        for (ci, c) in d.channels.iter().enumerate() {
            v.push(DeviceNode { depth: 2, label: format!("{} ch{}", c.name, c.number).into(), online: c.real_handle >= 0, device_index: di as i32, channel_index: ci as i32, is_channel: true, is_root: false });
        }
    }
    AppState::get(ui).set_tree_rows(ModelRc::new(VecModel::from(v)));
}

fn sync_log(ui: &MainWindow, data: &Rc<RefCell<GlobalData>>) {
    let v: Vec<LogRow> = data.borrow().log.logs.iter().map(|e| LogRow {
        time: SharedString::from(&e.time), interface: SharedString::from(&e.interface),
        device_ip: SharedString::from(&e.device_ip), info: SharedString::from(&e.info), success: e.success,
    }).collect();
    AppState::get(ui).set_logs(ModelRc::new(VecModel::from(v)));
}

fn try_login(ui: &MainWindow, data: &Rc<RefCell<GlobalData>>, device_index: usize, sdk_ok: bool) {
    let dev = match data.borrow().devices.borrow().get(device_index).cloned() { Some(d) => d, _ => return };
    if dev.is_logged_in() { AppState::get(ui).set_status_text(SharedString::from("Already logged in")); return; }
    if !sdk_ok { data.borrow_mut().log.add_log(LogEntry::failure("Login", &dev.ip, "SDK missing")); sync_log(ui, data); return; }
            match sdk::login(&dev.ip, dev.effective_port(), dev.effective_user(), &dev.password, &dev.secret_key) {
        Ok((uid, info)) => {
            let mut guard = data.borrow_mut();
            let mut inner = guard.devices.borrow_mut();
            if let Some(d) = inner.get_mut(device_index) { d.user_id = uid; d.zero_chan_num = info.by_zero_chan_num; }
            drop(inner);
            guard.log.add_log(LogEntry::success("Login", &dev.ip, &format!("uid={}", uid)));
            devicetree::save(std::path::Path::new(TREE_PATH), &guard.devices.borrow()).ok();
            drop(guard);
            refresh_tree(ui, data); sync_log(ui, data);
            AppState::get(ui).set_status_text(SharedString::from(format!("Logged in: {}", dev.ip)));
        }
        Err(e) => { data.borrow_mut().log.add_log(LogEntry::failure("Login", &dev.ip, &e)); sync_log(ui, data); AppState::get(ui).set_status_text(SharedString::from(format!("Login fail: {e}"))); }
    }
}

fn activate_channel(ui: &MainWindow, data: &Rc<RefCell<GlobalData>>, device_index: usize, channel_index: usize, sdk_ok: bool) -> i32 {
    let dev = match data.borrow().devices.borrow().get(device_index).cloned() { Some(d) => d, _ => return -1 };
    if !dev.is_logged_in() { try_login(ui, data, device_index, sdk_ok); }
    let uid = data.borrow().devices.borrow().get(device_index).map(|d| d.user_id).unwrap_or(-1);
    if uid <= 0 { return -1; }
    let ch = dev.channels.get(channel_index).map(|c| c.number).unwrap_or(1);
    AppState::get(ui).set_current_user_id(uid);
    AppState::get(ui).set_preview_channel(ch);
    AppState::get(ui).set_preview_active(false);
    AppState::get(ui).set_status_text(SharedString::from(format!("Active: {} ch{}", dev.ip, ch)));
    ch
}

struct GlobalData {
    devices: RefCell<Vec<DeviceData>>,
    log: LogAlarmState,
    sdk_version: String,
    player_version: String,
    sdk_loaded: bool,
}
impl GlobalData {
    fn new(d: Vec<DeviceData>, sdk_ok: bool) -> Self {
        Self {
            devices: RefCell::new(d),
            log: LogAlarmState::default(),
            sdk_version: if sdk_ok { sdk::sdk_version() } else { "HCNetSDK (not loaded)".into() },
            player_version: "PlayCtrl (loading)".into(),
            sdk_loaded: sdk_ok,
        }
    }
}
