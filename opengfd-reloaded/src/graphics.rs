use crate::globals;
use opengfd::{
    device::ngr::renderer::platform::d3d::ngrDX11Renderer,
    kernel::global::Global
};
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{
    logln, 
    sigscan_resolver 
};
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn set_gfd_global_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match riri_mod_tools_rt::sigscan_resolver::get_indirect_address_short2(ofs) {
        Some(v) => v,
        None => return None
    };
    let out = addr.sub(20);
    logln!(Information, "got gfdGlobal: 0x{:x}", out.as_ptr() as usize);
    globals::set_gfd_global(out.as_ptr() as *mut Global);
    Some(out)
}

#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "F7 05 ?? ?? ?? ?? 00 00 00 02",
    resolve_type = set_gfd_global_hook,
    calling_convention = "microsoft",
))]
riri_static!(GFD_GLOBAL, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_draw_state_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let ngr_init_state = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_long_abs(ngr_init_state.add(0x26).as_ptr()) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_draw_state(addr.as_ptr() as 
        *mut *mut opengfd::device::ngr::renderer::state::DrawState);
    logln!(Information, "got ngrDrawState: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 83 EC 28 E8 ?? ?? ?? ?? B9 E0 17 00 00",
    resolve_type = set_ngr_draw_state_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DRAWSTATE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_dx11_renderer_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_dx11_renderer(addr.as_ptr() as *mut *mut ngrDX11Renderer);
    logln!(Information, "got ngr DX11 renderer: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8B 0D ?? ?? ?? ?? 48 8D 15 ?? ?? ?? ?? 44 8B 1D ?? ?? ?? ??",
    resolve_type = set_ngr_dx11_renderer_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DX11_RENDERER, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_rasterstate_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs + 0x64) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_rasterstate_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrRasterState vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 8B 0E 89 4B ?? 8B 46 ?? 89 43 ?? 0F B6 46 ??",
    resolve_type = set_ngr_rasterstate_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_RASTERSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_blendstate_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs + 0x41) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_blendstate_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrBlendState vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // TODO: Fix this! 
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_blendstate_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_BLENDSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_depthstencilstate_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs + 0x82) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_depthstencilstate_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrDepthStencilState vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // TODO: Fix this!
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_depthstencilstate_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DEPTHSTENCILSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_sampler_state_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs + 0x66) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_sampler_state(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrDepthStencilState vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // TODO: Fix this!
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_sampler_state_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_SAMPLER_STATE, usize);