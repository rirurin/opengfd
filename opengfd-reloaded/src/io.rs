use crate::globals;
use opengfd::io::{
    controller::ControllerPlatform,
    keyboard::Keyboard,
    mouse::WindowMouseState
};
use std::ptr::NonNull;
use riri_mod_tools_proc::{ riri_hook_fn, riri_hook_static, riri_init_fn };
use riri_mod_tools_rt::{ logln, sigscan_resolver };

// gfdKeyboard

#[no_mangle]
pub unsafe extern "C" fn set_key_code_for_mouse_click(ofs: usize) -> Option<NonNull<u8>> { 
    let addr_ptr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_long_abs(addr_ptr.as_ptr()) {
        Some(v) => v,
        None => return None
    };
    globals::set_keycode_for_mouse_click(addr.as_ptr());
    logln!(Information, "got keyCodeForMouseClick: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static({
    XRD759_UWP_1011 => dynamic_offset(
        signature = "0F B6 05 ?? ?? ?? ?? 48 89 AC 24 ?? ?? ?? ??",
        resolve_type = set_key_code_for_mouse_click,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "0F B6 05 ?? ?? ?? ?? 84 C0 74 ??",
        resolve_type = set_key_code_for_mouse_click,
        calling_convention = "microsoft",
    )
})]
riri_static!(HOOK_KEYCODE_FOR_MOUSE_CLICK, usize);

#[no_mangle]
pub unsafe extern "C" fn set_device_keyboard_get_data(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got gfdDeviceKeyboardGetData 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "40 53 48 81 EC 20 01 00 00 C5 FA 10 05 ?? ?? ?? ??",
    resolve_type = set_device_keyboard_get_data,
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDeviceKeyboardGetData(_board_id: usize, result: *mut u8) -> bool {
    if globals::get_keyboard_data().is_none() {
        logln!(Verbose, "set gfdKeyboardData to 0x{:x}", result as usize);
        globals::set_keyboard_data(result as *mut Keyboard);
    }
    let keyboard_data = &mut *(result as *mut Keyboard);
    keyboard_data.update()
}

#[riri_init_fn]
fn set_block_keyboard_update() {
    let _ = opengfd::io::keyboard::BLOCK_KEYBOARD_UPDATE.set(false);
    crate::globals::set_block_keyboard_focus(&raw const *opengfd::io::keyboard::BLOCK_KEYBOARD_UPDATE.get().unwrap() as *mut bool);
}

// gfdPad

#[no_mangle]
pub unsafe extern "C" fn set_device_pad_get_data(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got gfdDevicePadGetData: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn({
    XRD759_UWP_1011 => dynamic_offset(
        signature = "40 55 53 56 57 41 54 41 57",
        resolve_type = set_device_pad_get_data,
        calling_convention = "microsoft"),
    _ => dynamic_offset(
        signature = "48 89 5C 24 ?? 57 48 83 EC 30 48 63 D9 48 8B FA",
        resolve_type = set_device_pad_get_data,
        calling_convention = "microsoft")
})]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDevicePadGetData(id: u32, result: *mut u8) -> bool {
    let result = &mut *(result as *mut ControllerPlatform); 
    let success = result.update(id);
    // if id == 0 {
    //     logln!(Verbose, "Controller {}: {}", id, result);
    // }
    success
}

// gfdMouse

#[no_mangle]
pub unsafe extern "C" fn set_gfd_mouse_from_wnd_proc(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let glb = match sigscan_resolver::get_indirect_address_long_abs(addr.as_ptr().add(0x15)) {
        Some(v) => v,
        None => return None
    };
    // vmovups ymm0,YMMWORD PTR [rdx+r9*1+0x48] 
    globals::set_window_mouse_state(glb.as_ptr().add(0x48) as *mut WindowMouseState);
    logln!(Information, "got gfdMouseFromWindowProc: 0x{:x}", addr.as_ptr() as usize);
    logln!(Information, "got WindowMouseState: 0x{:x}", glb.as_ptr().add(0x48) as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "80 3D ?? ?? ?? ?? 00 4C 8B C2 75 ??",
    resolve_type = set_gfd_mouse_from_wnd_proc,
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdMouseFromWindowProc(_id: usize, p_state: *mut u8) -> bool {
    let wnd = globals::get_window_mouse_state_unchecked_mut();
    let state= unsafe { &mut *(p_state as *mut WindowMouseState) };
    let ret = state.update_from(wnd);
    wnd.set_scroll(0);
    ret
}

#[riri_init_fn]
fn set_block_mouse_update() {
    let _ = opengfd::io::mouse::BLOCK_MOUSE_UPDATE.set(false);
    crate::globals::set_block_mouse_focus(&raw const *opengfd::io::mouse::BLOCK_MOUSE_UPDATE.get().unwrap() as *mut bool);
}