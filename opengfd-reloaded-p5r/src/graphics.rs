use std::ptr::NonNull;
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::logln;
use opengfd::kernel::global::Global;
use crate::globals;

#[no_mangle]
pub unsafe extern "C" fn set_gfd_global_hook(ofs: usize) -> Option<NonNull<u8>> {
    let addr = match riri_mod_tools_rt::sigscan_resolver::get_indirect_address_short2(ofs) {
        Some(v) => v,
        None => return None
    };
    let out = addr.sub(44);
    logln!(Information, "got gfdGlobal: 0x{:x}", out.as_ptr() as usize);
    globals::set_gfd_global(out.as_ptr() as *mut Global);
    Some(out)
}

#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.04
    signature = "F7 05 ?? ?? ?? ?? 00 00 00 02",
    resolve_type = set_gfd_global_hook,
    calling_convention = "microsoft",
))]
riri_static!(GFD_GLOBAL, usize);