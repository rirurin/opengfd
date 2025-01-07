#![allow(unused_imports)]
use allocator_api2::alloc::Allocator;
use opengfd::{
    device::ngr::{
        structures::CrcHash,
        renderer::{
            blend::BlendModePkt,
            cbuffer::ConstantBuffer,
            platform::d3d::ngrDX11Renderer,
            ps::PixelShader, 
            state::{ 
                DeferredContext, DeferredContextBase, DeferredContextDX11,
                RasterizerKey, RasterizerState
            }, 
            vs::{ VertexShader, VertexShaderPlatform }
        }
    },
    globals,
    graphics::{
        render::cmd_buffer::CmdBuffer, 
        render_ot::{ RenderOt, RenderOtBase, RenderOtEx }
    },
    utility::reference::{ GfdRc, GfdRcType }
};
use windows::{
    core::Interface,
    Win32::Graphics::Direct3D11::{ 
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
    let addr = match sigscan_resolver::get_indirect_address_long(ofs + 0x66) {
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

/*
#[riri_hook_fn(static_offset(0x1192e20))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetVertexProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    if !p_shader.is_null() { 
        let ctx = (p_ctx as *mut DeferredContextDX11).as_mut().unwrap();
        let shader = (p_shader as *mut VertexShaderPlatform).as_ref().unwrap();
        // compare by-reference (equality operator checks by-value, following PartialEq/Eq impl)
        if std::ptr::eq(&shader.d3d_vertex, &ctx.super_.target_vertex_shader) { return; }
        let device_ctx = &ctx.super_.device_context;
        device_ctx.IASetInputLayout(&shader.d3d_input_layout);
        device_ctx.VSSetShader(&shader.d3d_vertex, None);
        *&mut &ctx.super_.target_vertex_shader = &shader.d3d_vertex;
    }
}

#[allow(non_snake_case)] // Verified
pub unsafe fn ngrSetVertexProgramLoad(ctx: &mut DeferredContextDX11, shader: &VertexShaderPlatform) {
    // logln!(Information, "ctx: 0x{:x}, shader: 0x{:x}", &raw const ctx as usize, &raw const shader as usize);
    // compare by-reference (equality operator checks by-value, following PartialEq/Eq impl)
    if std::ptr::eq(&shader.d3d_vertex, &ctx.super_.target_vertex_shader) { return; }
    let device_ctx = &ctx.super_.device_context;
    device_ctx.IASetInputLayout(&shader.d3d_input_layout);
    device_ctx.VSSetShader(&shader.d3d_vertex, None);
    *&mut &ctx.super_.target_vertex_shader = &shader.d3d_vertex;
}

#[allow(non_snake_case)] // Verified
pub unsafe fn gfdDeviceShaderVertexBind(id: i32, shader: &VertexShaderPlatform) {
    let draw = globals::get_ngr_draw_state_unchecked();
    ngrSetVertexProgramLoad(
        &mut **draw.basicBuffers.get_unchecked(id as usize).deferredContexts.get_unchecked(draw.otFrameId as usize),
        shader
    );
}
*/
/*

#[riri_hook_fn(static_offset(0x114aec0))]
#[allow(non_snake_case)] // Completely broken
pub unsafe extern "C" fn gfdDeviceShaderVertexBindHook(id: i32, shader: *mut u8) {
    let shader = &*(shader as *mut VertexShaderPlatform);
    let draw = globals::get_ngr_draw_state_unchecked().as_mut();
    ngrSetVertexProgramLoad(
        &mut **draw.basicBuffers.get_unchecked(id as usize).deferredContexts.get_unchecked(draw.otFrameId as usize),
        shader
    );
}
*/
/*
#[riri_hook_fn(static_offset(0x1103150))]
#[allow(non_snake_case)] // Major graphical issues
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallbackHook(_ot: *mut u8, id: i32, data: *mut u8) {
    let current = &mut (&mut *globals::get_gfd_global()).graphics.shader_current_vertex;
    let data = (data as *mut VertexShader).as_ref();
    if std::ptr::eq(
        std::mem::transmute::<Option<&VertexShader>, *const VertexShader>(data), 
        *current
    ) { return; }
    *current = std::mem::transmute::<Option<&VertexShader>, *mut VertexShader>(data);
    if let Some(shader) = data {
        if !shader.data.is_null() {
            gfdDeviceShaderVertexBind(id, shader.data.as_ref().unwrap());
        }
    }
}
*/
/*
#[riri_hook_fn(static_offset(0x1103150))]
#[allow(non_snake_case)] // Major graphical issues
// Glow materials are completely cooked
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallbackHook(_ot: *mut u8, id: i32, data: *mut u8) {
    let current = &mut (globals::get_gfd_global_unchecked().graphics.shader_current_vertex as *const VertexShader);
    let data = (data as *mut VertexShader).as_ref();
    if std::ptr::eq(
        std::mem::transmute::<Option<&VertexShader>, *const VertexShader>(data), 
        *current
    ) { return; }
    *current = std::mem::transmute::<Option<&VertexShader>, *mut VertexShader>(data);
    if let Some(shader) = data {
        if !shader.data.is_null() {
            gfdDeviceShaderVertexBind(id, shader.data.as_ref().unwrap());
        }
    }
}
*/

#[allow(non_snake_case)] // Verified (currently broken lmao)
pub unsafe fn ngrSetPixelProgramLoad(ctx: &mut DeferredContextDX11, shader: Option<&VertexShaderPlatform>) {
    let shader = match shader {
        Some(v) => match v.get_d3d_pixel() {
            Some(v) => Some(v),
            None => None
        },
        None => None
    };
    if std::ptr::eq(
        std::mem::transmute::<Option<&ID3D11PixelShader>, *const ID3D11PixelShader>(shader), 
        &raw const ctx.super_.target_pixel_shader
    ) { return; }
    let device_ctx = &ctx.super_.device_context;
    device_ctx.PSSetShader(shader, None);
    match shader {
        Some(v) => *&mut &ctx.super_.target_pixel_shader = v,
        None => *&mut &raw const ctx.super_.target_pixel_shader = std::ptr::null()
    }
}

#[allow(non_snake_case)] // Verified
pub unsafe fn gfdDeviceShaderPixelBind(id: i32, shader: Option<&VertexShaderPlatform>) {
    let draw = globals::get_ngr_draw_state_unchecked();
    ngrSetPixelProgramLoad(
        &mut **draw.basicBuffers.get_unchecked(id as usize).deferredContexts.get_unchecked(draw.otFrameId as usize),
        shader
    );
}
/*
#[riri_hook_fn(static_offset(0x11031c0))]
#[allow(non_snake_case)] // Some minor graphical issues:
// - Gallica doesn't appear properly when running the loading screen for the first time
pub unsafe extern "C" fn gfdShaderPixelBindOtPreCallbackHook(_ot: *mut u8, id: i32, p_data: *mut u8) {
    let shader_current_fragment = &mut (globals::get_gfd_global_unchecked().graphics.shader_current_fragment as *const PixelShader);
    let data = (p_data as *mut PixelShader).as_ref();
    if std::ptr::eq(
        std::mem::transmute::<Option<&PixelShader>, *const PixelShader>(data), 
        *shader_current_fragment
    ) { return; }
    *shader_current_fragment = std::mem::transmute::<Option<&PixelShader>, *mut PixelShader>(data);
    if let Some(shader) = data {
        if !shader.data.is_null() {
            gfdDeviceShaderPixelBind(id, shader.data.as_ref());
        }
    }
}
*/

#[allow(non_snake_case)] // Some minor graphical issues:
// - Gallica doesn't appear properly when running the loading screen for the first time
pub unsafe extern "C" fn gfdShaderPixelBindOtPreCallback(_ot: *mut RenderOt, id: *mut u8, p_data: *mut u8) {
    let id = id as i32;
    let shader_current_fragment = &mut (globals::get_gfd_global_unchecked().graphics.shader_current_fragment as *const PixelShader);
    let data = (p_data as *mut PixelShader).as_ref();
    if std::ptr::eq(
        std::mem::transmute::<Option<&PixelShader>, *const PixelShader>(data), 
        *shader_current_fragment
    ) { return; }
    *shader_current_fragment = std::mem::transmute::<Option<&PixelShader>, *mut PixelShader>(data);
    if let Some(shader) = data {
        if !shader.data.is_null() {
            gfdDeviceShaderPixelBind(id, shader.data.as_ref());
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum RenderStateTable {
    PS3RS_ZENABLE = 1,
    PS3RS_FILLMODE,
    PS3RS_ZWRITEENABLE,
    PS3RS_ALPHATESTENABLE,
    PS3RS_CULLMODE,
    PS3RS_ZFUNC,
    PS3RS_ALPHAREF,
    PS3RS_ALPHAFUNC,
    PS3RS_ALPHABLENDENABLE,
    PS3RS_STENCILENABLE,
    PS3RS_STENCILFAIL,
    PS3RS_STENCILZFAIL,
    PS3RS_STENCILPASS,
    PS3RS_STENCILFUNC,
    PS3RS_STENCILREF,
    PS3RS_STENCILMASK,
    PS3RS_STENCILWRITEMASK,
    PS3RS_WRAP0,
    PS3RS_WRAP1,
    PS3RS_WRAP2,
    PS3RS_WRAP3,
    PS3RS_WRAP4,
    PS3RS_WRAP5,
    PS3RS_WRAP6,
    PS3RS_WRAP7,
    PS3RS_POINTSIZE,
    PS3RS_POINTSIZE_MIN,
    PS3RS_POINTSPRITEENABLE,
    PS3RS_MULTISAMPLEANTIALIAS,
    PS3RS_MULTISAMPLEMASK,
    PS3RS_POINTSIZE_MAX,
    PS3RS_COLORWRITEENABLE,
}

impl TryFrom<u32> for RenderStateTable {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= RenderStateTable::PS3RS_COLORWRITEENABLE as u32 {
            Ok(unsafe{std::mem::transmute(value)})
        } else {
            Err(())
        }
    }
}

// FUN_1410982f0
#[riri_hook_fn(static_offset(0x10a1cf0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrRenderStateTable(p_buffer: i32, p_fun: u32, value: *const u8) {
    let buffer = p_buffer as usize;
    let fun: RenderStateTable = p_fun.try_into().unwrap();
    let draw = globals::get_ngr_draw_state_unchecked_mut();
    // logln!(Information, "Function {:?}", fun);
    match fun {
        RenderStateTable::PS3RS_ZENABLE => {
            // This makes the camp menu category names disappear
            // logln!(Information, "ZENABLE, value is {:?}", value.is_null());
            if draw.basicBuffers.get_unchecked(buffer).z_enable != value.is_null() {
                draw.basicBuffers.get_unchecked_mut(buffer).z_enable = value.is_null();
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 4;
            }
        },
        RenderStateTable::PS3RS_ZWRITEENABLE => {
            if draw.basicBuffers.get_unchecked(buffer).z_write_enable != value.is_null() as i32 {
                // logln!(Information, "ZWRITEENABLE, value is {:?}", value.is_null());
                draw.basicBuffers.get_unchecked_mut(buffer).z_write_enable = value.is_null() as i32;
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 4;
            }
        },
        RenderStateTable::PS3RS_CULLMODE => {
            let cull_type = match value as u16 {
                1 => 0,
                2 => 1,
                _ => 2
            };
            if draw.basicBuffers.get_unchecked(buffer).cull_mode != cull_type {
                draw.basicBuffers.get_unchecked_mut(buffer).cull_mode = cull_type;
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 1;
            }
        },
        RenderStateTable::PS3RS_ZFUNC => {
            if draw.basicBuffers.get_unchecked(buffer).z_func != value as i32 {
                draw.basicBuffers.get_unchecked_mut(buffer).z_func = value as i32;
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 4;
            }
        },
        /*
        // (Very broken)
        RenderStateTable::PS3RS_ALPHABLENDENABLE => {
            if draw.basicBuffers.get_unchecked(buffer).alpha_blend_enable != value.is_null() {
                draw.basicBuffers.get_unchecked_mut(buffer).alpha_blend_enable = value.is_null();
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 2;
            }
        },
        */
        RenderStateTable::PS3RS_COLORWRITEENABLE => {
            let val = value as i32;
            if draw.basicBuffers.get_unchecked(buffer).color_write_enable != val {
                draw.basicBuffers.get_unchecked_mut(buffer).color_write_enable = val;
                draw.basicBuffers.get_unchecked_mut(buffer).flags |= 2;
            }
        },
        _ => original_function!(p_buffer, p_fun, value),
        // _ => (),
    }
}

#[allow(non_snake_case)]
pub unsafe fn gfdDeviceRenderSetState(p_buffer: i32, p_fun: u32, value: *mut u8) {
    ngrRenderStateTable(p_buffer, p_fun, value);
    *globals::get_gfd_global_unchecked_mut().graphics.render_state_current.get_unchecked_mut(p_fun as usize) = value as usize;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStateSetOtPreCallback
(_render_ot: *mut RenderOt, buffer: *mut u8, data: *mut u8) {
    gfdDeviceRenderSetState(buffer as i32, *(data as *const u32), data.add(8));
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePushOtPreCallback(_ot: *mut RenderOt, _a2: *mut u8, stack: *mut u8) {
    let stack = stack as u32;
    let global = globals::get_gfd_global_unchecked_mut();
    global.graphics.render_state_stack[stack as usize][1] = global.graphics.render_state_stack[stack as usize][0];
    global.graphics.render_state_stack[stack as usize][0] = global.graphics.render_state_current[stack as usize];
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePopOtPreCallback(_ot: *mut RenderOt, buffer: *mut u8, fun: *mut u8) {
    let fun = fun as u32;
    let buffer = buffer as i32;
    let global = globals::get_gfd_global_unchecked_mut();
    let popped = *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(0);
    *global.graphics.render_state_stack.get_unchecked_mut(fun as usize).get_unchecked_mut(0) = 
        *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(1);
    gfdDeviceRenderSetState(buffer, fun, popped as *mut u8);
}

#[riri_hook_fn(static_offset(0x1072960))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStateSet(prio: u32, state: u32, value: *mut u8) {
    let ot = RenderOtEx::<16>::new();
    ot.set::<u32>(0, state).unwrap();
    ot.set::<*mut u8>(8, value).unwrap();
    ot.set_pre_cb(gfdRenderStateSetOtPreCallback);
    ot.set_pre_cb_data(ot.data_raw());
    ot.link(prio)
}

#[riri_hook_fn(static_offset(0x1072a50))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePop(prio: u32, state: u32) {
    let ot = RenderOtEx::<0>::new();
    ot.set_pre_cb(gfdRenderStatePopOtPreCallback);
    ot.set_pre_cb_data(state as *mut u8);
    ot.link(prio)
}

#[riri_hook_fn(static_offset(0x10729e0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePush(prio: u32, state: u32) {
    let ot = RenderOtEx::<0>::new();
    ot.set_pre_cb(gfdRenderStatePushOtPreCallback);
    ot.set_pre_cb_data(state as *mut u8);
    ot.link(prio)
}

#[riri_hook_fn(static_offset(0x1072d40))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderSetBlendMode(prio: u32, blend: u32) {
    let ot = RenderOtEx::<0>::new();
    ot.set_data(&raw const *BlendModePkt::new(blend));
    ot.link(prio);
}

#[riri_hook_fn(static_offset(0x1101030))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdShaderVertexBind(prio: u32, vertex: *mut u8) {
    let ot = RenderOtEx::<0>::new();
    ot.set_pre_cb_data(vertex);
    ot.set_pre_cb(gfdShaderVertexBindOtPreCallback);
    ot.link(prio);
}

#[allow(non_snake_case)] 
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallback(_ot: *mut RenderOt, id: *mut u8, p_data: *mut u8) {
    gfdShaderVertexBindOtPreCallbackHook(_ot as *mut u8, id as u32, p_data);
}

#[riri_hook_fn(static_offset(0x1103150))]
#[allow(non_snake_case)] 
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallbackHook(_ot: *mut u8, id: u32, p_data: *mut u8) {
    original_function!(_ot, id, p_data)
}
#[riri_hook_fn(static_offset(0x1101600))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdShaderFragmentBind(prio: u32, fragment: *mut u8) {
    let ot = RenderOtEx::<0>::new();
    ot.set_pre_cb_data(fragment);
    ot.set_pre_cb(gfdShaderPixelBindOtPreCallback);
    ot.link(prio);
}
/*
#[riri_hook_fn(static_offset(0x11d81b0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrHashRasterizerKey(p_key: *const u8, p_hash: *mut u8) -> *mut u8 {
    let key = &*(p_key as *const RasterizerKey);
    std::ptr::write(p_hash as *mut CrcHash, CrcHash::new(key));
    p_hash
}
*/
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
/*
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRcTest(raster_key: *mut u8) -> u32 {
    let key = &*(raster_key as *const RasterizerKey);
    let ptr = GfdRc::new_in(RasterizerState::new(key), globals::get_ngr_allocator_unchecked());
    let ptr2 = ptr.clone();
    (*ptr2).get_field10() as u32
}
*/

#[riri_hook_fn(static_offset(0x1cb05d50))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn add_to_rasterizer_list(p_renderer: *mut u8, rasterizer_state: *mut u8) {
    original_function!(p_renderer, rasterizer_state)
}

