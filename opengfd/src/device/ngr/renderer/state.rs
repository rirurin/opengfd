#![allow(non_snake_case, unused_imports)]
use bitflags::bitflags;
use crate::{
    device::ngr::{
        hint::MemHint,
        renderer::cbuffer::{
            BufferType, ConstantBuffer
        }, structures::CrcHash
    }, globals, graphics::texture::Texture, object::{ mesh::Mesh, node::Node }, utility::{ 
        misc::RGBAFloat,
        reference::{ GfdRcType, Reference }
    }
};
use glam::Mat4;
use std::{
    hash::Hash,
    ptr::NonNull
};
use opengfd_proc::GfdRcAuto;
use riri_mod_tools_proc::ensure_layout;
use windows::{
    core::Interface,
    Win32::Graphics::Direct3D11::{
        D3D11_FILL_MODE,
        ID3D11BlendState,
        ID3D11Buffer,
        ID3D11DeviceContext,
        ID3D11DepthStencilState,
        ID3D11PixelShader,
        ID3D11VertexShader,
        ID3D11Resource,
        ID3D11RasterizerState,
        ID3D11SamplerState
    }
};

#[ensure_layout(size = 6112usize)]
pub struct DrawState {
    #[field_offset(0usize)]
    pub vtable: *mut ::std::os::raw::c_void,
    #[field_offset(8usize)]
    pub field1_0x8: *mut ::std::os::raw::c_void,
    #[field_offset(16usize)]
    pub otFrameId: i32,
    #[field_offset(24usize)]
    pub field4_0x18: u64,
    #[field_offset(32usize)]
    pub ClearColor: RGBAFloat,
    #[field_offset(48usize)]
    pub field6_0x30: [f32; 2usize],
    #[field_offset(56usize)]
    pub fogCbuffer0: *mut ConstantBuffer,
    #[field_offset(64usize)]
    pub shadowCbuffer: *mut ConstantBuffer,
    #[field_offset(72usize)]
    pub fogCbuffer1: *mut ConstantBuffer,
    #[field_offset(80usize)]
    pub field10_0x50: *mut ConstantBuffer,
    #[field_offset(88usize)]
    pub DefaultEnv: *mut Texture,
    #[field_offset(96usize)]
    pub DefaultEnvToon: *mut Texture,
    #[field_offset(104usize)]
    pub field13_0x68: *mut ConstantBuffer,
    #[field_offset(112usize)]
    pub DefaultIBL: *mut Texture,
    #[field_offset(120usize)]
    pub field15_0x78: *mut ConstantBuffer,
    #[field_offset(128usize)]
    pub DefaultLUT: *mut Texture,
    #[field_offset(136usize)]
    pub sampler88: *mut ::std::os::raw::c_void,
    #[field_offset(144usize)]
    pub field18_0x90: *mut ConstantBuffer,
    #[field_offset(152usize)]
    pub field19_0x98: *mut ConstantBuffer,
    #[field_offset(160usize)]
    pub samplerA0: *mut ::std::os::raw::c_void,
    #[field_offset(168usize)]
    pub BayerMatrix4x4: *mut Texture,
    #[field_offset(176usize)]
    pub samplerB0: *mut ::std::os::raw::c_void,
    #[field_offset(184usize)]
    pub skyboxNode: *mut Node,
    #[field_offset(192usize)]
    pub skyboxMesh: *mut Mesh,
    #[field_offset(200usize)]
    pub infOceanNode: *mut Node,
    #[field_offset(208usize)]
    pub infOceanMesh: *mut Mesh,
    #[field_offset(224usize)]
    pub field35_0xe0: *mut ::std::os::raw::c_void,
    #[field_offset(240usize)]
    pub field44_0xf0: *mut ::std::os::raw::c_void,
    #[field_offset(256usize)]
    pub depthStencilViews: [*mut ::std::os::raw::c_void; 3usize],
    #[field_offset(280usize)]
    pub field54_0x118: *mut ::std::os::raw::c_void,
    #[field_offset(288usize)]
    pub field55_0x120: *mut ::std::os::raw::c_void,
    #[field_offset(296usize)]
    pub field56_0x128: *mut ::std::os::raw::c_void,
    #[field_offset(304usize)]
    pub field57_0x130: *mut ::std::os::raw::c_void,
    #[field_offset(312usize)]
    pub field58_0x138: [*mut ::std::os::raw::c_void; 2usize],
    #[field_offset(328usize)]
    pub field59_0x148: [*mut ::std::os::raw::c_void; 2usize],
    #[field_offset(344usize)]
    pub field60_0x158: [*mut ::std::os::raw::c_void; 4usize],
    #[field_offset(648usize)]
    pub GFD_PSCONST_HDR: *mut ConstantBuffer,
    #[field_offset(656usize)]
    pub field334_0x290: *mut ConstantBuffer,
    #[field_offset(664usize)]
    pub field335_0x298: *mut ConstantBuffer,
    #[field_offset(672usize)]
    pub field336_0x2a0: *mut ConstantBuffer,
    #[field_offset(680usize)]
    pub field337_0x2a8: *mut ConstantBuffer,
    #[field_offset(688usize)]
    pub field338_0x2b0: *mut ConstantBuffer,
    #[field_offset(696usize)]
    pub field339_0x2b8: *mut ConstantBuffer,
    #[field_offset(704usize)]
    pub CBUF_2C0: *mut ConstantBuffer,
    #[field_offset(712usize)]
    pub CBUF_2C8: *mut ConstantBuffer,
    #[field_offset(720usize)]
    pub field342_0x2d0: *mut ConstantBuffer,
    #[field_offset(728usize)]
    pub field343_0x2d8: *mut ConstantBuffer,
    #[field_offset(736usize)]
    pub REG_12_BUF_2E0: *mut ConstantBuffer,
    #[field_offset(748usize)]
    pub field349_0x2ec: [GraphicsStarFilter; 4usize],
    #[field_offset(812usize)]
    pub field350_0x32c: [[f32; 4usize]; 3usize],
    #[field_offset(1196usize)]
    pub field687_0x4ac: u32,
    #[field_offset(1200usize)]
    pub field688_0x4b0: f32,
    #[field_offset(1204usize)]
    pub field689_0x4b4: f32,
    #[field_offset(1208usize)]
    pub field690_0x4b8: f32,
    #[field_offset(1216usize)]
    pub REG_11_BUF_4C0: *mut ConstantBuffer,
    #[field_offset(1320usize)]
    pub GFD_PSCONST_EFFECT_FocalBlur: *mut ConstantBuffer,
    #[field_offset(1328usize)]
    pub field793_0x530: *mut ConstantBuffer,
    #[field_offset(1336usize)]
    pub field794_0x538: *mut ConstantBuffer,
    #[field_offset(1344usize)]
    pub field795_0x540: *mut ConstantBuffer,
    #[field_offset(1352usize)]
    pub field796_0x548: *mut ConstantBuffer,
    #[field_offset(1408usize)]
    pub grad_texture: *mut Texture,
    #[field_offset(1416usize)]
    pub tex588: *mut Texture,
    #[field_offset(1424usize)]
    pub smaaBuffer: *mut ConstantBuffer,
    #[field_offset(1432usize)]
    pub colorCorrectBuffer: *mut ConstantBuffer,
    #[field_offset(1440usize)]
    pub field849_0x5a0: *mut ::std::os::raw::c_void,
    #[field_offset(1448usize)]
    pub field850_0x5a8: *mut ::std::os::raw::c_void,
    #[field_offset(1456usize)]
    pub field851_0x5b0: *mut ::std::os::raw::c_void,
    #[field_offset(1464usize)]
    pub ssaoBuffer0: *mut ConstantBuffer,
    #[field_offset(1472usize)]
    pub ssaoBuffer1: *mut ConstantBuffer,
    #[field_offset(1480usize)]
    pub field854_0x5c8: *mut _142234cb0,
    #[field_offset(1488usize)]
    pub field855_0x5d0: *mut _142234cb0,
    #[field_offset(1496usize)]
    pub temperareWobbingTex: *mut Texture,
    #[field_offset(1504usize)]
    pub temperareBuffer: *mut ConstantBuffer,
    #[field_offset(1512usize)]
    pub field858_0x5e8: *mut ::std::os::raw::c_void,
    #[field_offset(1520usize)]
    pub field859_0x5f0: *mut ::std::os::raw::c_void,
    #[field_offset(1528usize)]
    pub field860_0x5f8: *mut ::std::os::raw::c_void,
    #[field_offset(1536usize)]
    pub CloudMain: *mut Texture,
    #[field_offset(1544usize)]
    pub CloudSub: *mut Texture,
    #[field_offset(1552usize)]
    pub Cloud2D: *mut Texture,
    #[field_offset(1560usize)]
    pub REG_13_BUFFER_618: *mut ConstantBuffer,
    #[field_offset(1592usize)]
    pub REG_11_BUF_638: *mut ConstantBuffer,
    #[field_offset(1600usize)]
    pub REG_11_BUF_640: *mut ConstantBuffer,
    #[field_offset(1608usize)]
    pub REG_11_BUF_648: *mut ConstantBuffer,
    #[field_offset(1616usize)]
    pub REG_11_BUF_650: *mut ConstantBuffer,
    #[field_offset(1624usize)]
    pub REG_11_BUF_658: *mut ConstantBuffer,
    #[field_offset(1632usize)]
    pub GFD_PSCONST_EFFECT_Blur: *mut ConstantBuffer,
    #[field_offset(1640usize)]
    pub REG_11_BUF_668: *mut ConstantBuffer,
    #[field_offset(1648usize)]
    pub GFD_PSCONST_METABALL: *mut ConstantBuffer,
    #[field_offset(1664usize)]
    pub REG_11_BUF_680: *mut ConstantBuffer,
    #[field_offset(1672usize)]
    pub GFD_PSCONST_EFFECT_Outline: *mut ConstantBuffer,
    #[field_offset(1680usize)]    
    pub GFD_PSCONST_EFFECT_BrushStroke: *mut ConstantBuffer,
    #[field_offset(1688usize)]
    pub fullHDCanvas: *mut ::std::os::raw::c_void,
    #[field_offset(1696usize)]
    pub fullHDBrushstroke01: *mut ::std::os::raw::c_void,
    #[field_offset(1704usize)]
    pub REG_11_BUF_6A8: *mut ConstantBuffer,
    #[field_offset(1712usize)]
    pub field911_0x6b0: _142236508,
    #[field_offset(1936usize)]
    pub field912_0x790: _142236510,
    #[field_offset(2088usize)]
    pub Field828: [*mut ::std::os::raw::c_void; 2usize],
    #[field_offset(2224usize)]
    pub sampler8B0: *mut ::std::os::raw::c_void,
    #[field_offset(2248usize)]
    pub toonShadowHatching: [*mut Texture; 3usize],
    #[field_offset(2272usize)]
    pub basicBuffers: [BasicBuffers; 4usize],
}

