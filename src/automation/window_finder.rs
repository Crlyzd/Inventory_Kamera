//! Win32 window discovery and client dimensions inspector for Genshin Impact.
#![allow(dead_code)]

use windows::core::w;
use windows::Win32::Foundation::{HWND, POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetClientRect, IsWindowVisible};

/// Information about an attached Genshin window.
#[derive(Debug, Clone)]
pub struct GameWindowTarget {
    pub hwnd: isize,
    pub title: String,
    pub client_x: i32,
    pub client_y: i32,
    pub width: u32,
    pub height: u32,
}

/// Discovers the active Genshin Impact window handle and retrieves its client bounds.
pub fn find_genshin_window() -> Option<GameWindowTarget> {
    unsafe {
        // Try English window title first
        let (hwnd, title) = if let Ok(h) = FindWindowW(None, w!("Genshin Impact")) {
            (h, "Genshin Impact".to_string())
        } else if let Ok(h) = FindWindowW(None, w!("原神")) {
            (h, "原神".to_string())
        } else {
            return None;
        };

        if !IsWindowVisible(hwnd).as_bool() {
            return None;
        }

        extract_window_bounds(hwnd, title)
    }
}

unsafe fn extract_window_bounds(hwnd: HWND, title: String) -> Option<GameWindowTarget> {
    let mut rect = RECT::default();
    if GetClientRect(hwnd, &mut rect).is_err() {
        return None;
    }

    let mut pt = POINT { x: 0, y: 0 };
    if !ClientToScreen(hwnd, &mut pt).as_bool() {
        return None;
    }

    let width = (rect.right - rect.left).max(0) as u32;
    let height = (rect.bottom - rect.top).max(0) as u32;

    if width == 0 || height == 0 {
        return None;
    }

    Some(GameWindowTarget {
        hwnd: hwnd.0 as isize,
        title,
        client_x: pt.x,
        client_y: pt.y,
        width,
        height,
    })
}
