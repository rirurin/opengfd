#![allow(non_snake_case)]

use crate::{
    graphics::texture::Texture,
    object::{ mesh::Mesh, node::Node },
    utility::misc::RGBAFloat
};
use glam::Mat4;
use riri_mod_tools_proc::ensure_layout;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11DeviceContext,
    ID3D11PixelShader,
    ID3D11VertexShader
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

#[ensure_layout(size = 136usize)]
pub struct ConstantBuffer {
    #[field_offset(0usize)]
    pub vtable: *mut [*mut ::std::os::raw::c_void; 24usize],
    #[field_offset(8usize)]
    pub ref_: i32,
    #[field_offset(16usize)]
    pub field3_0x10: *mut ::std::os::raw::c_void,
    #[field_offset(24usize)]
    pub cbufferFields: CbufferFields,
    #[field_offset(80usize)]
    pub byteWidth: u32,
    #[field_offset(88usize)]
    pub field7_0x58: i32,
    #[field_offset(92usize)]
    pub cbufferSlot: i32,
    #[field_offset(96usize)]
    pub d3d11Cbuffer: [*mut ::std::os::raw::c_void; 3usize],
    #[field_offset(124usize)]
    pub field11_0x7c: i32,
    #[field_offset(128usize)]
    pub activeBuffers: i32,
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
    pub field712_0x3b0: u32,
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

#[ensure_layout(size = 16usize)]
pub struct MemHint {
    #[field_offset(0usize)]
    pub vtable: *mut ::std::os::raw::c_void,
}

#[derive(Debug)]
#[ensure_layout(size = 1800usize)]
pub struct DeferredContextBase {
    #[field_offset(0usize)]
    pub vtable: *mut ::std::os::raw::c_void,
    #[field_offset(8usize)]
    pub device_context: ID3D11DeviceContext,
    #[field_offset(160usize)]
    pub target_vertex_shader: ID3D11VertexShader,
    #[field_offset(176usize)]
    pub target_pixel_shader: ID3D11PixelShader,
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

// gfdShaderVertexBindOtPreCallback
// gfdShaderFragmentBindOtPreCallback
// gfdRenderStatePushOtPreCallback
// gfdRenderStateSetOtPreCallback
// gfdRenderStatePopOtPreCallback
// gfdCmdBufferAlloc
// BindVertexShader
// BindPixelShader
// PushRenderState
// SetRenderState
// PopRenderState
// SetupOt
// RenderOtLink
// gfdRender2D_PosCol
// gfdRender2D_PosColTex
// gfdSetupTexture2D
// gfdTexture2D
// gfdDrawPrimLine2D
// gfdDrawPrimRect2D
// DrawDebugFont

#[ensure_layout(size = 4usize)]
pub struct D3D11DeviceContext {}

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
