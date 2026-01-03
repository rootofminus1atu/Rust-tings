use enigo::{Enigo, MouseButton, MouseControllable};
use std::{thread, time::Duration};
use sysinfo::System;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};

fn is_tf2_foreground() -> bool {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0 == 0 {
            return false;
        }
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return false;
        }
        let mut sys = System::new_all();
        sys.refresh_processes();
        let sys_pid = sysinfo::Pid::from_u32(pid);
        if let Some(process) = sys.process(sys_pid) {
            return process.name().eq_ignore_ascii_case("tf_win64.exe");
        }
        false
    }
}

fn main() {
    let mut enigo = Enigo::new();
    loop {
        if is_tf2_foreground() {
            println!("tf2");
            enigo.mouse_down(MouseButton::Back);  // m4
            enigo.mouse_up(MouseButton::Back);
        } else {
            println!("no");
        }
        thread::sleep(Duration::from_millis(500));
    }
}