use anyhow::{anyhow, Result};
use std::cell::LazyCell;
use std::ptr::null_mut;
use winapi::shared::minwindef::{HINSTANCE, LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{
    CallNextHookEx, GetDoubleClickTime, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    HC_ACTION, MSLLHOOKSTRUCT, WH_MOUSE_LL, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE,
    WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP,
};

static mut HOOK: HHOOK = null_mut();
const DOUBLE_CLICK_TIME: LazyCell<u32> = LazyCell::new(|| unsafe { GetDoubleClickTime() });
static mut LAST_CLICK_TIME: u32 = 0;

unsafe extern "system" fn mouse_hook_proc(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == HC_ACTION {
        let mouse_info = *(l_param as *const MSLLHOOKSTRUCT);
        match w_param as u32 {
            WM_LBUTTONDOWN => {
                if LAST_CLICK_TIME == 0 || mouse_info.time - LAST_CLICK_TIME > *DOUBLE_CLICK_TIME {
                    println!(
                        "Left Button Down at ({}, {})",
                        mouse_info.pt.x, mouse_info.pt.y
                    );
                } else {
                    println!("Double Click at ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
                }
                LAST_CLICK_TIME = mouse_info.time;
            }
            WM_LBUTTONUP => {
                println!(
                    "Left Button Up at ({}, {})",
                    mouse_info.pt.x, mouse_info.pt.y
                );
            }
            WM_MOUSEMOVE => {
                println!("Mouse Move at ({}, {})", mouse_info.pt.x, mouse_info.pt.y);
            }
            WM_RBUTTONDOWN => {
                println!(
                    "Right Button Down at ({}, {})",
                    mouse_info.pt.x, mouse_info.pt.y
                );
            }
            WM_RBUTTONUP => {
                println!(
                    "Right Button Up at ({}, {})",
                    mouse_info.pt.x, mouse_info.pt.y
                );
            }
            WM_MOUSEWHEEL => {}
            _ => {
                panic!("Unknown message: {}", w_param);
            }
        }
    }
    CallNextHookEx(HOOK, code, w_param, l_param)
}

pub unsafe fn set_mouse_hook(handle: HINSTANCE) -> Result<()> {
    let hook = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook_proc), handle, 0);
    if hook.is_null() {
        let error = GetLastError();
        return Err(anyhow!(error));
    }
    HOOK = hook;
    Ok(())
}

pub fn listen() -> Result<()> {
    unsafe {
        let handle = GetModuleHandleW(null_mut());
        if let Err(error) = set_mouse_hook(handle) {
            println!("Error: {}", error);
        };
        GetMessageW(null_mut(), null_mut(), 0, 0);
    }
    Ok(())
}

pub fn close() -> Result<()> {
    unsafe {
        UnhookWindowsHookEx(HOOK);
        HOOK = null_mut();
    }
    Ok(())
}