#[repr(C)]
#[derive(PartialEq)]
pub struct BufferBlendMode {
    pub(super) field00: i32,
    pub(super) field04: i32,
    pub(super) field08: i32,
    pub(super) field0c: i32,
    pub(super) field10: i32,
    pub(super) field14: i32
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BufferFlags: u32 {
        const USING_RASTERIZER = 1 << 0;
        const USING_BLEND = 1 << 1;
        const USING_DEPTH_STENCIL = 1 << 2;
        const USING_VSCONST_TRANSFORM = 1 << 24;
        const USING_VSCONST_VIEWPROJ = 1 << 25;
        const USING_VSCONST_COLORS = 1 << 26;
        const USING_VSCONST_UV_TRANSFORM0 = 1 << 27;
        const USING_VSCONST_UV_TRANSFORM1 = 1 << 28;
        const USING_VSCONST_UV_TRANSFORM2 = 1 << 29;
        const USING_VSCONST_VAT = 1 << 30;
        const USING_REG_8_BUF_360 = 1 << 31;
    }
}

#[ensure_layout(size = 960usize)]
pub struct BasicBuffers {
    #[field_offset(0usize)]
    pub vtable: *mut ::std::os::raw::c_void,
    #[field_offset(16usize)]
    pub opaqueTexture: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub field10_0x18: u32,
    #[field_offset(32usize)]
    pub field12_0x20: *mut ::std::os::raw::c_void,
    #[field_offset(72usize)]
    pub vatBoundingBoxMin: f32,
    #[field_offset(84usize)]
    pub vatBoundingBoxMax: f32,
    #[field_offset(0x5c)] pub cull_mode: i32,
    #[field_offset(0x74)] pub alpha_blend_enable: bool,
    #[field_offset(0x78)] pub blend_mode: BufferBlendMode,
    #[field_offset(0x90)] pub color_write_enable: i32,
    #[field_offset(0x98)] pub z_enable: bool,
    #[field_offset(0x9c)] pub z_write_enable: i32,
    #[field_offset(0xa0)] pub z_func: i32,
    #[field_offset(200usize)]
    pub field167_0xc8: u32,
    #[field_offset(204usize)]
    pub field168_0xcc: u32,
    #[field_offset(728usize)]
    pub deferredContexts: [*mut DeferredContextDX11; 3usize],
    #[field_offset(752usize)]
    pub GFD_VSCONST_SYSTEM: *mut ConstantBuffer,
    #[field_offset(760usize)]
    pub GFD_VSCONST_TRANSFORM: *mut ConstantBuffer,
    #[field_offset(768usize)]
    pub GFD_VSCONST_VIEWPROJ: *mut ConstantBuffer,
    #[field_offset(776usize)]
    pub GFD_VSCONST_COLORS: *mut ConstantBuffer,
    #[field_offset(784usize)]
    pub GFD_VSCONST_UVX_TRANSFORM: [*mut ConstantBuffer; 3usize],
    #[field_offset(808usize)]
    pub GFD_VSCONST_LIGHT_VEC: *mut ConstantBuffer,
    #[field_offset(816usize)]
    pub Toon_GFD_PSCONST_LIGHT_PS: *mut ConstantBuffer,
    #[field_offset(824usize)]
    pub PBR_GFD_PSCONST_LIGHT_PS: *mut ConstantBuffer,
    #[field_offset(832usize)]
    pub Skylight_GFD_PSCONST_SKY_LIGHT_PS: *mut ConstantBuffer,
    #[field_offset(840usize)]
    pub GFD_PSCONST_SYSTEM: *mut ConstantBuffer,
    #[field_offset(848usize)]
    pub GFD_PSCONST_ENV_COLORS: *mut ConstantBuffer,
    #[field_offset(856usize)]
    pub GFD_VSCONST_VAT: *mut ConstantBuffer,
    #[field_offset(864usize)]
    pub REG_8_BUF_360: *mut ConstantBuffer,
    #[field_offset(880usize)]
    pub field711_0x370: Mat4,
    #[field_offset(944usize)]
    pub flags: u32,
    #[field_offset(948usize)]
    pub field713_0x3b4: u32,
}

