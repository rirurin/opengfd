use crate::globals;
use opengfd::device::ngr::structures::{ 
    ListNodeFreeList,
    PointerListEntry
};
use riri_mod_tools_proc::riri_hook_static;
use riri_mod_tools_rt::{
    logln, 
    sigscan_resolver 
};
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn set_ngr_allocator_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_allocator(addr.as_ptr() as 
        *mut *mut opengfd::device::ngr::allocator::Allocator);
    logln!(Information, "got ngrAllocator: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8B 0D ?? ?? ?? ?? 4C 8D 4C 24 ?? 48 89 44 24 ?? 41 B8 10 00 00 00 48 8D 05 ?? ?? ?? ?? C7 44 24 ?? 00 00 00 13 48 89 44 24 ?? 48 8D 05 ?? ?? ?? ?? C7 44 24 ?? 35 00 00 00",
    resolve_type = set_ngr_allocator_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_ALLOCATOR, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_window_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_window(addr.as_ptr() as 
        *mut *mut opengfd::device::ngr::renderer::platform::d3d::ngr_142ed6270);
    logln!(Information, "got ngrWindow: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 1.01 Demo: 48 8B 05 ?? ?? ?? ?? 48 8B 98 ?? ?? ?? ?? 8B 4E ??
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8B 05 ?? ?? ?? ?? 4C 8B B8 ?? ?? ?? ?? 8B 4E ??",
    resolve_type = set_ngr_window_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_WINDOW, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_crchash_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_crchash_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrCrcHash vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 02 89 4A ?? C7 44 24 ?? 01 00 00 00 48 8B CA E8 ?? ?? ?? ?? 0F B6 13 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ??",
    resolve_type = set_ngr_crchash_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_HASHER_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_memhint_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_memhint_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrMemHint vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}

// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static({
    XRD759_STEAM_1013 => dynamic_offset(
        signature = "48 8D 15 ?? ?? ?? ?? 48 89 54 24 ?? 44 8B 05 ?? ?? ?? ??",
        resolve_type = set_ngr_memhint_vtable_hook,
        calling_convention = "microsoft",
    ),
    XRD759_UWP_1011 => dynamic_offset(
        signature = "49 89 E3 48 81 EC 88 00 00 00 48 8D 05 ?? ?? ?? ?? 45 31 C9 49 89 43 ?? 48 8D 15 ?? ?? ?? ?? 49 8D 43 ?? 31 C9",
        resolve_type = set_ngr_memhint_vtable_hook,
        calling_convention = "microsoft",
    ),
    // Checked with Steam 1.02, Steam 1.011
    _ => dynamic_offset(
        signature = "48 8D 15 ?? ?? ?? ?? 48 89 54 24 ?? C7 44 24 ?? 00 00 00 01",
        resolve_type = set_ngr_memhint_vtable_hook,
        calling_convention = "microsoft",
    )
})]
riri_static!(NGR_MEMHINT_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_spinlock_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    // scary!!!
    let inner_fn = match sigscan_resolver::get_indirect_address_short(ofs) {
        Some(v) => v,
        None => return None
    };
    let inner2_fn = match sigscan_resolver::get_indirect_address_short_abs(inner_fn.add(0x2d).as_ptr()) {
        Some(v) => v,
        None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_long_abs(inner2_fn.add(0x21).as_ptr()) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_spinlock_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrSpinlock vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b061c, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "E8 ?? ?? ?? ?? 48 89 43 ?? 48 0F AF EF",
    resolve_type = set_ngr_spinlock_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_SPINLOCK_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_pointer_freelist_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v.add(1),
        None => return None
    };
    globals::set_ngr_pointer_freelist(addr.as_ptr() as 
        *mut *mut ListNodeFreeList<PointerListEntry<u8>>);
    logln!(Information, "got ngrFreeList pointer: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static({
    XRD759_STEAM_1011 => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 5D 89 D0 29",
        resolve_type = set_ngr_pointer_freelist_hook,
        calling_convention = "microsoft",
    ),
    XRD759_UWP_1011 => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? 8B 0D ?? ?? ?? ??",
        resolve_type = set_ngr_pointer_freelist_hook,
        calling_convention = "microsoft",
    ),
    XRD759_STEAM_1013 => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 10 00 00 00 E8 ?? ?? ?? ?? 48 89 C3",
        resolve_type = set_ngr_pointer_freelist_hook,
        calling_convention = "microsoft",
    ),
    // Checked with Steam 1.02
    _ => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 9D 64 24 08",
        resolve_type = set_ngr_pointer_freelist_hook,
        calling_convention = "microsoft",
    )
})]
riri_static!(NGR_POINTER_FREELIST, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_string_freelist_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v.add(1),
        None => return None
    };
    globals::set_ngr_string_freelist(addr.as_ptr() as 
        *mut *mut ListNodeFreeList<u8>);
    logln!(Information, "got ngrFreeList string: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static({
    XRD759_STEAM_1011 => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? 8B 0D ?? ?? ?? ??",
        resolve_type = set_ngr_string_freelist_hook,
        calling_convention = "microsoft",
    ),
    // Checked with Steam 1.011 and UWP 1.011
    _ => dynamic_offset(
        signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 10 00 00 00 E8 ?? ?? ?? ?? 48 89 44 24 ??",
        resolve_type = set_ngr_string_freelist_hook,
        calling_convention = "microsoft",
    )
})]
riri_static!(NGR_STRING_FREELIST, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_freelist_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_freelist_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngrFreeList vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 48 89 4B ?? 44 89 6B ??",
    resolve_type = set_ngr_freelist_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_FREELIST_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_1422ecad8_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_1422ecad8_vtable(addr.as_ptr() as 
        *mut u8);
    logln!(Information, "got ngr1422ecad8 vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    // Checked with Steam 1.02, Steam 1.011 and UWP 1.011
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 01 48 8B D9 48 89 79 ?? 48 89 79 ??",
    resolve_type = set_ngr_1422ecad8_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_1422ECAD8_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_texture_resource_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    globals::set_ngr_texture_resource_vtable(addr.as_ptr() as *mut u8);
    logln!(Information, "got ngrTextureResource vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 4C 89 6B ?? 4C 89 6B ??",
    resolve_type = set_ngr_texture_resource_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_TEXTURE_RESOURCE_HOOK, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_string_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    globals::set_ngr_string_vtable(addr.as_ptr() as *mut u8);
    logln!(Information, "got ngrString vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 01 48 8D 2D ?? ?? ?? ?? 48 89 69 ?? 41 8B 40 ?? 89 41 ?? 48 85 D2 74 ?? 48 8B CA",
    resolve_type = set_ngr_string_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_STRING_VTABLE_HOOK, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_string_hash_vtable_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v, None => return None
    };
    globals::set_ngr_string_hash_vtable(addr.as_ptr() as *mut u8);
    logln!(Information, "got ngrStringHash vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    signature = "48 8D 3D ?? ?? ?? ?? EB ?? 31 DB",
    // 48 8D 3D ?? ?? ?? ?? EB ?? 33 DB
    resolve_type = set_ngr_string_hash_vtable_hook,
    calling_convention = "microsoft",
))]
riri_static!(NGR_STRING_HASH_VTABLE_HOOK, usize);