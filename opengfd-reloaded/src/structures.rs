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

/*

#[riri_hook_fn(static_offset(0x1105890))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdFreeListCreate(
    entry_size: u32, entries_per_block: u32,
    alignment: u32, prealloc_blocks: u32,
    _place_space_for_handle: *mut u8, hint: u32
    ) -> *mut u8 {
    let out = opengfd::utility::free_list::FreeList::<u8, GfdAllocator>::new_inner_untyped(
        entry_size, alignment, entries_per_block, prealloc_blocks, hint, GfdAllocator);
    out.link();
    logln!(Debug, "handle: 0x{:x}, entry size: {}, entries_per_block: {}, alignment: {}, prealloc: {}, hint: 0x{:x}",
        &raw const *out as usize, entry_size, entries_per_block, alignment, prealloc_blocks, hint
    );
    &raw mut *out as *mut u8
}

#[riri_hook_fn(static_offset(0x1105b70))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdFreeListAllocCore(handle: *mut u8) -> *mut u8 {
    let free_list = &mut *(handle as *mut opengfd::utility::free_list::FreeList<u8, GfdAllocator>);
    free_list.add()
}

#[riri_hook_fn(static_offset(0x1105cf0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdFreeListFree(list: *mut u8, entry: *mut u8) {
    // let free_list = &mut *(handle as *mut opengfd::utility::free_list::FreeList<u8, GfdAllocator>);
    // free_list.remove(entry)
    original_function!(list, entry)
}

#[riri_hook_fn(static_offset(0x74c0f40))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn fldMainCreateTask(parent: *mut u8, seq_set: *mut u8) -> *mut u8 {
    // GfdTask::<GfdAllocator, u8>::print_running_tasks();
    // logln!(Debug, "fldMain!");
    original_function!(parent, seq_set)
}

#[riri_hook_fn(static_offset(0x1b524470))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdTaskAttachUpdateList(p_task: *mut u8) {
    let task = &mut *(p_task as *mut GfdTask<GfdAllocator>);
    logln!(Debug, "Attach Update for {}", task);
    task.attach_to_update_list();
    // let _ = original_function!(p_task);
}

#[riri_hook_fn(static_offset(0x1b521310))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdTaskAttachRenderList(p_task: *mut u8) {
    let task = &mut *(p_task as *mut GfdTask<GfdAllocator>);
    task.attach_to_render_list();
    // let _ = original_function!(p_task);
}
*/