

pub fn is_wsl() -> bool {
    if let Ok(os_release) = std::fs::read_to_string("/proc/sys/kernel/osrelease") {
        os_release.contains("microsoft")
    } else {
        false
    }
}


#[cfg(target_os="windows")]
mod platform {

}

#[cfg(target_os="linux")]
mod platform {

}





