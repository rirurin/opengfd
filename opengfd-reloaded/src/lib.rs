#![allow(unused_imports)]
use allocator_api2::alloc::Allocator;
use opengfd::{
    device::ngr::{
        allocator::AllocatorHook,
        structures::{ 
            CrcHash,
            ListNodeFreeList,
            FreeList,
            FreeListBlockLink,
            PointerListEntry
        },
        renderer::{
            cbuffer::{ BufferType, ConstantBuffer },
            platform::d3d::{ 
                ngrDX11Renderer,
                PlatformCmdBuffer,
                TextureResource,
                ResourceView,
                ResourceView2
            },
            pkt::BlendModePkt,
            shader::{
                PixelShader,
                PixelShaderPlatform,
                ShaderPlatform,
                VertexShader,
                VertexShaderPlatform
            },
            state::{
                BlendState,
                BufferFlags,
                BufferFlags2,
                CullMode,
                DeferredContext, 
                DeferredContextBase,
                DeferredContextDX11,
                DeferredContextResources,
                DepthStencilState,
                DepthWriteMask,
                DrawState,
                IndexBuffer,
                RasterizerKey, 
                RasterizerState,
                SamplerState,
                VertexBuffer
            }, 
        }
    },
    globals,
    graphics::{
        draw2d::{ Draw, Im2DVertexG4 },
        render::{
            cmd_buffer::CmdBuffer,
            render::Render
        },
        render_ot::{ RenderOt, RenderOtBase, RenderOtEx },
        texture::Texture
    },
    utility::{
        misc::RGBA,
        reference::{ GfdRc, GfdRcType }
    }
};
use windows::{
    core::Interface,
    Win32::Graphics::Direct3D11::{ 
        D3D11_VIEWPORT,
        ID3D11Buffer,
        ID3D11PixelShader, 
        ID3D11VertexShader,
        ID3D11Resource
    }
};
use riri_mod_tools_proc::{ ensure_layout, original_function, riri_hook_fn, riri_hook_static };
use riri_mod_tools_rt::{ logln, sigscan_resolver };

#[no_mangle]
pub unsafe extern "C" fn set_gfd_global(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match riri_mod_tools_rt::sigscan_resolver::get_indirect_address_short2(ofs) {
        Some(v) => v,
        None => return None
    };
    let out = addr.sub(20);
    logln!(Information, "got gfdGlobal: 0x{:x}", out.as_ptr() as usize);
    globals::set_gfd_global(out.as_ptr() as *mut opengfd::kernel::global::Global);
    Some(out)
}

