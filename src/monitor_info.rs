#[cfg(target_os = "windows")]
mod monitor_info {
    extern crate winapi;

    use std::ptr::null_mut;
    use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFO, MonitorFromWindow, MONITOR_DEFAULTTOPRIMARY};
    use winapi::shared::windef::{HMONITOR, HDC, RECT};

    unsafe extern "system" fn monitor_enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprect: *mut RECT,
        _lparam: winapi::shared::minwindef::LPARAM,
    ) -> i32 {
        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            dwFlags: 0,
        };

        if GetMonitorInfoW(hmonitor, &mut mi) != 0 {
            let width = mi.rcMonitor.right - mi.rcMonitor.left;
            let height = mi.rcMonitor.bottom - mi.rcMonitor.top;
            println!("Monitor size (Windows): {}x{}", width, height);
        }
        1 // Continue enumeration
    }

    pub fn get_monitor_info() {
        unsafe {
            EnumDisplayMonitors(null_mut(), null_mut(), Some(monitor_enum_proc), 0);
        }
    }
}

#[cfg(target_os = "linux")]
mod monitor_info {
    extern crate x11;

    use x11::xlib;
    use std::ptr;

    pub fn get_monitor_info() {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                println!("Unable to open X display");
                return;
            }

            let screen = xlib::XDefaultScreen(display);
            let width = xlib::XDisplayWidth(display, screen);
            let height = xlib::XDisplayHeight(display, screen);

            println!("Screen size (Linux): {}x{}", width, height);

            xlib::XCloseDisplay(display);
        }
    }
}

#[cfg(target_os = "macos")]
mod monitor_info {
    extern crate core_graphics;

    use core_graphics::display::{CGDisplay, CGDisplayBounds};

    pub fn get_monitor_info() {
        let displays = CGDisplay::active_displays().unwrap();

        for display in displays {
            let bounds = CGDisplayBounds::new(display);
            let width = bounds.size.width;
            let height = bounds.size.height;
            println!("Monitor size (macOS): {}x{}", width, height);
        }
    }
}

fn main() {
    monitor_info::get_monitor_info();
}
