use crate::globals;
use opengfd::io::{
    controller::ControllerPlatform,
    keyboard::Keyboard
};
use std::ptr::NonNull;
use riri_mod_tools_proc::{ riri_hook_fn, riri_hook_static };
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

#[riri_hook_static(dynamic_offset(
    signature = "0F B6 05 ?? ?? ?? ?? 84 C0 74 ??",
    resolve_type = set_key_code_for_mouse_click,
    calling_convention = "microsoft",
))]
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

// gfdPad

#[no_mangle]
pub unsafe extern "C" fn set_device_pad_get_data(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got gfdDevicePadGetData 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_fn(dynamic_offset(
    signature = "48 89 5C 24 ?? 57 48 83 EC 30 48 63 D9 48 8B FA",
    resolve_type = set_device_pad_get_data,
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDevicePadGetData(id: u32, result: *mut u8) -> bool {
    let result = &mut *(result as *mut ControllerPlatform); 
    let success = result.update(id);
    // if id == 0 {
    //     logln!(Verbose, "Controller {}: {}", id, result);
    // }
    success
}