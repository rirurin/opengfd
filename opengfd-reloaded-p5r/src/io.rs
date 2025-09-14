use std::{
    ptr::NonNull,
    sync::{ MutexGuard, Mutex, OnceLock },
};
use std::ops::{Index, IndexMut};
use riri_mod_tools_proc::riri_hook_fn;
use riri_mod_tools_rt::{logln, sigscan_resolver};
use opengfd::{
    device::hedge::hid::keyboard::KeyboardManager,
    io::controller::{ ControllerPlatform, ControllerPlatformManager }
};
use crate::{
    globals,
    util::Ptr
};

#[no_mangle]
pub unsafe extern "C" fn setCreateSysKeyboardDevice(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    // globals::set_keycode_for_mouse_click(addr.as_ptr());
    logln!(Information, "got CreateSysKeyboardDevice: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

static KEYBOARD_MANAGER_SINGLETON: OnceLock<Ptr<KeyboardManager>> = OnceLock::new();

#[riri_hook_fn(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 74 24 ?? 57 48 83 EC 20 48 8B F9 48 8B 49 ?? 48 85 C9 74 ?? 48 C7 47 ?? 00 00 00 00",
    resolve_type = setCreateSysKeyboardDevice,
    calling_convention = "microsoft",
)
)]
pub unsafe extern "C" fn CreateSysKeyboardDevice(p_instance: *mut u8) {
    let _ = original_function!(p_instance);
    if KEYBOARD_MANAGER_SINGLETON.get().is_none() {
        let _ = KEYBOARD_MANAGER_SINGLETON.set(Ptr::new(p_instance as *mut KeyboardManager));
        let pptr = unsafe { std::mem::transmute::<_, *mut *mut KeyboardManager>(KEYBOARD_MANAGER_SINGLETON.get().unwrap()) };
        logln!(Verbose, "Create hhKeyboardWin32 instance: 0x{:x} -> 0x{:x}", pptr as usize, p_instance as usize);
        unsafe { globals::set_keyboard_instance(pptr) };
    }
    if CONTROLLER_DATA_SINGLETON.get().is_none() {
        let mut ctrl = get_controller_data_instance();
        let _ = CONTROLLER_DATA_SINGLETON.set(Ptr::new(ctrl.as_mut().unwrap() as *mut ControllerPlatformManager));
        let pptr = unsafe { std::mem::transmute::<_, *mut *mut ControllerPlatformManager>(CONTROLLER_DATA_SINGLETON.get().unwrap()) };
        logln!(Verbose, "Create ControllerPlatformManager instance: 0x{:x} -> 0x{:x}", pptr as usize, &raw const *ctrl.as_ref().unwrap() as usize);
        unsafe { globals::set_pad_instance(pptr) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_device_pad_get_data(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v, None => return None
    };
    logln!(Information, "got gfdDevicePadGetData: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

static CONTROLLER_MAX: usize = 4;
static CONTROLLER_DATA: Mutex<Option<ControllerPlatformManager>> = Mutex::new(None);
static CONTROLLER_DATA_SINGLETON: OnceLock<Ptr<ControllerPlatformManager>> = OnceLock::new();

fn get_controller_data_instance() -> MutexGuard<'static, Option<ControllerPlatformManager>> {
    let mut ctrl = CONTROLLER_DATA.lock().unwrap();
    if ctrl.is_none() {
        *ctrl = Some(ControllerPlatformManager::new());
    }
    ctrl
}

fn set_controller_data(index: usize, value: &ControllerPlatform) {
    let mut ctrl = get_controller_data_instance();
    if index < CONTROLLER_MAX {
        ctrl.as_mut().unwrap()[index] = value.clone();
    }
}

#[riri_hook_fn(dynamic_offset(
    signature = "40 53 57 41 54 41 55 41 57",
    resolve_type = set_device_pad_get_data,
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDevicePadGetData(id: u32, p_result: *mut u8, a3: *mut u8) -> bool {
    let success = original_function!(id, p_result, a3);
    if success {
        let result = &mut *(p_result as *mut ControllerPlatform);
        set_controller_data(id as usize, result);
    }
    success
}