use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, IsProcessDPIAware, SM_CXSCREEN, SM_CYSCREEN, SetProcessDPIAware};
use anyhow::{Result, anyhow};

fn get_screen_res() -> Result<(i32, i32)> {
    unsafe {
        if !IsProcessDPIAware().as_bool() {
            if !SetProcessDPIAware().as_bool() {
                return Err(anyhow!("Failed to set proccess to DPI Aware"));
            }
        }
    }

    Ok(unsafe {
        (GetSystemMetrics(SM_CXSCREEN),
        GetSystemMetrics(SM_CYSCREEN))
    })
}

fn main() {
    dbg!(get_screen_res().unwrap());
}