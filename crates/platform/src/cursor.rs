//! The mouse position in screen pixels, for when the windowing layer has none.
//!
//! While files are dragged in from the OS file manager, winit on Windows delivers no cursor events
//! (it only reports hovered and dropped files), so the UI cannot learn where a drop landed.
//!
//! This is the one non-media OS call in this crate: `crates/platform` is the only crate allowed
//! `unsafe` (AGENTS.md rule 13, `unsafe`, ADR 0001), so the single `GetCursorPos` call lives here rather than
//! in the UI. It is read-only, has a safe `Option`-returning API, and every other platform (and a
//! failed call) falls back to the last pointer position egui saw. Drop it once winit reports the
//! pointer position during an OS drag-and-drop.

/// The cursor's position on the virtual screen in physical pixels, or `None` where the OS cannot
/// say or this platform's windowing layer already reports it during a drag.
#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
pub fn cursor_screen_position() -> Option<(i32, i32)> {
    use windows::Win32::Foundation::POINT;
    use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: `p` is a valid, writable POINT for the duration of the call.
    unsafe { GetCursorPos(&mut p) }.ok()?;
    Some((p.x, p.y))
}

#[cfg(not(target_os = "windows"))]
pub fn cursor_screen_position() -> Option<(i32, i32)> {
    None
}

/// Apply dark title bar theme to our Windows window via DwmSetWindowAttribute.
/// Sets immersive dark mode and dark caption color (#181818) so the native title bar matches
/// the professional NLE dark UI.
#[cfg(target_os = "windows")]
#[allow(unsafe_code)]
pub fn set_window_dark_theme() {
    use windows::Win32::Foundation::HWND;
    use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryA};
    use windows::Win32::UI::WindowsAndMessaging::FindWindowW;
    use windows::core::{s, w};

    // SAFETY: Dynamic loading of dwmapi.dll to call DwmSetWindowAttribute on our own window.
    unsafe {
        let Ok(hwnd) = FindWindowW(None, w!("FrameCut Studio")) else {
            return;
        };

        if let Ok(dwm) = LoadLibraryA(s!("dwmapi.dll")) {
            if let Some(proc) = GetProcAddress(dwm, s!("DwmSetWindowAttribute")) {
                type DwmSetWindowAttributeFn = unsafe extern "system" fn(HWND, u32, *const std::ffi::c_void, u32) -> i32;
                let set_attr: DwmSetWindowAttributeFn = std::mem::transmute(proc);

                // DWMWA_USE_IMMERSIVE_DARK_MODE = 20 (Win11 / Win10 20H1+), 19 on older Win10
                let dark: i32 = 1;
                let _ = set_attr(hwnd, 20, &dark as *const _ as *const _, 4);
                let _ = set_attr(hwnd, 19, &dark as *const _ as *const _, 4);

                // DWMWA_CAPTION_COLOR = 35 (Win11 22000+) -> #181818 (BGR: 0x00181818)
                let caption_color: u32 = 0x00181818;
                let _ = set_attr(hwnd, 35, &caption_color as *const _ as *const _, 4);

                // DWMWA_TEXT_COLOR = 36 (Win11 22000+) -> #E0E0E0 (BGR: 0x00E0E0E0)
                let text_color: u32 = 0x00E0E0E0;
                let _ = set_attr(hwnd, 36, &text_color as *const _ as *const _, 4);

                // DWMWA_BORDER_COLOR = 34 (Win11 22000+) -> #282828 (BGR: 0x00282828)
                let border_color: u32 = 0x00282828;
                let _ = set_attr(hwnd, 34, &border_color as *const _ as *const _, 4);
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn set_window_dark_theme() {}