#[riri_hook_static(dynamic_offset(
    signature = "F7 05 ?? ?? ?? ?? 00 00 00 02",
    resolve_type = set_gfd_global,
    calling_convention = "microsoft",
))]
riri_static!(GFD_GLOBAL, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_draw_state(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 83 EC 28 E8 ?? ?? ?? ?? B9 E0 17 00 00",
    resolve_type = set_ngr_draw_state,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DRAWSTATE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_allocator(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8B 0D ?? ?? ?? ?? 4C 8D 4C 24 ?? 48 89 44 24 ?? 41 B8 10 00 00 00 48 8D 05 ?? ?? ?? ?? C7 44 24 ?? 00 00 00 13 48 89 44 24 ?? 48 8D 05 ?? ?? ?? ?? C7 44 24 ?? 35 00 00 00",
    resolve_type = set_ngr_allocator,
    calling_convention = "microsoft",
))]
riri_static!(NGR_ALLOCATOR, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_window(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8B 05 ?? ?? ?? ?? 4C 8B B8 ?? ?? ?? ?? 8B 4E ??",
    resolve_type = set_ngr_window,
    calling_convention = "microsoft",
))]
riri_static!(NGR_WINDOW, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_crchash_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 02 89 4A ?? C7 44 24 ?? 01 00 00 00 48 8B CA E8 ?? ?? ?? ?? 0F B6 13 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ?? 48 8B CF E8 ?? ?? ?? ?? 0F B6 53 ??",
    resolve_type = set_ngr_crchash_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_HASHER_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_dx11_renderer(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    globals::set_ngr_dx11_renderer(addr.as_ptr() as 
        *mut *mut opengfd::device::ngr::renderer::platform::d3d::ngrDX11Renderer);
    logln!(Information, "got ngr DX11 renderer: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    signature = "48 8B 0D ?? ?? ?? ?? 48 8D 15 ?? ?? ?? ?? 44 8B 1D ?? ?? ?? ??",
    resolve_type = set_ngr_dx11_renderer,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DX11_RENDERER, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_rasterstate_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 8B 0E 89 4B ?? 8B 46 ?? 89 43 ?? 0F B6 46 ??",
    resolve_type = set_ngr_rasterstate_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_RASTERSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_blendstate_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_blendstate_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_BLENDSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_depthstencilstate_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_depthstencilstate_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_DEPTHSTENCILSTATE_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_sampler_state(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 0F B6 0E 88 4B ?? 8B 46 ?? 89 43 ?? 8B 46 ?? 89 43 ?? 8B 46 ??",
    resolve_type = set_ngr_sampler_state,
    calling_convention = "microsoft",
))]
riri_static!(NGR_SAMPLER_STATE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_memhint_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
#[riri_hook_static(dynamic_offset(
    signature = "48 8D 15 ?? ?? ?? ?? 48 89 54 24 ?? C7 44 24 ?? 00 00 00 01",
    resolve_type = set_ngr_memhint_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_MEMHINT_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_spinlock_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "E8 ?? ?? ?? ?? 48 89 43 ?? 48 0F AF EF",
    resolve_type = set_ngr_spinlock_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_SPINLOCK_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_pointer_freelist(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
#[riri_hook_static(dynamic_offset(
    signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 9D 64 24 08",
    resolve_type = set_ngr_pointer_freelist,
    calling_convention = "microsoft",
))]
riri_static!(NGR_POINTER_FREELIST, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_string_freelist(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v.add(1),
        None => return None
    };
    globals::set_ngr_string_freelist(addr.as_ptr() as 
        *mut *mut ListNodeFreeList<u8>);
    logln!(Information, "got ngrFreeList string: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    signature = "48 83 3D ?? ?? ?? ?? 00 0F 85 ?? ?? ?? ?? B9 10 00 00 00 E8 ?? ?? ?? ?? 48 89 44 24 ??",
    resolve_type = set_ngr_string_freelist,
    calling_convention = "microsoft",
))]
riri_static!(NGR_STRING_FREELIST, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_freelist_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 03 48 89 4B ?? 44 89 6B ??",
    resolve_type = set_ngr_freelist_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_FREELIST_VTABLE, usize);

#[no_mangle]
pub unsafe extern "C" fn set_ngr_1422ecad8_vtable(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
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
    signature = "48 8D 05 ?? ?? ?? ?? 48 89 01 48 8B D9 48 89 79 ?? 48 89 79 ??",
    resolve_type = set_ngr_1422ecad8_vtable,
    calling_convention = "microsoft",
))]
riri_static!(NGR_1422ECAD8_VTABLE, usize);
/*
#[riri_hook_fn(static_offset(0x1192e20))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetVertexProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    let ctx = &mut *(p_ctx as *mut DeferredContextBase);
    let shader = if p_shader.is_null() { None } else { Some(&*(p_shader as *const VertexShaderPlatform)) };
    ctx.set_vertex_shader(shader);
}

#[riri_hook_fn(static_offset(0x1192ee0))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetPixelProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    let ctx = &mut *(p_ctx as *mut DeferredContextBase);
    let shader = if p_shader.is_null() { None } else { Some(&*(p_shader as *const PixelShaderPlatform)) };
    ctx.set_pixel_shader(shader);
}
*/
#[riri_hook_fn(static_offset(0x1072960))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStateSetHook(prio: u32, state: u32, value: *mut u8) {
    Render::set_state(prio, state, value);
}

#[riri_hook_fn(static_offset(0x1072a50))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePopHook(prio: u32, state: u32) {
    Render::pop_state(prio, state);
}

#[riri_hook_fn(static_offset(0x10729e0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePushHook(prio: u32, state: u32) {
    Render::push_state(prio, state);
}
#[riri_hook_fn(static_offset(0x1072d40))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetBlendModeHook(prio: u32, blend: u32) {
    Render::set_blend_mode(prio, blend);
}
/*
#[riri_hook_fn(static_offset(0x1101030))]
#[allow(non_snake_case)] // Major graphical issues (still!)
pub unsafe extern "C" fn gfdShaderVertexBindHook(prio: u32, vertex: *mut u8) {
    Render::bind_vertex_shader(prio, &*(vertex as *mut VertexShader));
}
*/
// #[riri_hook_fn(static_offset(0x10e36c0))]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn gfdRenderOtLink(prio: u32, ot: *mut u8) {
//     let ot = &mut *(ot as *mut RenderOt);
//     ot.link(prio);
// }
#[riri_hook_fn(static_offset(0x11947b0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetRasterizerStateInner(p_renderer: *mut u8, raster_key: *mut u8) -> *mut u8 {
    let renderer = &mut *(p_renderer as *mut ngrDX11Renderer);
    let key = &mut *(raster_key as *mut RasterizerKey);
    let hash = CrcHash::new(key);
    match renderer.rasterizers.find_by_predicate(|f| f == key && f == &hash) {
        Some(v) => (&raw const *v) as *mut RasterizerState as *mut u8,
        None => std::ptr::null_mut()
    }
}