#[riri_hook_fn(static_offset(0x11b56d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrGetRasterizer(p_state: *mut *mut u8, p_key: *mut u8) -> *mut *mut u8 {
    let state = &mut *(p_state as *mut *mut RasterizerState);
    let renderer = globals::get_ngr_dx11_renderer_unchecked_mut();
    let key = &mut *(p_key as *mut RasterizerKey);
    let renderer_mutex = (&mut **renderer.mutexes.get_unchecked_mut(1)).lock(renderer);
    match (*renderer_mutex).try_get_rasterizer_state_mut(key) {
        Some(n) => {
            *state = &raw mut *n;
            GfdRc::into_raw(GfdRc::clone_from_raw(n, globals::get_ngr_allocator_unchecked()));
        },
        None => {
            let mut new = GfdRc::new_in(RasterizerState::new(key), globals::get_ngr_allocator_unchecked());
            *state = &raw mut *new;
            add_to_rasterizer_list((&raw const *renderer_mutex) as *const u8 as *mut u8, (&raw const **state) as *const u8 as *mut u8);
            (*renderer_mutex).set_rasterizer_state(&mut**state);
            GfdRc::into_raw(new); // so we don't drop this!
        }
    };
    p_state
}

// pub unsafe extern "C" fn ngrCreateRasterizerState()
/*
#[riri_hook_fn(static_offset(0x1192f80))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrSetConstantBuffers(p_this: *mut u8, p_ty: u32, p_buf: *mut u8, upd: u32) {
    let ty:opengfd::device::ngr::renderer::cbuffer::BufferType = p_ty.try_into().unwrap();
    let this = &mut *(p_this as *mut DeferredContextBase);
    let buf = &mut *(p_buf as *mut ConstantBuffer);
    this.set_constant_buffers(ty, buf, upd);
}
*/
/*
#[riri_hook_fn(static_offset(0x112ad60))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn ngrImmediateRenderPrepareForDraw(state: *mut u8, a2: u32) {
    let state = &mut *(state as *mut opengfd::device::ngr::renderer::state::DrawState);
    let ctx = &mut**state.basicBuffers.get_unchecked_mut(a2 as usize).deferredContexts.get_unchecked_mut(state.otFrameId as usize);
    let flags = state.basicBuffers.get_unchecked_mut(a2 as usize).flags;
    if flags
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
