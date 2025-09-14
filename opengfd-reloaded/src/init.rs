use crate::globals;
use opengfd::{
    kernel::job::Job,
    platform::utils::PlatformInfo
};
use riri_mod_tools_proc::{ riri_hook_fn, riri_hook_static, riri_init_fn };
use riri_mod_tools_rt::{
    address::get_thread_id,
    logln, 
    sigscan_resolver 
};
use std::{
    ptr::NonNull,
    sync::OnceLock
};

#[no_mangle]
pub unsafe extern "C" fn set_gfd_job_list_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_job_list(addr.as_ptr() as *mut *mut Job);
    logln!(Information, "got gfdJobList: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static({
    XRD759_UWP_1011 => dynamic_offset(
        signature = "4C 8D 35 ?? ?? ?? ?? 4C 89 7C 24 ?? 41 BF 00 00 4D 8F",
        resolve_type = set_gfd_job_list_hook,
        calling_convention = "microsoft",
    ),
    _ => dynamic_offset(
        signature = "4C 8D 35 ?? ?? ?? ?? 4C 89 7C 24 ?? 41 BF 00 00 00 80",
        resolve_type = set_gfd_job_list_hook,
        calling_convention = "microsoft",
    )
})]
riri_static!(GFD_JOB_LIST, usize);

#[no_mangle]
pub unsafe extern "C" fn set_gfd_win32_start(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got gfdWin32Start: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

static MAIN_THREAD_ID: OnceLock<u32> = OnceLock::new();

#[riri_hook_fn(dynamic_offset(
    signature = "40 55 53 57 48 8D AC 24 ?? ?? ?? ?? 48 81 EC 90 02 00 00",
    resolve_type = set_gfd_win32_start,
    calling_convention = "microsoft",
))]
pub unsafe extern "C" fn gfd_win32_start(p_exec: *const u8) -> bool {
    let _ = MAIN_THREAD_ID.set(get_thread_id() as u32);
    logln!(Verbose, "Main thread is {}", MAIN_THREAD_ID.get().unwrap());
    globals::set_main_thread_id(&raw const *MAIN_THREAD_ID.get().unwrap() as *mut u32);
    original_function!(p_exec)
}

#[riri_init_fn()]
fn init_set_steam_check() {
    PlatformInfo::set_platform_steam();
}