use opengfd::{
    device::ngr::renderer::{
        ps::PixelShader,
        vs::{ VertexShader, VertexShaderPlatform },
        state::{ DeferredContextBase, DeferredContextDX11 }
    },
    globals
};
use std::ptr::NonNull;
use windows::{
    core::Param,
    Win32::Graphics::Direct3D11::{ ID3D11PixelShader, ID3D11VertexShader }
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
#[riri_hook_fn(static_offset(0x1192ee0))]
#[allow(non_snake_case)] // Verified
pub unsafe extern "C" fn ngrSetPixelProgramLoadHook(p_ctx: *mut u8, p_shader: *mut u8) {
    let ctx = &mut *(p_ctx as *mut DeferredContextDX11);
    let shader = if p_shader.is_null() { None } else { Some(&(&*(p_shader as *const VertexShaderPlatform)).d3d_pixel) };
    // compare by-reference (equality operator checks by-value, following PartialEq/Eq impl)
    if std::ptr::eq(
        // Option<&T> has same memory layout as &T and *T
        std::mem::transmute::<Option<&ID3D11PixelShader>, *const ID3D11PixelShader>(shader), 
        &raw const ctx.super_.target_pixel_shader
    ) { return; }
    // get D3D context from NGR context
    let device_ctx = &ctx.super_.device_context;
    device_ctx.PSSetShader(shader, None);
    match shader {
        Some(v) => *&mut &ctx.super_.target_pixel_shader = v,
        None => *&mut &raw const ctx.super_.target_pixel_shader = std::ptr::null()
    }
}
*/
#[allow(non_snake_case)] // Verified
pub unsafe fn ngrSetPixelProgramLoad(ctx: &mut DeferredContextDX11, shader: Option<&VertexShaderPlatform>) {
    let shader = shader.map(|f| &f.d3d_pixel);
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

#[riri_hook_fn(static_offset(0x10743f0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePushOtPreCallbackHook(_ot: *mut u8, _a2: *mut u8, stack: i32) {
    let global = globals::get_gfd_global_unchecked_mut();
    global.graphics.render_state_stack[stack as usize][1] = global.graphics.render_state_stack[stack as usize][0];
    global.graphics.render_state_stack[stack as usize][0] = global.graphics.render_state_current[stack as usize];
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

#[riri_hook_fn(static_offset(0x1074430))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStatePopOtPreCallback(_render_ot: *mut u8, buffer: i32, fun: u32) {
    let global = globals::get_gfd_global_unchecked_mut();
    let popped = *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(0);
    *global.graphics.render_state_stack.get_unchecked_mut(fun as usize).get_unchecked_mut(0) = 
        *global.graphics.render_state_stack.get_unchecked(fun as usize).get_unchecked(1);
    gfdDeviceRenderSetState(buffer, fun, popped as *mut u8);
}

#[riri_hook_fn(static_offset(0x10743d0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdRenderStateSetOtPreCallback(_render_ot: *mut u8, buffer: i32, data: *mut u8) {
    gfdDeviceRenderSetState(buffer, *(data as *const u32), data.add(8));
}

/*
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdDeviceShaderVertexBindHook(id: i32, p_shader: *const u8) {
    let shader = p_shader as *const VertexShaderPlatform;
    if shader.is_null() { return; }
    let draw = &mut *globals::get_ngr_draw_state().unwrap();
    //logln!(Information, "Context: 0x{:x}, Shader: 0x{:x}", 
    /*
    println!("Context: 0x{:x}, Shader: 0x{:x}",
        draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize] as usize,
        shader as usize
    );
    */
    // original_function!(id, p_shader)
    ngrSetVertexProgramLoad(
        draw.basicBuffers[id as usize].deferredContexts[draw.otFrameId as usize].as_mut().unwrap(),
        shader.as_ref().unwrap());
}
*/
/*
#[riri_hook_fn(static_offset(0x1103150))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn gfdShaderVertexBindOtPreCallback(_render_ot: *mut u8, id: i32, shader: *mut u8) {
    let global = &mut*globals::get_gfd_global();
    let new_shader = shader as *const VertexShader;
    let current_vertex_shader = global.graphics.shader_current_vertex;
    if !std::ptr::eq(current_vertex_shader, new_shader) &&
        !new_shader.is_null() && !(&*current_vertex_shader).data.is_null()
    {
        /*
        gfdDeviceShaderVertexBind(id, &*(&*current_vertex_shader).data);
        *&mut global.graphics.shader_current_vertex = new_shader as *mut VertexShader;
        */
    }
    original_function!(_render_ot, id, shader)
}
*/
