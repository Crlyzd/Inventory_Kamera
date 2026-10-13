//! DirectInput scan code virtual keyboard driver using Win32 SendInput.
#![allow(dead_code)]

use std::thread::sleep;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE,
};

/// Common DirectInput scan codes for Genshin Impact menu traversal.
pub struct KeyCodes;

impl KeyCodes {
    pub const ESCAPE: u16 = 0x01;
    pub const KEY_1: u16 = 0x02;
    pub const ENTER: u16 = 0x1C;
    pub const KEY_C: u16 = 0x2E;
    pub const KEY_B: u16 = 0x30;
    pub const ARROW_UP: u16 = 0xC8;
    pub const ARROW_LEFT: u16 = 0xCB;
    pub const ARROW_RIGHT: u16 = 0xCD;
    pub const ARROW_DOWN: u16 = 0xD0;
}

/// Dispatches a key-down event using DirectInput scan code.
pub fn send_key_down(scan_code: u16) {
    let input = create_keyboard_input(scan_code, false);
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// Dispatches a key-up event using DirectInput scan code.
pub fn send_key_up(scan_code: u16) {
    let input = create_keyboard_input(scan_code, true);
    unsafe {
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

/// Sends an atomic keystroke tap with a designated hold duration.
pub fn send_key_tap(scan_code: u16, hold_ms: u64) {
    send_key_down(scan_code);
    sleep(Duration::from_millis(hold_ms.max(20)));
    send_key_up(scan_code);
}

fn create_keyboard_input(scan_code: u16, is_key_up: bool) -> INPUT {
    let mut flags = KEYEVENTF_SCANCODE;
    if is_key_up {
        flags |= KEYEVENTF_KEYUP;
    }

    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY(0),
                wScan: scan_code,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}