#[riri_hook_fn(static_offset(0x11b56d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetRasterizer(p_state: *mut *mut u8, p_key: *mut u8) -> *mut *mut u8 {
    let state = &mut *(p_state as *mut *mut RasterizerState);
    let renderer = globals::get_ngr_dx11_renderer_unchecked_mut();
    let key = &mut *(p_key as *mut RasterizerKey);
    *state = GfdRc::into_raw(renderer.get_or_create_rasterizer(key)) as *mut RasterizerState;
    p_state
}
#[riri_hook_fn(static_offset(0x11ba3c0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrFreeListCreate(
    p_list: *mut *const u8, 
    element_size: usize, 
    entries_per_block: usize,
    alignment: usize,
    block_count: usize,
    a6: i32,
    p_hint: *const u8
) -> *mut *const u8 {
    let new_list: GfdRc<FreeList<FreeListBlockLink<u8>>, AllocatorHook> = 
        FreeList::new_inner(alignment, entries_per_block, block_count, a6, AllocatorHook);
    let p_newlist = GfdRc::into_raw(new_list);
    *p_list = p_newlist as *const u8;
    p_list
}

#[riri_hook_fn(static_offset(0x1192f80))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetConstantBuffers(p_this: *mut u8, p_ty: u32, p_buf: *mut u8, upd: u32) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buf = &mut *(p_buf as *mut ConstantBuffer);
    this.set_constant_buffers(ty, buf, upd);
}

#[riri_hook_fn(static_offset(0x1193240))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextDraw(p_this: *mut u8, ia_topo: u32, vtx_count: u32, vtx_start: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let topo = ia_topo.try_into().unwrap();
    this.draw(topo, vtx_count, vtx_start);
}

#[riri_hook_fn(static_offset(0x109a590))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetBlendKeyPreset2(buf_id: u32, blend_id: u32, set_blend_key: bool) {
    opengfd::device::ngr::renderer::pkt::set_blend_key_preset2(buf_id as usize, blend_id as usize, set_blend_key);
}

#[riri_hook_fn(static_offset(0x109a4f0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetDepthStencilKeyLessEqual(buf_id: u32, set_depth_stencil: bool, set_depth_write_mask: bool) {
    opengfd::device::ngr::renderer::render::set_depth_stencil_key_less_equal(
        buf_id as usize, set_depth_stencil, set_depth_write_mask.try_into().unwrap()
    );
}

#[riri_hook_fn(static_offset(0x109a6e0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetSamplerKeyValues(buf_id: u32, sampler_id: u32, a3: bool, a4: bool, addru: u8, addrv: u8) {
    opengfd::device::ngr::renderer::render::set_sampler_key_values(
        buf_id as usize, sampler_id as usize, a3, a4, addru.try_into().unwrap(), addrv.try_into().unwrap()
    );
}

#[riri_hook_fn(static_offset(0x1094060))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetEffectScaleWidth() -> usize {
    globals::get_ngr_dx11_renderer_unchecked().get_effect_scale_width()
}

#[riri_hook_fn(static_offset(0x10940d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetEffectScaleHeight() -> usize {
    globals::get_ngr_dx11_renderer_unchecked().get_effect_scale_height()
}

#[riri_hook_fn(static_offset(0x1192ae0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetShaderSampler(p_this: *mut u8, p_ty: u32, p_id: u32, p_state: *mut u8) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = if p_state.is_null() { None } else { Some(&*(p_state as *mut SamplerState)) };
    this.set_shader_sample(ty, p_id as usize, state);
}