#[ensure_layout(size = 56usize)]
pub struct CbufferFields {
    #[field_offset(8usize)]
    pub field8_0x8: *mut ::std::os::raw::c_void,
    #[field_offset(16usize)]
    pub field9_0x10: *mut ::std::os::raw::c_void,
    #[field_offset(40usize)]
    pub field26_0x28: MemHint,
}

pub trait DeferredContext {
    unsafe fn set_constant_buffers(&mut self, ty: BufferType, buf: &mut ConstantBuffer, upd: u32);
    unsafe fn set_depth_stencil_state(&mut self, a2: usize, a3: u8);
    unsafe fn rs_set_state(&mut self, rasterizer: *mut u8);
}

#[derive(Debug)]
#[ensure_layout(size = 1800usize)]
pub struct DeferredContextBase {
    #[field_offset(0)] vtable: *const std::ffi::c_void,
    #[field_offset(8)] pub device_context: ID3D11DeviceContext,
    #[field_offset(0x90)] pub rasterizer: ID3D11RasterizerState,
    #[field_offset(0xa0)] pub target_vertex_shader: ID3D11VertexShader,
    #[field_offset(0xb0)] pub target_pixel_shader: ID3D11PixelShader,
    #[field_offset(0x108)] field108: [DeferredContext108; 4]
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeferredContext108 {
    buffer: [ID3D11Buffer; 4],
    _unk: [u8; 0x160]
}

impl DeferredContextBase {
    // unsafe fn set_rasterizer_state(&mut self)
}

impl DeferredContext for DeferredContextBase {
    // 0x141188f80 (Metaphor: Refantazio Prologue Demo, Steam 1.01)
    // 0x141192f80 (Metaphor: Refantazio, Steam 1.02)
    // TODO: Fix this! Flickering!
    unsafe fn set_constant_buffers(&mut self, ty: BufferType, buf: &mut ConstantBuffer, upd: u32) {
        let ctx = &self.device_context;
        let first = buf.get_buffer();
        let update_flags = if !buf.has_resources() { 1 } else { 1 << (upd & 0x1f) };
        if (buf.active_buffers & update_flags) != 0 { 
            ctx.UpdateSubresource(
                first.map(|f| f.into()), 
                0, 
                None, 
                buf.get_resource(upd),
                0, 
                0);
            buf.active_buffers &= !update_flags; 
        }
        if !std::ptr::eq(
            self.field108.get_unchecked(ty as usize)
            .buffer.get_unchecked(buf.slot as usize)
            , std::mem::transmute(buf.get_buffer())
        ) {
            let slice = buf.get_buffer_as_slice();
            match ty {
                BufferType::Vertex => ctx.VSSetConstantBuffers(buf.slot as u32, Some(slice)),
                BufferType::Geometry => ctx.GSSetConstantBuffers(buf.slot as u32, Some(slice)),
                BufferType::Pixel => ctx.PSSetConstantBuffers(buf.slot as u32, Some(slice)),
                BufferType::Compute => ctx.CSSetConstantBuffers(buf.slot as u32, Some(slice)),
            };
            let old_buf = self.field108.get_unchecked(ty as usize);
            match buf.get_buffer() {
                Some(v) => *&mut old_buf.buffer.get_unchecked(buf.slot as usize) = v,
                None => 
            *&mut (old_buf.buffer.get_unchecked(buf.slot as usize) as *const ID3D11Buffer) 
            = std::ptr::null()
            };
        }
    }
    unsafe fn set_depth_stencil_state(&mut self, a2: usize, a3: u8) {
        
    }
    unsafe fn rs_set_state(&mut self, rasterizer: *mut u8) {

    }
    // GetRasterizerState
    // GetBlendState
    // OMSetBlendState
    // SetShaderSampler
}

#[derive(Debug)]
#[ensure_layout(size = 1808usize)]
pub struct DeferredContextDX11 {
    #[field_offset(0usize)]
    pub super_: DeferredContextBase,
}


/*
impl DeferredContextDX11 {
    pub(crate) unsafe fn set_vertex_program_load(&mut self, shader: Option<std::ptr::NonNull<super::vs::VertexShaderPlatform>>) {
        if shader.is_some() {
            let target_shader = shader.unwrap().as_ref().d3d_shader.as_ref().unwrap();
            let mut context_shader = self.super_.target_shader.as_mut().unwrap();
            let device_ctx = self.super_.device_context.as_ref().unwrap();
            if target_shader != context_shader {
                let input_layout = shader.unwrap().as_ref().d3d_input_layout.as_ref().unwrap();
                device_ctx.IASetInputLayout(input_layout);
                device_ctx.VSSetShader(target_shader, None);
                context_shader = (target_shader as *const ID3D11VertexShader as *mut ID3D11VertexShader).as_mut().unwrap();
            }
        }
    }
}
*/

// gfdShaderVertexBindOtPreCallback (completely breaks UI)
// gfdShaderFragmentBindOtPreCallback (works, with minor graphical issues)
// gfdRenderStatePushOtPreCallback (verified)
// gfdRenderStateSetOtPreCallback (verified)
// gfdRenderStatePopOtPreCallback (verified)
// gfdCmdBufferAlloc (verified)
// BindVertexShader (TODO. Hooked, but needs more work)
// BindPixelShader (verified)
// PushRenderState (verified)
// SetRenderState (verified)
// PopRenderState (verified)
// SetupOt (verified)
// RenderOtLink (verified)
// gfdRender2D_PosCol
// gfdRender2D_PosColTex
// gfdSetupTexture2D
// gfdTexture2D
// gfdDrawPrimLine2D
// gfdDrawPrimRect2D
// DrawDebugFont

#[ensure_layout(size = 16usize)]
pub struct GraphicsStarFilter {
    #[field_offset(0usize)]
    pub field0_0x0: i32,
    #[field_offset(4usize)]
    pub field1_0x4: f32,
    #[field_offset(8usize)]
    pub field2_0x8: f32,
    #[field_offset(12usize)]
    pub field3_0xc: f32,
}

#[ensure_layout(size = 224usize)]
pub struct _142236508 {}
#[ensure_layout(size = 128usize)]
pub struct _142236510 {}
#[ensure_layout(size = 96usize)]
pub struct _142234cb0 {}

#[allow(dead_code)]
pub struct PlatformTexture {
    data: [u8; 0xb0]
}

#[allow(dead_code)]
pub(crate) trait PipelineStateObject {
    type PlatformState;
    type Key;

