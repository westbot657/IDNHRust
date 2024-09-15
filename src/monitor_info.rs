#[cfg(target_os = "windows")]
mod monitor_info {
    extern crate winapi;

    use std::ptr::null_mut;
    use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFO};
    use winapi::shared::windef::{HMONITOR, HDC, RECT};
    use winapi::shared::minwindef::LPARAM;

    struct MonitorData<'a> {
        monitors: &'a mut Vec<(i32, i32, u32, u32)>,
    }

    unsafe extern "system" fn monitor_enum_proc(
        hmonitor: HMONITOR,
        _hdc: HDC,
        _lprect: *mut RECT,
        lparam: LPARAM,
    ) -> i32 {
        let monitor_data = &mut *(lparam as *mut MonitorData);

        let mut mi = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            rcWork: RECT { left: 0, top: 0, right: 0, bottom: 0 },
            dwFlags: 0,
        };

        if GetMonitorInfoW(hmonitor, &mut mi) != 0 {
            let x = mi.rcMonitor.left;
            let y = mi.rcMonitor.top;
            let width = (mi.rcMonitor.right - mi.rcMonitor.left) as u32;
            let height = (mi.rcMonitor.bottom - mi.rcMonitor.top) as u32;
            
            monitor_data.monitors.push((x, y, width, height));
        }
        1 // Continue enumeration
    }

    pub fn get_monitor_info() -> Vec<(i32, i32, u32, u32)> {
        let mut monitors = Vec::new();

        let mut monitor_data = MonitorData {
            monitors: &mut monitors,
        };

        unsafe {
            EnumDisplayMonitors(
                null_mut(),
                null_mut(),
                Some(monitor_enum_proc),
                &mut monitor_data as *mut _ as LPARAM,
            );
        }

        monitors
    }
}


#[cfg(target_os = "linux")]
mod monitor_info {
    extern crate x11;
    use x11::xlib;
    use std::ptr;

    pub fn get_monitor_info() -> Vec<(i32, i32, u32, u32)> {
        let mut monitors: Vec<(i32, i32, u32, u32)> = Vec::new();
        unsafe {
            // Open the connection to the X server
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                eprintln!("Cannot open X display");
                return;
            }

            // Get the root window (usually the whole screen)
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);

            // Get the number of monitors (screen count)
            let mut num_monitors = 0;
            let monitors = xlib::XRRGetMonitors(display, root, xlib::True, &mut num_monitors);
            let monitors_slice = std::slice::from_raw_parts(monitors, num_monitors as usize);

            for monitor in monitors_slice {
                let x = (*monitor).x;
                let y = (*monitor).y;
                let width = (*monitor).width;
                let height = (*monitor).height;

                monitors.push((x, y, width, height));
            }

            xlib::XRRFreeMonitors(monitors);
            xlib::XCloseDisplay(display);
        }

        monitors
    }
}

#[cfg(target_os = "macos")]
mod monitor_info {
    extern crate core_graphics;
    use core_graphics::display::{CGDisplay, CGDisplayBounds};

    pub fn get_monitor_info() -> Vec<(i32, i32, u32, u32)> {
        let active_displays = CGDisplay::active_displays().unwrap();
        let monitors: Vec<(i32, i32, u32, u32)> = Vec::new();
        for display_id in active_displays {
            let display = CGDisplay::new(display_id);
            let bounds: CGDisplayBounds = display.bounds();
            let width = bounds.size.width;
            let height = bounds.size.height;
            let x = bounds.origin.x;
            let y = bounds.origin.y;
            
            monitors.push((x, y, width, height));
        }

        monitors
    }
}

pub fn get_info() -> Vec<(i32, i32, u32, u32)> {
    monitor_info::get_monitor_info()
}
