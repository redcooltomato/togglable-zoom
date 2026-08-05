use std::{
    thread::sleep, time::{Duration, SystemTime},
};
use windows::Win32::UI::{
    Input::KeyboardAndMouse::{GetKeyState}, Magnification::{MagInitialize, MagSetFullscreenTransform}, WindowsAndMessaging::{GetSystemMetrics, IsProcessDPIAware, SM_CXSCREEN, SM_CYSCREEN, SetProcessDPIAware},
};
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
    let (screen_w, screen_h) = get_screen_res().unwrap();

    unsafe {
        if !MagInitialize().as_bool() {
            println!("failed to initialize magnifier");
            return;
        }
    }

    let magfactor: f32;

    let mut magfactor_s = String::new();
    loop {
        println!("input magnification factor you desire, 1.0-4096.0");
        match std::io::stdin().read_line(&mut magfactor_s) {
            Ok(_) => {
                match magfactor_s.trim().parse::<f32>() {
                    Ok(r) => {
                        magfactor = r;
                        break;
                    },
                    Err(e) => eprintln!("{}", e),
                }
                magfactor_s.clear();
            },
            Err(_) => (),
        }
    }

    let key: char;

    let mut key_s = String::new();
    loop {
        println!("input a character which key you want to toggle magnifier with");
        match std::io::stdin().read_line(&mut key_s) {
            Ok(_) => {
                if key_s.trim().len() == 1 {
                    key = key_s.chars().nth(0).unwrap().to_ascii_uppercase();
                    break;
                }
                key_s.clear();
            },
            Err(_) => (),
        }
    }

    let mut offset_x: i32;
    let mut offset_y: i32;

    let mut current_magfactor: f32;
    let mut prev_state = false;

    /* let start_time = SystemTime::now();
    let safeguard: Duration = Duration::from_secs(10); */

    println!("press ctrl-c to stop the program");
    loop {
        let key_active = unsafe {
            (GetKeyState(key as i32) as u16 & 0x8000) != 0
        };

        if key_active {
            current_magfactor = magfactor;
        } else {
            current_magfactor = 1.0;
        }

        offset_x = ((screen_w as f32) / 2.0 * (1. - (1. / magfactor))) as i32;
        offset_y = ((screen_h as f32) / 2.0 * (1. - (1. / magfactor))) as i32;

        if prev_state != key_active && 
        !unsafe { MagSetFullscreenTransform(current_magfactor, offset_x, offset_y).as_bool() } {
            println!("failed to magnify");
            break;
        }

        /* if SystemTime::now().duration_since(start_time).unwrap() > safeguard {
            break;
        } */

        prev_state = key_active;

        sleep(Duration::from_millis(50));
    }
}