    fn get_key(&self) -> &Self::Key;
    fn get_key_hash(&self) -> u32;
    fn set_platform_state(&mut self, plat: Option<Self::PlatformState>);
    // to pass into CreateState for D3D
    fn get_platform_state_ptr(&mut self) -> Option<*mut Option<Self::PlatformState>>;
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RasterizerKey {
    pub(crate) field_mode: FillMode,
    pub(crate) cull_mode: CullMode,
    pub(crate) is_front_counter_clockwise: bool,
    pub(crate) scissor_enable: bool,
    pub(crate) antialiased_line_enable: bool,
    pub(crate) depth_bias: u32,
    pub(crate) depth_bias_clamp: f32,
    pub(crate) slope_scaled_depth_bias: f32,
    pub(crate) depth_clip_enable: bool
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FillMode {
    Wireframe = 0,
    Solid
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CullMode {
    None = 0,
    Front,
    Back
}

impl Hash for RasterizerKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { 
        state.write_u8(self.field_mode as u8);
        state.write_u8(self.cull_mode as u8);
        state.write_u8(self.is_front_counter_clockwise as u8);
        state.write_u8(self.scissor_enable as u8);
        state.write_u8(self.antialiased_line_enable as u8);
        state.write_u32(self.depth_bias);
        state.write_u32(self.depth_bias_clamp as u32);
        state.write_u32(self.slope_scaled_depth_bias as u32);
        state.write_u8(self.depth_clip_enable as u8);
    }
}

impl RasterizerKey {
    pub fn crc_hash(&self) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        self.hash(&mut hasher);
        hasher.finalize()
    } 
}

impl PartialEq<CrcHash> for RasterizerState {
    fn eq(&self, other: &CrcHash) -> bool {
        self.key.crc_hash() == other.get_hash()
    }
}

impl PartialEq<RasterizerKey> for RasterizerState {
    fn eq(&self, other: &RasterizerKey) -> bool {
        self.key.field_mode == other.field_mode &&
        self.key.cull_mode == other.cull_mode &&
        self.key.is_front_counter_clockwise == other.is_front_counter_clockwise &&
        self.key.scissor_enable == other.scissor_enable &&
        self.key.antialiased_line_enable == other.antialiased_line_enable &&
        self.key.depth_bias              == other.depth_bias &&
        self.key.depth_bias_clamp        == other.depth_bias_clamp &&
        self.key.slope_scaled_depth_bias == other.slope_scaled_depth_bias &&
        self.key.depth_clip_enable       == other.depth_clip_enable
    }
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct RasterizerState {
    _cpp_vtable: *const u8,
    ref_count: Reference,
    field10: usize,
    key: RasterizerKey,
    hash: CrcHash,
    platform_rasterizer: Option<ID3D11RasterizerState>
}

impl RasterizerState {
    pub fn new(key: &RasterizerKey) -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_rasterstate_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            ref_count: Reference::new(),
            field10: 0,
            key: key.clone(),
            hash: CrcHash::new(key),
            platform_rasterizer: None
        }
    }

