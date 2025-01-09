#![allow(non_snake_case, unused_imports)]
use bitflags::bitflags;
use crate::{
    device::ngr::{
        hint::MemHint,
        renderer::{
            blend::BufferBlendMode,
            cbuffer::{
                BufferType, ConstantBuffer
            },
            platform::d3d::{
                ResourceView,
                ResourceView2,
                TextureResource
            }
        },
        structures::CrcHash
    },
    globals, 
    graphics::texture::Texture, 
    object::{ mesh::Mesh, node::Node }, 
    utility::{ 
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
    Win32::Graphics::{
        Direct3D::D3D_PRIMITIVE_TOPOLOGY,
        Direct3D11::{
            D3D11_FILL_MODE,
            ID3D11BlendState,
            ID3D11Buffer,
            ID3D11DeviceContext,
            ID3D11DepthStencilState,
            ID3D11DepthStencilView,
            ID3D11PixelShader,
            ID3D11VertexShader,
            ID3D11Resource,
            ID3D11RasterizerState,
            ID3D11RenderTargetView,
            ID3D11SamplerState,
            ID3D11ShaderResourceView
        }
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
    #[field_offset(0x820)] pub effect_scale_adjust: bool,
    #[field_offset(2088usize)]
    pub Field828: [*mut ::std::os::raw::c_void; 2usize],
    #[field_offset(0x888)] pub effect_scale_index: u32,
    #[field_offset(2224usize)]
    pub sampler8B0: *mut ::std::os::raw::c_void,
    #[field_offset(2248usize)]
    pub toonShadowHatching: [*mut Texture; 3usize],
    #[field_offset(2272usize)]
    pub basicBuffers: [BasicBuffers; 4usize],
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
    #[field_offset(0x58)] pub rasterizer_key: RasterizerKey,
    #[field_offset(0x74)] pub blend_key: BlendKey,
    #[field_offset(0x98)] pub depth_stencil_key: DepthStencilKey,
    #[field_offset(0xc8)] pub sampler_flag: u32,
    #[field_offset(0xcc)] pub sampler_mask: u32,
    #[field_offset(0xd0)] pub sampler_keys: [ SamplerKey; 13 ],
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
    pub flags: BufferFlags,
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

    unsafe fn set_shader_sample(&mut self, ty: BufferType, id: usize, state: Option<&SamplerState>);
    unsafe fn set_shader_resource_view(&mut self, ty: BufferType, id: usize, state: Option<&TextureResource>);
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IATopology {
    Undefined = 0,
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl TryFrom<u32> for IATopology {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= Self::TriangleStrip as u32 {
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
#[ensure_layout(size = 0x708)]
pub struct DeferredContextBase {
    #[field_offset(0)] vtable: *const std::ffi::c_void,
    #[field_offset(8)] pub device_context: ID3D11DeviceContext,
    #[field_offset(0x38)] render_target_view: Option<ID3D11RenderTargetView>,
    #[field_offset(0x40)] field40: [*mut u8; 7],
    #[field_offset(0x78)] depth_stencil_view: Option<ID3D11DepthStencilView>,
    #[field_offset(0x90)] pub rasterizer: Option<ID3D11RasterizerState>,
    #[field_offset(0xa0)] pub target_vertex_shader: ID3D11VertexShader,
    #[field_offset(0xb0)] pub target_pixel_shader: ID3D11PixelShader,
    #[field_offset(0xc0)] ia_topology: D3D_PRIMITIVE_TOPOLOGY,
    #[field_offset(0x108)] resources: [DeferredContextResources; 4],
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct DeferredContextResources {
    pub buffer: [Option<ID3D11Buffer>; 14],
    pub samplers: [Option<ID3D11SamplerState>; 17],
    pub shader_resource_views: [Option<ID3D11ShaderResourceView>; 17]
}

impl DeferredContextBase {
    pub unsafe fn get_buffer(&self, ty: BufferType, buf: &ConstantBuffer) -> Option<&ID3D11Buffer> {
        self.resources.get_unchecked(ty as usize).buffer
            .get_unchecked(buf.get_slot()).as_ref()
    }
    pub unsafe fn get_buffer_ptr_raw(&self, ty: BufferType, buf: &ConstantBuffer) -> *mut *mut std::ffi::c_void {
        std::mem::transmute::<&Option<ID3D11Buffer>, *mut *mut std::ffi::c_void>(
            self.resources.get_unchecked(ty as usize).buffer.get_unchecked(buf.get_slot()))
    }
    fn are_buffers_equal(&self, ty: BufferType, buf: &ConstantBuffer) -> bool {
        let pexist = unsafe { match self.get_buffer(ty, buf) {
                Some(v) => v.as_raw(),
                None => std::ptr::null()
            }};
        let pbuf = unsafe { match buf.get_buffer().as_ref() {
                Some(v) => v.as_raw(),
                None => std::ptr::null()
            }};
        pexist == pbuf
    }

    unsafe fn set_constant_buffers_inner(&self, ty: BufferType, buf: &ConstantBuffer) {
        let ctx = &self.device_context;
        let slice = buf.get_buffer_as_slice();
        match ty {
            BufferType::Vertex => ctx.VSSetConstantBuffers(buf.slot as u32, Some(slice)),
            BufferType::Geometry => ctx.GSSetConstantBuffers(buf.slot as u32, Some(slice)),
            BufferType::Pixel => ctx.PSSetConstantBuffers(buf.slot as u32, Some(slice)),
            BufferType::Compute => ctx.CSSetConstantBuffers(buf.slot as u32, Some(slice)),
        };
    }

    pub fn draw(&mut self, topology: IATopology, vtx_count: u32, vtx_start: u32) {
        let d3d_topology = topology.into();
        if self.ia_topology != d3d_topology {
            unsafe { self.device_context.IASetPrimitiveTopology(d3d_topology); }
            self.ia_topology = d3d_topology;
        }
        unsafe { self.device_context.Draw(vtx_count, vtx_start); }
    }

    unsafe fn set_shader_sampler_inner(&mut self, ty: BufferType, id: usize, state: Option<&SamplerState>) {
        let ctx = &self.device_context;
        let slice = state.map(|f| f.get_sampler_as_slice());
        match ty {
            BufferType::Vertex => ctx.VSSetSamplers(id as u32, slice),
            BufferType::Geometry => ctx.GSSetSamplers(id as u32, slice),
            BufferType::Pixel => ctx.PSSetSamplers(id as u32, slice),
            BufferType::Compute => ctx.CSSetSamplers(id as u32, slice),
        };
    }

    unsafe fn get_sampler_ptr_raw(&self, ty: BufferType, id: usize) -> *mut *mut std::ffi::c_void {
        std::mem::transmute::<&Option<ID3D11SamplerState>, *mut *mut std::ffi::c_void>(
            self.resources.get_unchecked(ty as usize).samplers.get_unchecked(id)
        )
    }

    unsafe fn set_shader_resource_view_inner(&mut self, ty: BufferType, id: usize, state: Option<&TextureResource>) {
        let ctx = &self.device_context;
        let slice = state.map(|f| f.get_shader_resource_view_as_slice());
        match ty {
            BufferType::Vertex => ctx.VSSetShaderResources(id as u32, slice),
            BufferType::Geometry => ctx.GSSetShaderResources(id as u32, slice),
            BufferType::Pixel => ctx.PSSetShaderResources(id as u32, slice),
            BufferType::Compute => ctx.CSSetShaderResources(id as u32, slice),
        };
    }

    unsafe fn get_shader_resource_view_ptr(&self, ty: BufferType, id: usize) -> *mut *mut std::ffi::c_void {
        std::mem::transmute::<&Option<ID3D11ShaderResourceView>, *mut *mut std::ffi::c_void>(
            self.resources.get_unchecked(ty as usize).shader_resource_views.get_unchecked(id)
        )
    }
    /*
    unsafe fn check_set_render_targets(&mut self) -> bool {
        let rtv_exist = match self.render_target_view {
            Some(v) => v.as_raw(),
            None => std::ptr::null()
        };
        false
    }
    */

    unsafe fn get_render_target_view_ptr(&self) -> *mut *mut std::ffi::c_void {
        std::mem::transmute::<&Option<ID3D11RenderTargetView>, *mut *mut std::ffi::c_void>(&self.render_target_view)
    }

    unsafe fn get_depth_stencil_view_ptr(&self) -> *mut *mut std::ffi::c_void {
        std::mem::transmute::<&Option<ID3D11DepthStencilView>, *mut *mut std::ffi::c_void>(&self.depth_stencil_view)
    }

    pub unsafe fn om_set_render_targets(&mut self, rtv: Option<&ResourceView>, dsv: Option<&ResourceView2>) {
        let p_rtv = rtv.map(|f| f.get_render_target_view_as_slice());
        let p_dsv = match dsv {
            Some(v) => v.get_depth_stencil_view(),
            None => None
        };
        let e_rtv = self.get_render_target_view_ptr();
        let e_dsv = self.get_depth_stencil_view_ptr();
        let a_rtv = match rtv {
            Some(v) => v.get_render_target_view_as_raw(),
            None => std::ptr::null_mut()
        };
        // let a_rtv = (&raw const *p_rtv) as *const std::ffi::c_void as *mut std::ffi::c_void;
        let a_dsv = match p_dsv { Some(v) => v.as_raw(), None => std::ptr::null_mut() };
        if *e_rtv != a_rtv || *e_dsv != a_dsv {
            *e_rtv = a_rtv;
            *e_dsv = a_dsv;
            self.device_context.OMSetRenderTargets(p_rtv, p_dsv);
        }
        for i in 0..7 {
            if *self.field40.get_unchecked(i) == std::ptr::null_mut() { return; }
            *self.field40.get_unchecked_mut(i) = std::ptr::null_mut();
        }
    }
}

impl DeferredContext for DeferredContextBase {
    unsafe fn set_constant_buffers(&mut self, ty: BufferType, buf: &mut ConstantBuffer, upd: u32) {
        let update_flags = buf.get_resource_flag(upd);
        if (buf.active_buffers & update_flags) != 0 { 
            self.device_context.UpdateSubresource(buf.get_buffer().map(|f| f.into()), 0, None, buf.get_resource(upd), 0, 0);
            buf.active_buffers &= !update_flags; 
        }
        if !self.are_buffers_equal(ty, buf) {
            unsafe { self.set_constant_buffers_inner(ty, buf) };
            let ppexist = self.get_buffer_ptr_raw(ty, buf);
            match buf.get_buffer() {
                Some(v) => std::ptr::write(ppexist, v.as_raw()),
                None => std::ptr::write(ppexist, std::ptr::null_mut())
            };
        }
    }
    unsafe fn set_depth_stencil_state(&mut self, a2: usize, a3: u8) {
        
    }
    unsafe fn rs_set_state(&mut self, rasterizer: *mut u8) {

    }

    unsafe fn set_shader_sample(&mut self, ty: BufferType, id: usize, state: Option<&SamplerState>) {
        self.set_shader_sampler_inner(ty, id, state);
        let ppexist = self.get_sampler_ptr_raw(ty, id);
        match state {
            Some(v) => std::ptr::write(ppexist, v.get_sampler_as_raw()),
            None => std::ptr::write(ppexist, std::ptr::null_mut())
        }
    }

    unsafe fn set_shader_resource_view(&mut self, ty: BufferType, id: usize, state: Option<&TextureResource>) {
        self.set_shader_resource_view_inner(ty, id, state);
        let ppexist = self.get_shader_resource_view_ptr(ty, id);
        match state {
            Some(v) => std::ptr::write(ppexist, v.get_shader_resource_view_as_raw()),
            None => std::ptr::write(ppexist, std::ptr::null_mut())
        }
    }
    // OMSetBlendState
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
    pub cull_mode: CullMode, // pub(crate)
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
        self.hash.get_hash() == other.get_hash()
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
    pub enable_blending: bool, // crate
    pub(crate) source_blend: BlendType,
    pub(crate) dest_blend: BlendType,
    pub(crate) blend_op: BlendTypeOperation,
    pub(crate) source_blend_alpha: BlendType,
    pub(crate) dest_blend_alpha: BlendType,
    pub(crate) blend_op_alpha: BlendTypeOperation,
    pub render_mask: u32, // crate
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
    pub depth_enable: bool, // crate
    pub depth_write_mask: DepthWriteMask, // crate
    pub depth_func: ComparisonFunc, // crate
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

impl TryFrom<u32> for DepthWriteMask {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DepthWriteMask::MaskNone),
            1 => Ok(DepthWriteMask::MaskAll),
            _ => Err(())
        }
    }
}
impl TryFrom<bool> for DepthWriteMask {
    type Error = ();
    fn try_from(value: bool) -> Result<Self, Self::Error> {
        match value {
            true => Ok(DepthWriteMask::MaskAll),
            false => Ok(DepthWriteMask::MaskNone)
        }
    }
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

impl TryFrom<u32> for ComparisonFunc {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value <= ComparisonFunc::Always as u32 {
            Ok(unsafe{std::mem::transmute(value)})
        } else {
            Err(())
        }
    }
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

impl TryFrom<u8> for TextureAddressMode {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= Self::MirrorOnce as u8 {
            Ok(unsafe{std::mem::transmute(value as u32)})
        } else {
            Err(())
        }
    }
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
    pub fn get_sampler(&self) -> Option<&ID3D11SamplerState> {
        self.platform_sampler.as_ref()
    }
    pub unsafe fn get_sampler_as_slice(&self) -> &[Option<ID3D11SamplerState>] {
        std::slice::from_raw_parts(&self.platform_sampler, 1)
    }
    pub unsafe fn get_sampler_as_raw(&self) -> *mut std::ffi::c_void {
        match &self.platform_sampler {
            Some(v) => v.as_raw(),
            None => std::ptr::null_mut()
        }
    }
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

impl DrawState {
    pub unsafe fn set_state(&mut self, buffer: usize, fun: RenderStateTable, value: *const u8) {
        match fun {        
            RenderStateTable::PS3RS_ZENABLE => {
                if self.basicBuffers.get_unchecked(buffer).depth_stencil_key.depth_enable != value.is_null() {
                    self.basicBuffers.get_unchecked_mut(buffer).depth_stencil_key.depth_enable = value.is_null();
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_DEPTH_STENCIL;
                }
            },
            RenderStateTable::PS3RS_ZWRITEENABLE => {
                let depth_write = (value as u32 & 1).try_into().unwrap();
                if self.basicBuffers.get_unchecked(buffer).depth_stencil_key.depth_write_mask != depth_write {
                    self.basicBuffers.get_unchecked_mut(buffer).depth_stencil_key.depth_write_mask = depth_write;
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_DEPTH_STENCIL;
                }
            },
            RenderStateTable::PS3RS_CULLMODE => {
                let cull_type = match value as u16 {
                    1 => CullMode::None,
                    2 => CullMode::Front,
                    _ => CullMode::Back
                };
                if self.basicBuffers.get_unchecked(buffer).rasterizer_key.cull_mode != cull_type {
                    self.basicBuffers.get_unchecked_mut(buffer).rasterizer_key.cull_mode = cull_type;
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_RASTERIZER;
                }
            },
            RenderStateTable::PS3RS_ZFUNC => {
                let value = (value as u32).try_into().unwrap();
                if self.basicBuffers.get_unchecked(buffer).depth_stencil_key.depth_func != value {
                    self.basicBuffers.get_unchecked_mut(buffer).depth_stencil_key.depth_func = value;
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_DEPTH_STENCIL;
                }
            },
            RenderStateTable::PS3RS_ALPHABLENDENABLE => {
                if self.basicBuffers.get_unchecked(buffer).blend_key.enable_blending != !value.is_null() {
                    self.basicBuffers.get_unchecked_mut(buffer).blend_key.enable_blending = !value.is_null();
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_BLEND;
                }
            },
            RenderStateTable::PS3RS_COLORWRITEENABLE => {
                let val = value as u32;
                if self.basicBuffers.get_unchecked(buffer).blend_key.render_mask != val {
                    self.basicBuffers.get_unchecked_mut(buffer).blend_key.render_mask = val;
                    self.basicBuffers.get_unchecked_mut(buffer).flags |= BufferFlags::USING_BLEND;
                }
            },
            _ => ()
        }
    }
}

impl BasicBuffers {
    pub fn set_depth_stencil_depth_enable(&mut self, new: bool) {
        if self.depth_stencil_key.depth_enable != new {
            self.depth_stencil_key.depth_enable = new;
            self.flags |= BufferFlags::USING_DEPTH_STENCIL;
        }
    }
    pub fn set_depth_stencil_depth_func(&mut self, new: ComparisonFunc) {
        if self.depth_stencil_key.depth_func != new {
            self.depth_stencil_key.depth_func = new;
            self.flags |= BufferFlags::USING_DEPTH_STENCIL;
        }
    }
    pub fn set_depth_stencil_depth_write_mask(&mut self, new: DepthWriteMask) {
        if self.depth_stencil_key.depth_write_mask != new {
            self.depth_stencil_key.depth_write_mask = new;
            self.flags |= BufferFlags::USING_DEPTH_STENCIL;
        }
    }
    pub fn set_depth_stencil_stencil_enable(&mut self, new: bool) {
        if self.depth_stencil_key.stencil_enable != new {
            self.depth_stencil_key.stencil_enable = new;
            self.flags |= BufferFlags::USING_DEPTH_STENCIL;
        }
    }
    pub fn set_sampler_mask(&mut self, index: usize) {
        let mask = self.get_sampler_mask_for_index(index);
        if (self.sampler_mask & mask) == 0 {
            self.sampler_flag |= mask;
            self.sampler_mask |= mask;
        }
    }

    pub fn get_sampler_mask_for_index(&self, index: usize) -> u32 {
        1 << (index & (u32::BITS-1) as usize)
    }

    pub fn set_sampler_filter(&mut self, index: usize, new: FilterMode) {
        let sampler = unsafe { self.sampler_keys.get_unchecked_mut(index) };
        if sampler.filter != new {
            sampler.filter = new;
            self.sampler_flag |= self.get_sampler_mask_for_index(index);
        }
    }
    pub fn set_sampler_address2d(&mut self, index: usize, addru: TextureAddressMode, addrv: TextureAddressMode) {
        let sampler = unsafe { self.sampler_keys.get_unchecked_mut(index) };
        if sampler.address_u != addru || sampler.address_v != addrv {
            sampler.address_u = addru;
            sampler.address_v = addrv;
            self.sampler_flag |= self.get_sampler_mask_for_index(index);
        }
    }
}
