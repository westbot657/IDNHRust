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
            cbSize: size_of::<MONITORINFO>() as u32,
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

    pub fn get_monitor_info() -> Vec<(i32, i32, u32, u32)> {
        let mut monitors: Vec<(i32, i32, u32, u32)> = Vec::new();

        monitors
    }
}

#[cfg(target_os = "macos")]
mod monitor_info {
    pub fn get_monitor_info() -> Vec<(i32, i32, u32, u32)> {
        let mut monitors: Vec<(i32, i32, u32, u32)> = Vec::new();

        monitors
    }
}

pub fn get_info() -> Vec<(i32, i32, u32, u32)> {
    monitor_info::get_monitor_info()
}
