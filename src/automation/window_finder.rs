//! Win32 multi-tier window discovery and client dimensions inspector for Genshin Impact.
#![allow(dead_code)]

use std::sync::atomic::{AtomicIsize, Ordering};
use std::time::{Duration, Instant};
use windows::core::w;
use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM, POINT, RECT};
use windows::Win32::Graphics::Gdi::ClientToScreen;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::UI::WindowsAndMessaging::{
    FindWindowW, GetClientRect, GetWindow, GetWindowTextW, GetWindowThreadProcessId, IsIconic,
    IsWindow, IsWindowVisible, EnumWindows, GW_OWNER,
};

static CACHED_HWND: AtomicIsize = AtomicIsize::new(0);
static LAST_SCAN_TIME: std::sync::Mutex<Option<Instant>> = std::sync::Mutex::new(None);

/// Information about an attached Genshin window.
#[derive(Debug, Clone)]
pub struct GameWindowTarget {
    pub hwnd: isize,
    pub title: String,
    pub client_x: i32,
    pub client_y: i32,
    pub width: u32,
    pub height: u32,
    pub is_minimized: bool,
}

/// Discovers the active Genshin Impact window handle with handle caching and multi-tier fallbacks.
pub fn find_genshin_window() -> Option<GameWindowTarget> {
    let cached = CACHED_HWND.load(Ordering::Relaxed);
    if cached != 0 {
        let hwnd = HWND(cached as *mut _);
        unsafe {
            if IsWindow(hwnd).as_bool() {
                if let Some(target) = extract_window_bounds(hwnd, "Genshin Impact".to_string()) {
                    return Some(target);
                }
            }
        }
        CACHED_HWND.store(0, Ordering::Relaxed);
    }

    let mut last = LAST_SCAN_TIME.lock().ok()?;
    if let Some(instant) = *last {
        if instant.elapsed() < Duration::from_millis(500) {
            return None;
        }
    }
    *last = Some(Instant::now());

    let target = search_active_game_window()?;
    CACHED_HWND.store(target.hwnd, Ordering::Relaxed);
    Some(target)
}

fn search_active_game_window() -> Option<GameWindowTarget> {
    const PROCESS_NAMES: &[&str] = &["GenshinImpact.exe", "YuanShen.exe"];

    // Tier 1: Process snapshot discovery
    if let Some(target) = find_by_process_names(PROCESS_NAMES) {
        return Some(target);
    }

    // Tier 2: Unity class with known title
    if let Some(target) = unsafe { find_by_class_and_title() } {
        return Some(target);
    }

    // Tier 3: Fuzzy window title enumeration
    if let Some(target) = unsafe { find_by_title_contains(&["Genshin Impact", "原神"]) } {
        return Some(target);
    }

    // Tier 4: Exact FindWindowW literal fallback
    find_by_exact_fallback()
}

fn find_by_process_names(process_names: &[&str]) -> Option<GameWindowTarget> {
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0).ok()?;
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            ..Default::default()
        };

        let mut matched_pid = None;
        if Process32FirstW(snap, &mut entry).is_ok() {
            loop {
                let len = entry.szExeFile.iter().position(|&c| c == 0).unwrap_or(entry.szExeFile.len());
                let name = String::from_utf16_lossy(&entry.szExeFile[..len]);
                if process_names.iter().any(|&p| p.eq_ignore_ascii_case(&name)) {
                    matched_pid = Some(entry.th32ProcessID);
                    break;
                }
                if Process32NextW(snap, &mut entry).is_err() {
                    break;
                }
            }
        }
        let _ = CloseHandle(snap);

        let pid = matched_pid?;
        find_main_window_for_pid(pid)
    }
}

unsafe fn find_main_window_for_pid(target_pid: u32) -> Option<GameWindowTarget> {
    struct Context {
        pid: u32,
        found_hwnd: Option<HWND>,
    }

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = &mut *(lparam.0 as *mut Context);
        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == ctx.pid && IsWindowVisible(hwnd).as_bool() {
            let owner = GetWindow(hwnd, GW_OWNER).unwrap_or_default();
            if owner == HWND::default() {
                ctx.found_hwnd = Some(hwnd);
                return BOOL(0);
            }
        }
        BOOL(1)
    }

    let mut ctx = Context { pid: target_pid, found_hwnd: None };
    let _ = EnumWindows(Some(enum_proc), LPARAM(&mut ctx as *mut _ as isize));
    let hwnd = ctx.found_hwnd?;
    extract_window_bounds(hwnd, "Genshin Impact".to_string())
}

unsafe fn find_by_class_and_title() -> Option<GameWindowTarget> {
    let titles = [w!("Genshin Impact"), w!("原神")];
    for title in titles {
        if let Ok(hwnd) = FindWindowW(w!("UnityWndClass"), title) {
            if IsWindowVisible(hwnd).as_bool() {
                return extract_window_bounds(hwnd, "Genshin Impact".to_string());
            }
        }
    }
    None
}

unsafe fn find_by_title_contains(keywords: &[&str]) -> Option<GameWindowTarget> {
    struct Context<'a> {
        keywords: &'a [&'a str],
        found_hwnd: Option<(HWND, String)>,
    }

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx = &mut *(lparam.0 as *mut Context);
        if IsWindowVisible(hwnd).as_bool() {
            let mut buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buf);
            if len > 0 {
                let title = String::from_utf16_lossy(&buf[..len as usize]);
                for kw in ctx.keywords {
                    if title.to_lowercase().contains(&kw.to_lowercase()) {
                        ctx.found_hwnd = Some((hwnd, title));
                        return BOOL(0);
                    }
                }
            }
        }
        BOOL(1)
    }

    let mut ctx = Context { keywords, found_hwnd: None };
    let _ = EnumWindows(Some(enum_proc), LPARAM(&mut ctx as *mut _ as isize));
    let (hwnd, title) = ctx.found_hwnd?;
    extract_window_bounds(hwnd, title)
}

fn find_by_exact_fallback() -> Option<GameWindowTarget> {
    unsafe {
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
    let is_minimized = IsIconic(hwnd).as_bool();
    let mut rect = RECT::default();
    if GetClientRect(hwnd, &mut rect).is_err() {
        return None;
    }

    let mut pt = POINT { x: 0, y: 0 };
    if !ClientToScreen(hwnd, &mut pt).as_bool() {
        return None;
    }

    let mut width = (rect.right - rect.left).max(0) as u32;
    let mut height = (rect.bottom - rect.top).max(0) as u32;

    if is_minimized && (width == 0 || height == 0) {
        width = 1920;
        height = 1080;
    } else if width == 0 || height == 0 {
        return None;
    }

    Some(GameWindowTarget {
        hwnd: hwnd.0 as isize,
        title,
        client_x: pt.x,
        client_y: pt.y,
        width,
        height,
        is_minimized,
    })
}