#[riri_hook_fn(static_offset(0x1193130))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetShaderResources(p_this: *mut u8, p_ty: u32, p_id: u32, p_state: *mut u8) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = if p_state.is_null() { None } else { Some(&*(p_state as *mut TextureResource)) };
    this.set_shader_resource_view(ty, p_id as usize, state);
}
/*
#[riri_hook_fn(static_offset(0x1192530))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetRenderTargets(p_this: *mut u8, p_rv: *const u8, p_rv2: *const u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let rv = if !p_rv.is_null() { Some(&*(p_rv as *const ResourceView)) } else { None };
    let rv2 = if !p_rv.is_null() { Some(&*(p_rv2 as *const ResourceView2)) } else { None };
    // original_function!(p_this, p_rv, p_rv2)
    this.om_set_render_targets(rv, rv2);
}
*/
#[riri_hook_fn(static_offset(0x1192340))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetViewports(p_this: *mut u8, viewports: *const f32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let viewport = &*(viewports as *const D3D11_VIEWPORT);
    this.set_viewports(viewport);
}

#[riri_hook_fn(static_offset(0x1192680))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrClearRenderTargetDepthStencil(p_this: *mut u8, flags: u32, p_colors: *const f32, depth: f32, stencil: bool) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let colors = std::mem::transmute::<*const f32, &[f32; 4]>(p_colors);
    this.clear_depth_stencil_or_render_target_view(flags, colors, depth, stencil);
}

#[riri_hook_fn(static_offset(0x11929d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetBlendState(p_this: *mut u8, p_state: *const u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = &*(p_state as *const BlendState);
    this.om_clear_blend_state(state);
}

#[riri_hook_fn(static_offset(0x1192a20))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrOMSetDepthStencilState(p_this: *mut u8, p_state: *const u8, stencil_ref: u8) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let state = &*(p_state as *const DepthStencilState);
    this.om_depth_stencil_state(state, stencil_ref);
}

#[riri_hook_fn(static_offset(0x1192bf0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetVertexBuffers(
    p_this: *mut u8, start_slot: u32, p_buffer: *mut u8,
    a4: usize, stride: u32, offset: u32, buffer_index: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buffer = if p_buffer.is_null() { None } else { Some(&*(p_buffer as *const VertexBuffer)) };
    this.set_vertex_buffers(start_slot, buffer, a4, stride, offset, buffer_index as usize);
}
#[riri_hook_fn(static_offset(0x1192cb0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrDeferredContextSetIndexBuffer(
    p_this: *mut u8, p_buffer: *mut u8, offset: u32, buffer_index: u32) {
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buffer = if p_buffer.is_null() { None } else { Some(&*(p_buffer as *const IndexBuffer)) };
    this.set_index_buffer(buffer, offset, buffer_index as usize);
}

#[riri_hook_fn(dynamic_offset(
    signature = "48 8B C4 53 48 83 EC 70 80 3D ?? ?? ?? ?? 00",
    calling_convention = "microsoft"
))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdPostRender(a1: u32, a2: u32) {
    // line draw test
    opengfd::debug::perf_meter::draw_test();
    original_function!(a1, a2)
}

/*
#[riri_hook_fn(static_offset(0x112ad60))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrUpdateVertexBuffers(p_this: *mut u8, buffer_index: usize) {
    let this = &mut *(p_this as *mut DrawState);
    this.update_vertex_buffers(buffer_index);
}
*/
/* crashes
#[riri_hook_fn(static_offset(0x1072ac0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderTextureSet(prio: u32, stage: u32, tex: *mut u8) {
    Render::set_texture(prio, stage, &*(tex as *mut Texture));
}
*/

/*
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdIm2DRenderPrimitive2D(prio: u32, prim: u8, data: usize, num: u32, flags: u32) {
    original_function!(prio. prim, data, num, flags)
}
*/
/*
#[riri_hook_fn(static_offset(0x1109580))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDevCmdMakeImmediateRenderPrimitivePkt(a1: usize, a2: u8, a3: u32, a4: u32, a5: u32) {
    original_function!(a1, a2, a3, a4, a5)
}
*/
/*
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetTransformWorldOtPreCallback
(_render_ot: *mut RenderOt, buffer: *mut u8, data: *mut u8) {
    let buf_id = buffer as i32;
    let data = data as *mut glam::Mat4;
    
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetTransformWorld(prio: u32, v: *mut u8) {
    let v = v as *mut glam::Mat4;
    let ot = RenderOtEx::<64>::new();
    ot.set::<glam::Mat4>(0, *v).unwrap();
    ot.set_pre_cb(gfdRenderSetTransformWorldOtPreCallback);
    ot.set_pre_cb_data(ot.data_raw());
    ot.link(prio);
}
*/