    pub fn get_field10(&self) -> usize { self.field10 }
}

impl PipelineStateObject for RasterizerState {
    type PlatformState = ID3D11RasterizerState;
    type Key = RasterizerKey;
    fn get_key(&self) -> &Self::Key { &self.key }
    fn get_key_hash(&self) -> u32 { self.hash.get_hash() }
    fn set_platform_state(&mut self, plat: Option<Self::PlatformState>) {
        self.platform_rasterizer = plat;
    }
    fn get_platform_state_ptr(&mut self) -> Option<*mut Option<Self::PlatformState>> {
        Some(&raw mut self.platform_rasterizer)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BlendKey {
    pub(crate) enable_blending: bool,
    pub(crate) source_blend: BlendType,
    pub(crate) dest_blend: BlendType,
    pub(crate) blend_op: BlendTypeOperation,
    pub(crate) source_blend_alpha: BlendType,
    pub(crate) dest_blend_alpha: BlendType,
    pub(crate) blend_op_alpha: BlendTypeOperation,
    pub(crate) render_mask: u32,
    pub(crate) blend_state: bool
}

impl Hash for BlendKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { 
        state.write_u8(self.enable_blending as u8);
        if self.enable_blending {
            state.write_u8(self.source_blend as u8);
            state.write_u8(self.dest_blend as u8);
            state.write_u8(self.blend_op as u8);
            state.write_u8(self.source_blend_alpha as u8);
            state.write_u8(self.dest_blend_alpha as u8);
            state.write_u8(self.blend_op_alpha as u8);
            state.write_u32(self.render_mask as u32);
        }
        state.write_u8(self.blend_state as u8);
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendType {
    Zero = 0,
    One,
    SourceColor,
    InverseSourceColor,
    SourceAlpha,
    InverseSourceAlpha,
    DestinationAlpha,
    InverseDestinationAlpha,
    DestinationColor,
    InverseDestinationColor,
    SourceAlphaSaturate,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlendTypeOperation {
    Add = 0,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct BlendState {
    _cpp_vtable: *const u8,
    ref_count: Reference,
    field10: usize,
    key: BlendKey,
    hash: CrcHash,
    platform_blend: Option<ID3D11BlendState>
}

impl BlendState {
    pub fn new(key: &BlendKey) -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_blendstate_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            ref_count: Reference::new(),
            field10: 0,
            key: key.clone(),
            hash: CrcHash::new(key),
            platform_blend: None
        }
    }

    pub fn get_field10(&self) -> usize { self.field10 }
}

impl PipelineStateObject for BlendState {
    type PlatformState = ID3D11BlendState;
    type Key = BlendKey;
    fn get_key(&self) -> &Self::Key { &self.key }
    fn get_key_hash(&self) -> u32 { self.hash.get_hash() }
    fn set_platform_state(&mut self, plat: Option<Self::PlatformState>) {
        self.platform_blend = plat;
    }
    fn get_platform_state_ptr(&mut self) -> Option<*mut Option<Self::PlatformState>> {
        Some(&raw mut self.platform_blend)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DepthStencilDescriptions {
    pub(crate) stencil_fall_op: StencilOperation,
    pub(crate) stencil_depth_fall_op: StencilOperation,
    pub(crate) stencil_pass_op: StencilOperation,
    pub(crate) stencil_func: ComparisonFunc,
}

impl Hash for DepthStencilDescriptions {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { 
        state.write_u8(self.stencil_fall_op as u8);
        state.write_u8(self.stencil_depth_fall_op as u8);
        state.write_u8(self.stencil_pass_op as u8);
        state.write_u8(self.stencil_func as u8);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DepthStencilKey {
    pub(crate) depth_enable: bool,
    pub(crate) depth_write_mask: DepthWriteMask,
    pub(crate) depth_func: ComparisonFunc,
    pub(crate) stencil_enable: bool,
    pub(crate) stencil_read_mask: u8,
    pub(crate) stencil_write_mask: u8,
    pub(crate) front_face: DepthStencilDescriptions,
    pub(crate) back_face: DepthStencilDescriptions
}

// 0x1411cdff0
impl Hash for DepthStencilKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.depth_enable as u8);
        state.write_u8(self.depth_write_mask as u8);
        if self.depth_enable {
            state.write_u8(self.depth_func as u8);
        }
        state.write_u8(self.stencil_enable as u8);
        if self.stencil_enable {
            state.write_u8(self.stencil_read_mask);
            state.write_u8(self.stencil_write_mask);
            self.front_face.hash(state);
            self.back_face.hash(state);
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DepthWriteMask {
    MaskNone = 0,
    MaskAll
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StencilOperation {
    Keep = 0,
    Zero,
    Replace,
    IncrementClamp,
    DecrementClamp,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComparisonFunc {
    Never = 0,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct DepthStencilState {
    _cpp_vtable: *const u8,
    ref_count: Reference,
    field10: usize,
    key: DepthStencilKey,
    hash: CrcHash,
    platform_stencil: Option<ID3D11DepthStencilState>
}

impl DepthStencilState {
    pub fn new(key: &DepthStencilKey) -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_depthstencilstate_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            ref_count: Reference::new(),
            field10: 0,
            key: key.clone(),
            hash: CrcHash::new(key),
            platform_stencil: None
        }
    }

    pub fn get_field10(&self) -> usize { self.field10 }
}

impl PipelineStateObject for DepthStencilState {
    type PlatformState = ID3D11DepthStencilState;
    type Key = DepthStencilKey;
    fn get_key(&self) -> &Self::Key { &self.key }
    fn get_key_hash(&self) -> u32 { self.hash.get_hash() }
    fn set_platform_state(&mut self, plat: Option<Self::PlatformState>) {
        self.platform_stencil = plat;
    }
    fn get_platform_state_ptr(&mut self) -> Option<*mut Option<Self::PlatformState>> {
        Some(&raw mut self.platform_stencil)
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterMode {
    CmpMinMagMipPoint = 0,
    CmpMinMagPointMipLinear,
    CmpMinPointMagLinearMipPoint,
    MinPointMagMipLinear,
    MinLinearMagMipPoint,
    MinMagLinearMipPoint,
    MinMagMipLinear,
    Anisotropic
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FilterModeComparison {
    CmpMinMagMipPoint = 0,
    CmpMinMagPointMipLinear,
    CmpMinPointMagLinearMipPoint,
    MinPointMagMipLinear,
    MinLinearMagMipPoint,
    MinMagLinearMipPoint,
    MinMagMipLinear,
    Anisotropic
}

impl From<FilterMode> for FilterModeComparison {
    fn from(value: FilterMode) -> Self {
        // SAFETY: They are exactly the same in memory layout and valid discriminants. The only
        // difference is their implementation of Into<D3D_FILTER>
        unsafe { std::mem::transmute(value) }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureAddressMode {
    Wrap = 0,
    Mirror,
    Clamp,
    Border,
    MirrorOnce
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BorderColor {
    Clear = 0,
    Black,
    White
}

impl From<BorderColor> for RGBAFloat {
    fn from(value: BorderColor) -> Self {
        match value {
            BorderColor::Clear => RGBAFloat::from_rgba_array_f32([0f32; 4]),
            BorderColor::Black => RGBAFloat::from_rgba_array_f32([0f32, 0f32, 0f32, 1f32]),
            BorderColor::White => RGBAFloat::from_rgba_array_f32([1f32; 4]),
        }
    }
}
impl From<BorderColor> for [f32; 4] {
    fn from(value: BorderColor) -> Self {
        match value {
            BorderColor::Clear => [0f32; 4],
            BorderColor::Black => [0f32, 0f32, 0f32, 1f32],
            BorderColor::White => [1f32; 4],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SamplerKey {
    pub(crate) filter: FilterMode,
    pub(crate) address_u: TextureAddressMode,
    pub(crate) address_v: TextureAddressMode,
    pub(crate) address_w: TextureAddressMode,
    pub(crate) mip_lod_bias: f32,
    pub(crate) max_anistropy: u32,
    pub(crate) min_lod: f32,
    pub(crate) max_lod: f32,
    pub(crate) comparison: ComparisonFunc,
    pub(crate) border_color: BorderColor
}

// 0x1411cdb00
impl Hash for SamplerKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.filter as u8);
        state.write_u8(self.address_u as u8);
        state.write_u8(self.address_v as u8);
        state.write_u8(self.address_w as u8);
        state.write_u32(self.mip_lod_bias as u32);
        state.write_u32(self.max_anistropy);
        state.write_u32(self.min_lod as u32);
        state.write_u32(self.max_lod as u32);
        state.write_u8(self.comparison as u8);
        state.write_u8(self.border_color as u8);
    }
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct SamplerState {
    _cpp_vtable: *const u8,
    ref_count: Reference,
    field10: usize,
    key: SamplerKey,
    hash: CrcHash,
    platform_sampler: Option<ID3D11SamplerState>
}

impl SamplerState {
    pub fn new(key: &SamplerKey) -> Self {
        Self {
            _cpp_vtable: match globals::get_ngr_depthstencilstate_vtable() {
                Some(v) => &raw const *v,
                None => std::ptr::null()
            },
            ref_count: Reference::new(),
            field10: 0,
            key: key.clone(),
            hash: CrcHash::new(key),
            platform_sampler: None
        }
    }

    pub fn get_field10(&self) -> usize { self.field10 }
}

impl PipelineStateObject for SamplerState {
    type PlatformState = ID3D11SamplerState;
    type Key = SamplerKey;
    fn get_key(&self) -> &Self::Key { &self.key }
    fn get_key_hash(&self) -> u32 { self.hash.get_hash() }
    fn set_platform_state(&mut self, plat: Option<Self::PlatformState>) {
        self.platform_sampler = plat;
    }
    fn get_platform_state_ptr(&mut self) -> Option<*mut Option<Self::PlatformState>> {
        Some(&raw mut self.platform_sampler)
    }
}
