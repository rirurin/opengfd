#![allow(non_snake_case, unused_imports)]
use allocator_api2::alloc::{ Allocator, Global };
use bitflags::bitflags;
use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        hint::MemHint,
        renderer::{
            bytecode::ShaderBytecode,
            cbuffer::ConstantBuffer,
            shader::{
                ComputeShaderPlatform,
                GeometryShaderPlatform,
                PixelShaderPlatform,
                ShaderPlatform,
                VertexShaderPlatform
            },
            state::{ 
                BlendKey,
                BlendState,
                BufferObject,
                DepthStencilKey,
                DepthStencilState,
                PipelineStateObject,
                RasterizerKey, 
                RasterizerState,
                SamplerKey,
                SamplerState
            },
        },
        structures::{ 
            CriticalSection, 
            CrcHash, 
            PointerList,
            StringHashed
        }
    },
    graphics::render::cmd_buffer::CmdBufferInterface,
    globals,
    utility::reference::{ GfdRcType, GfdRc, Reference }
};
use opengfd_proc::GfdRcAuto;
use std::{
    alloc::Layout,
    sync::atomic::Ordering
};
use riri_mod_tools_proc::ensure_layout;
use windows::{
    core::Interface,
    Win32::{
        Foundation::{ BOOL, HWND },
        Graphics::{
            Direct3D::{
                D3D_DRIVER_TYPE_HARDWARE,
                D3D_FEATURE_LEVEL,
            },
            Direct3D11::{
                D3D11_BLEND_DESC,
                D3D11_CREATE_DEVICE_FLAG,
                D3D11_DEPTH_STENCIL_DESC,
                D3D11_DEPTH_STENCIL_VIEW_DESC,
                D3D11_RASTERIZER_DESC,
                D3D11_SHADER_RESOURCE_VIEW_DESC,
                D3D11_TEXTURE1D_DESC,
                D3D11_TEXTURE2D_DESC,
                D3D11_TEXTURE3D_DESC,
                D3D11CreateDeviceAndSwapChain,
                ID3D11BlendState,
                ID3D11Buffer,
                ID3D11Device,
                ID3D11DeviceContext,
                ID3D11DepthStencilState,
                ID3D11DepthStencilView,
                ID3D11RasterizerState,
                ID3D11RenderTargetView,
                ID3D11Resource,
                ID3D11SamplerState,
                ID3D11ShaderResourceView,
                ID3D11Texture1D,
                ID3D11Texture2D,
                ID3D11Texture3D,
                ID3D11VertexShader
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT,
                    DXGI_MODE_DESC,
                    DXGI_MODE_SCALING_UNSPECIFIED,
                    DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                    DXGI_SAMPLE_DESC,
                    DXGI_RATIONAL
                },
                DXGI_MWA_NO_WINDOW_CHANGES,
                DXGI_MWA_NO_ALT_ENTER,
                DXGI_SWAP_CHAIN_DESC,
                DXGI_SWAP_EFFECT_SEQUENTIAL,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
                IDXGIAdapter,
                IDXGIDevice1,
                IDXGIFactory,
                IDXGISwapChain
            }
        }
    }
};

#[repr(C)]
#[derive(Debug)]
pub struct InitParams {
    field0: usize,
    field8: u32,
    field10: usize,
    width: u32,
    height: u32,
    refresh_rate: u32,
    dxgi_format: u32,
    use_srgb: bool,
    field28: u32,
    field2c: u32,
    field30: u32,
    field34: u32,
    field38: u32,
    ngr_malloc: *mut u8,
    field48: u32,
    field50: usize,
    field58: usize,
    field60: usize
}

impl InitParams {
    pub fn get_dxgi_format(&self) -> DXGI_FORMAT {
        if self.use_srgb {
            DXGI_FORMAT_TABLE0[self.dxgi_format as usize]
        } else {
            DXGI_FORMAT_TABLE1[self.dxgi_format as usize]
        }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Flags0 : u32 {
        const Flag2 = 1 << 2;
    }
}
bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Flags1 : u32 {
        const Flag0 = 1 << 0;
        const Flag1 = 1 << 1;
        const Flag2 = 1 << 2;
        const Flag3 = 1 << 3;
        const Flag4 = 1 << 4;
        const Flag5 = 1 << 5;
        const Flag6 = 1 << 6;
    }
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct ngrDX11Renderer<A = AllocatorHook>
where A: Allocator + Clone
{
    vtable: *mut ::std::os::raw::c_void,
    flags0: Flags0,
    field2_0xc: i32,
    deferred_context: *mut DeferredContextSpecial,
    pub blends: PointerList<BlendState>,
    pub depth_stencils: PointerList<DepthStencilState>,
    pub rasterizers: PointerList<RasterizerState>,
    pub samplers: PointerList<SamplerState>,
    pub mutexes: [*mut CriticalSection; 4],
    field13_0xf8: ngr_1420f21d0,
    cmdBuffer: *mut PlatformCmdBuffer,
    unk0: [u8; 0x8],
    use_srgb: u32,
    unk2: [u8; 0xc],
    field39_0x128: ngr_1420f21d0,
    device: Option<ID3D11Device>,
    device_context: Option<ID3D11DeviceContext>,
    swapchain: Option<IDXGISwapChain>,
    factory: IDXGIFactory,
    feature_level: D3D_FEATURE_LEVEL,
    unk1: [u8; 0x4],
    resource_view: *mut ResourceView3,
    flags: Flags1,
    presentSyncInterval: i32,
    field59_0x170: i32,
    _allocator: A
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceView3 {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    view: *mut ResourceView,
    view2: *mut ResourceView2
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceView {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    render_target_view: Option<ID3D11RenderTargetView>,
    texture_resource: *mut TextureResource,
    field28: u32
}

impl ResourceView {
    pub unsafe fn get_render_target_view_as_slice(&self) -> &[Option<ID3D11RenderTargetView>] {
        std::slice::from_raw_parts(&self.render_target_view, 1)
    }
    pub unsafe fn get_render_target_view_as_raw(&self) -> *mut std::ffi::c_void {
        match &self.render_target_view { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceView2 {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    depth_stencil_view: Option<ID3D11DepthStencilView>,
    texture_resources: [ *mut TextureResource; 2 ],
    field30: u32
}

impl ResourceView2 {
    pub fn get_depth_stencil_view(&self) -> Option<&ID3D11DepthStencilView> {
        self.depth_stencil_view.as_ref()
    }
    pub fn get_depth_stencil_view_as_raw(&self) -> *mut std::ffi::c_void {
        match &self.depth_stencil_view { Some(v) => v.as_raw(), None => std::ptr::null_mut() }
    }
}

pub static NGR_FEATURE_LEVEL: [D3D_FEATURE_LEVEL; 6] = [
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0,
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_10_1,
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_10_0,
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_3,
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_2,
    windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_9_1,
];

pub static DXGI_FORMAT_TABLE0: [DXGI_FORMAT; 12] = [
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_UNKNOWN,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R11G11B10_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16B16A16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D24_UNORM_S8_UINT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D16_UNORM,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D32_FLOAT,
];

pub static DXGI_FORMAT_TABLE1: [DXGI_FORMAT; 12] = [
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_UNKNOWN,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R11G11B10_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16B16A16_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D24_UNORM_S8_UINT,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D16_UNORM,
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D32_FLOAT,
];

static EFFECT_SCALE: [f32; 8] = [
    0.5f32, 0.5714286f32, 0.66666f32, 0.8f32,
    1.0f32, 1.3333333f32, 2.0f32, 0.0f32
];

impl<A> ngrDX11Renderer<A>
where A: Allocator + Clone
{
    pub unsafe fn new_in(params: &InitParams, alloc: A) -> &'static mut Self {
        let hwnd = globals::get_ngr_window_unchecked();
        let out = &mut* (alloc.allocate_zeroed(Layout::new::<Self>()).unwrap().as_ptr() as *mut Self);
        let desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width: params.width,
                Height: params.height,
                RefreshRate: DXGI_RATIONAL { Numerator: 30, Denominator: 1 },
                Format: params.get_dxgi_format(),
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_UNSPECIFIED
            },
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 3,
            OutputWindow: hwnd.hwnd,
            Windowed: BOOL(0),
            SwapEffect: DXGI_SWAP_EFFECT_SEQUENTIAL,
            Flags: 2 // DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH
        };
        D3D11CreateDeviceAndSwapChain(
            None, // first adapter
            D3D_DRIVER_TYPE_HARDWARE,
            None, // we're not using software rendering
            D3D11_CREATE_DEVICE_FLAG(0),
            Some(&NGR_FEATURE_LEVEL),
            7,
            Some(&raw const desc),
            Some(&raw mut out.swapchain),
            Some(&raw mut out.device),
            Some(&raw mut out.feature_level),
            Some(&raw mut out.device_context)
        ).unwrap();
        if out.feature_level.0 < windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0.0 {
            panic!("TODO: Handle unsupported D3D version");
        }
        // TODO: Fail gracefully
        let device = out.device.as_ref().unwrap().cast::<IDXGIDevice1>().unwrap();
        let adapter = device.GetAdapter().unwrap();
        // Avoid dropping uninitialized IDXGIFactory
        std::ptr::write(&raw mut out.factory, adapter.GetParent::<IDXGIFactory>().unwrap());
        out.factory.MakeWindowAssociation(hwnd.hwnd, DXGI_MWA_NO_ALT_ENTER).unwrap();
        out.factory.MakeWindowAssociation(hwnd.hwnd, DXGI_MWA_NO_WINDOW_CHANGES).unwrap();
        out.flags0 = Flags0::Flag2;
        out.flags = Flags1::Flag0 | Flags1::Flag1 | Flags1::Flag2 | Flags1::Flag3 | Flags1::Flag4 | Flags1::Flag5;
        out._allocator = alloc;
        out.create_resource_views(0, 0);
        println!("ngr: Created DirectX11 renderer");
        out
    }

    pub fn try_get_rasterizer_state(&self, key: &RasterizerKey) -> Option<&RasterizerState> {
        let hash = CrcHash::new(key);
        self.rasterizers.find_by_predicate(|f| f == key && f == &hash)
    }
    pub fn try_get_rasterizer_state_mut(&self, key: &RasterizerKey) -> Option<&mut RasterizerState> {
        let hash = CrcHash::new(key);
        self.rasterizers.find_by_predicate_mut(|f| f == key && f == &hash)
    }
    pub fn get_or_create_rasterizer(&mut self, key: &RasterizerKey) -> GfdRc<RasterizerState, AllocatorHook> {
        let mut lock = unsafe { (&mut **self.mutexes.get_unchecked_mut(1)).lock(self) };
        match (*lock).try_get_rasterizer_state_mut(key) {
            Some(n) => GfdRc::clone_from_raw(n, AllocatorHook),
            None => {
                let mut new = GfdRc::new_in(RasterizerState::new(key), AllocatorHook);
                (*lock).add_to_rasterizer_list(&mut *new, AllocatorHook);
                (*lock).set_rasterizer_state(&mut *new);
                new
            }
        }
    }
    pub fn add_to_rasterizer_list(&mut self, state: &mut RasterizerState, alloc: AllocatorHook) {
        let state_rc = GfdRc::clone_from_raw(&raw const *state, alloc);
        self.rasterizers.add_in_rc(state_rc);
    }

    fn try_get_blend_state_mut(&self, key: &BlendKey) -> Option<&mut BlendState> {
        let hash = CrcHash::new(key);
        self.blends.find_by_predicate_mut(|f| f == key && f == &hash)
    }

    fn add_to_blend_list(&mut self, state: &mut BlendState, alloc: AllocatorHook) {
        let state_rc = GfdRc::clone_from_raw(&raw const *state, alloc);
        self.blends.add_in_rc(state_rc);
    }

    pub fn get_or_create_blend_state(&mut self, key: &BlendKey) -> GfdRc<BlendState, AllocatorHook> {
        let mut lock = unsafe { (&mut **self.mutexes.get_unchecked_mut(1)).lock(self) };
        match (*lock).try_get_blend_state_mut(key) {
            Some(n) => GfdRc::clone_from_raw(n, AllocatorHook),
            None => {
                let mut new = GfdRc::new_in(BlendState::new(key), AllocatorHook);
                (*lock).add_to_blend_list(&mut *new, AllocatorHook);
                (*lock).set_blend_state(&mut *new);
                new
            }
        }
    }

    fn try_get_depth_stencil_state_mut(&self, key: &DepthStencilKey) -> Option<&mut DepthStencilState> {
        let hash = CrcHash::new(key);
        self.depth_stencils.find_by_predicate_mut(|f| f == key && f == &hash)
    }

    fn add_to_depth_stencil_list(&mut self, state: &mut DepthStencilState, alloc: AllocatorHook) {
        let state_rc = GfdRc::clone_from_raw(&raw const *state, alloc);
        self.depth_stencils.add_in_rc(state_rc);
    }

    pub fn get_or_create_depth_stencil_state(&mut self, key: &DepthStencilKey) -> GfdRc<DepthStencilState, AllocatorHook> {
        let mut lock = unsafe { (&mut **self.mutexes.get_unchecked_mut(1)).lock(self) };
        match (*lock).try_get_depth_stencil_state_mut(key) {
            Some(n) => GfdRc::clone_from_raw(n, AllocatorHook),
            None => {
                let mut new = GfdRc::new_in(DepthStencilState::new(key), AllocatorHook);
                (*lock).add_to_depth_stencil_list(&mut *new, AllocatorHook);
                (*lock).set_depth_stencil_state(&mut *new);
                new
            }
        }
    }

    fn try_get_sampler_mut(&self, key: &SamplerKey) -> Option<&mut SamplerState> {
        let hash = CrcHash::new(key);
        self.samplers.find_by_predicate_mut(|f| f == key && f == &hash)
    }

    fn add_to_sampler_list(&mut self, state: &mut SamplerState, alloc: AllocatorHook) {
        let state_rc = GfdRc::clone_from_raw(&raw const *state, alloc);
        self.samplers.add_in_rc(state_rc);
    }

    pub fn get_or_create_sampler_state(&mut self, key: &SamplerKey) -> GfdRc<SamplerState, AllocatorHook> {
        let mut lock = unsafe { (&mut **self.mutexes.get_unchecked_mut(1)).lock(self) };
        match (*lock).try_get_sampler_mut(key) {
            Some(n) => GfdRc::clone_from_raw(n, AllocatorHook),
            None => {
                let mut new = GfdRc::new_in(SamplerState::new(key), AllocatorHook);
                (*lock).add_to_sampler_list(&mut *new, AllocatorHook);
                (*lock).set_sampler_state(&mut *new);
                new
            }
        }
    }

    pub fn get_command_buffer(&self) -> Option<&PlatformCmdBuffer> { unsafe { self.cmdBuffer.as_ref() } }
    pub unsafe fn get_command_buffer_unchecked(&self) -> &PlatformCmdBuffer { &*self.cmdBuffer }
    pub fn get_command_buffer_mut(&mut self) -> Option<&mut PlatformCmdBuffer> { unsafe { self.cmdBuffer.as_mut() } }
    pub unsafe fn get_command_buffer_unchecked_mut(&mut self) -> &mut PlatformCmdBuffer { &mut *self.cmdBuffer }
    
    // VTABLE ENTRIES
    
    pub fn get_window_width(&self) -> usize {
        unsafe {
            let vtable_offset = self.vtable.add(0x8);
            let state_func = *std::mem::transmute::<
                *mut std::ffi::c_void, 
                *const fn(*mut Self) -> usize
            >(vtable_offset);
            (state_func)((&raw const *self) as *mut Self)
        }
    }

    pub fn get_window_height(&self) -> usize {
        unsafe {
            let vtable_offset = self.vtable.add(0x10);
            let state_func = *std::mem::transmute::<
                *mut std::ffi::c_void, 
                *const fn(*mut Self) -> usize
            >(vtable_offset);
            (state_func)((&raw const *self) as *mut Self)
        }
    }

    pub fn get_effect_scale_width(&self) -> usize {
        let draw_state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        match draw_state.effect_scale_adjust {
            true => (self.get_window_width() as f32 * unsafe { *EFFECT_SCALE.get_unchecked(draw_state.effect_scale_index as usize)}) as usize,
            false => self.get_window_width()
        }
    }

    pub fn get_effect_scale_height(&self) -> usize {
        let draw_state = unsafe { globals::get_ngr_draw_state_unchecked_mut() };
        match draw_state.effect_scale_adjust {
            true => (self.get_window_height() as f32 * unsafe { *EFFECT_SCALE.get_unchecked(draw_state.effect_scale_index as usize)}) as usize,
            false => self.get_window_height()
        }
    }

    pub fn resize_buffers(&self, width: u64, height: u64, width2: u32, height2: u32) {
        unsafe {
            let vtable_offset = self.vtable.add(0x30);
            let state_func = *std::mem::transmute::<
                *mut std::ffi::c_void, 
                *const fn(*mut Self, u64, u64, u32, u32)
            >(vtable_offset);
            (state_func)((&raw const *self) as *mut Self, width, height, width2, height2);
        }
    }
    
    // impl Drop: vtable + 0x38 and vtable + 0x40
    // 

    pub unsafe fn create_resource_views(&mut self, width: usize, height: usize) {
        // TODO!
        let tex2d = self.swapchain.as_ref().unwrap().GetBuffer::<ID3D11Texture2D>(0).unwrap();
        let mut rtv: Option<ID3D11RenderTargetView> = None;
        self.device.as_ref().unwrap().CreateRenderTargetView(&tex2d, None, Some(&raw mut rtv)).unwrap();
    }

    // vtable + 0xc0
    pub fn set_blend_state(&self, state: &mut BlendState) {
        let desc = state.get_key().clone().into();
        unsafe { self.device.as_ref().unwrap().CreateBlendState(&raw const desc, state.get_platform_state_ptr()).unwrap(); }
    }
    // vtable + 0xc8
    pub fn set_depth_stencil_state(&mut self, state: &mut DepthStencilState) {
        let desc = state.get_key().clone().into();
        unsafe { self.device.as_ref().unwrap().CreateDepthStencilState(&raw const desc, state.get_platform_state_ptr()).unwrap(); }
    }
    // vtable + 0xd0
    pub fn set_rasterizer_state(&self, state: &mut RasterizerState) {
        let desc = state.get_key().clone().into();
        unsafe { self.device.as_ref().unwrap().CreateRasterizerState(&raw const desc, state.get_platform_state_ptr()).unwrap(); }
    }
    // vtable + 0xd8
    pub fn set_sampler_state(&mut self, state: &mut SamplerState) {
        let desc = state.get_key().clone().into();
        unsafe { self.device.as_ref().unwrap().CreateSamplerState(&raw const desc, state.get_platform_state_ptr()).unwrap(); }
    }
    // vtable + 0xe0
    pub fn create_vertex_shader(&self, shader: &mut VertexShaderPlatform, bytecode: &&ShaderBytecode) {
        unsafe { self.device.as_ref().unwrap().CreateVertexShader(bytecode.as_slice(), None, shader.get_shader_ptr()).unwrap(); }
        // shader.create_input_layout(std::ptr::null_mut()); // TODO!
    }
    // vtable + 0xe8
    pub fn create_geometry_shader(&self, shader: &mut GeometryShaderPlatform, bytecode: &&ShaderBytecode) {
        unsafe { self.device.as_ref().unwrap().CreateGeometryShader(bytecode.as_slice(), None, shader.get_shader_ptr()).unwrap(); }
    }
    // vtable + 0xf0
    pub fn create_pixel_shader(&self, shader: &mut PixelShaderPlatform, bytecode: &&ShaderBytecode) {
        unsafe { self.device.as_ref().unwrap().CreatePixelShader(bytecode.as_slice(), None, shader.get_shader_ptr()).unwrap(); }
    }
    // vtable + 0xf8
    pub fn create_compute_shader(&self, shader: &mut ComputeShaderPlatform, bytecode: &&ShaderBytecode) {
        unsafe { self.device.as_ref().unwrap().CreateComputeShader(bytecode.as_slice(), None, shader.get_shader_ptr()).unwrap(); }
    }
    // vtable + 0x100
    pub const fn vtable_32(&self) -> bool { true }
    // vtable + 0x108
    pub fn create_constant_buffer(&self, cbuffer: &mut ConstantBuffer) {

    }
    // vtable + 0x110
    pub const fn vtable_34(&self) -> bool { false }
}

// #[ensure_layout(size = 1880)]
#[repr(C)]
#[derive(Debug)]
pub struct DeferredContextSpecial {
    pub _super: usize
    // pub _super: DeferredContextBase
    // #[field_offset(0)] pub _super: DeferredContextBase
}

#[repr(C)]
#[derive(Debug)]
pub struct ngr_1422a73b0 {
    pub _cpp_vtable: *mut ::std::os::raw::c_void,
    pub field08: *mut u8,
    pub field10: *mut u8,
    pub field18: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct ngr_1420f21d0 {
    pub field0_0x0: *mut ::std::os::raw::c_void,
    pub field1_0x8: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PlatformCmdBuffer {
    pub _cpp_vtable: *const std::ffi::c_void,
    pub buffers: *mut CommandBuffer,
    pub bufStart: *mut std::ffi::c_void,
    pub bufSize: std::sync::atomic::AtomicI32,
}

impl CmdBufferInterface for PlatformCmdBuffer {
    unsafe fn alloc(&mut self, size: i32) -> *mut u8 {
        let offset = self.bufSize.fetch_add(size, Ordering::Relaxed) as usize;
        self.bufStart.add(offset) as *mut u8
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandBuffer {
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    buffers: [Option<ID3D11Buffer>; 3],
    field30: usize,
    field38: u32,
    field40: usize
}

impl BufferObject for CommandBuffer {
    unsafe fn get_buffer(&self, index: usize) -> Option<&ID3D11Buffer> {
        self.buffers.get_unchecked(index).as_ref()
    }
    unsafe fn get_buffer_mut(&mut self, index: usize) -> Option<&mut ID3D11Buffer> {
        self.buffers.get_unchecked_mut(index).as_mut()
    }
    unsafe fn get_buffer_ptr(&self, index: usize) -> *const Option<ID3D11Buffer> {
        &raw const *self.buffers.get_unchecked(index)
    }
    unsafe fn get_buffer_raw(&self, index: usize) -> *mut std::ffi::c_void {
        match self.buffers.get_unchecked(index) { Some(v) => v.as_raw(), None => std::ptr::null_mut() } 
    }
}

#[ensure_layout(size = 0xa78)]
pub struct ngr_142ed6270 {
    #[field_offset(0x8)] count0: usize,
    #[field_offset(0x10)] count1: usize,
    #[field_offset(0xa30)] hint: MemHint,
    #[field_offset(0xa48)] hwnd: HWND,
}

#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct TextureResource<A = AllocatorHook>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    ref_: Reference,
    field10: usize,
    name: StringHashed<AllocatorHook>,
    field50: u32,
    field58: usize,
    shader_view: Option<ID3D11ShaderResourceView>,
    field68: usize,
    resource: Option<ID3D11Resource>,
    field78: usize,
    field80: usize,
    field88: usize,
    desc: TextureResourceDescription,
    _allocator: A
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureResourceType {
    Texture1D = 0,
    Texture2D,
    Texture3D
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextureResourceFormat {
    Unknown = 0,
    U8RGBA,
    U8BGRA,
    F32R,
    F16R,
    F16RG,
    F11RGB,
    F16RGBA,
    F32RGBA,
    U8R,
    U8A,
    S8D24,
    U8D16,
    F32D,
    U8BC1,
    U8BC2,
    U8BC3,
    U8BC4,
    U8BC5,
    U8BC7,
}

#[repr(C)]
#[derive(Debug)]
pub struct TextureResourceDescription {
    ty: TextureResourceType,
    width: u32,
    height: u32,
    depth: u32,
    mip_levels: u32,
    array_size: u32,
    format: TextureResourceFormat,
    field1c: u32
}

impl From<D3D11_TEXTURE1D_DESC> for TextureResourceDescription {
    fn from(value: D3D11_TEXTURE1D_DESC) -> Self {
        Self {
            ty: TextureResourceType::Texture1D,
            width: value.Width,
            height: 0,
            depth: 0,
            mip_levels: value.MipLevels,
            array_size: 0,
            format: value.Format.into(),
            field1c: 0
        }
    }
}

impl From<D3D11_TEXTURE2D_DESC> for TextureResourceDescription {
    fn from(value: D3D11_TEXTURE2D_DESC) -> Self {
        Self {
            ty: TextureResourceType::Texture2D,
            width: value.Width,
            height: value.Height,
            depth: 0,
            mip_levels: value.MipLevels,
            array_size: 0,
            format: value.Format.into(),
            field1c: 0
        }
    }
}

impl From<D3D11_TEXTURE3D_DESC> for TextureResourceDescription {
    fn from(value: D3D11_TEXTURE3D_DESC) -> Self {
        Self {
            ty: TextureResourceType::Texture3D,
            width: value.Width,
            height: value.Height,
            depth: value.Depth,
            mip_levels: value.MipLevels,
            array_size: 0,
            format: value.Format.into(),
            field1c: 0
        }
    }
}

impl From<DXGI_FORMAT> for TextureResourceFormat {
    fn from(value: DXGI_FORMAT) -> Self {
        match value {
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM => TextureResourceFormat::U8RGBA,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => TextureResourceFormat::U8RGBA,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_B8G8R8A8_UNORM  => TextureResourceFormat::U8BGRA,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_FLOAT   => TextureResourceFormat::F32R,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_FLOAT    => TextureResourceFormat::F16R,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16_FLOAT    => TextureResourceFormat::F16RG,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R11G11B10_FLOAT    => TextureResourceFormat::F11RGB,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16G16B16A16_FLOAT    => TextureResourceFormat::F16RGBA,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT    => TextureResourceFormat::F32RGBA,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8_UNORM  => TextureResourceFormat::U8R,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_A8_UNORM  => TextureResourceFormat::U8A,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D24_UNORM_S8_UINT    => TextureResourceFormat::S8D24,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R24G8_TYPELESS    => TextureResourceFormat::S8D24,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D16_UNORM  => TextureResourceFormat::U8D16,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_D32_FLOAT  => TextureResourceFormat::F32D,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC1_UNORM  => TextureResourceFormat::U8BC1,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC2_UNORM  => TextureResourceFormat::U8BC2,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC3_UNORM  => TextureResourceFormat::U8BC3,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC4_UNORM  => TextureResourceFormat::U8BC4,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC5_UNORM  => TextureResourceFormat::U8BC5,
            windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC7_UNORM  => TextureResourceFormat::U8BC7,
            _ => TextureResourceFormat::Unknown
        }
    }
}
/*
impl TextureResource<AllocatorHook> {
    pub fn new() -> GfdRc<Self, AllocatorHook> {
        Self::new_in(AllocatorHook)
    }
}
*/
impl<A> TextureResource<A>
where A: Allocator + Clone
{
    /*
    pub fn new_in(alloc: A, desc: ID3D11Resource) -> GfdRc<Self, A> {
        GfdRc::new_in(Self {
            _cpp_vtable: std::ptr::null(),
            ref_: Reference::new(),
            field10: 0,
            name: StringHashed::new_in(alloc),
            field50: 0,
            field58: 0,
            field60: 0,
            field68: 0,
            resource: None,
            field78: 0,
            field80: 0,
            field88: 0,
            _allocator: alloc
        }, alloc)
    }
    */
    pub fn set_desc(&mut self, resource: ID3D11Resource) {
        self.resource = Some(resource);
        let dimension = unsafe { self.resource.as_ref().unwrap().GetType() };
        match dimension {
            windows::Win32::Graphics::Direct3D11::D3D11_RESOURCE_DIMENSION_TEXTURE1D => {
                let tex = &self.resource.as_ref().unwrap().cast::<ID3D11Texture1D>().unwrap();
                let desc: *const *mut D3D11_TEXTURE1D_DESC = std::ptr::null();
                unsafe { tex.GetDesc(*desc) };
                self.desc = unsafe { (**desc).into() };
            },
            windows::Win32::Graphics::Direct3D11::D3D11_RESOURCE_DIMENSION_TEXTURE2D => {
                let tex = &self.resource.as_ref().unwrap().cast::<ID3D11Texture2D>().unwrap();
                let desc: *const *mut D3D11_TEXTURE2D_DESC = std::ptr::null();
                unsafe { tex.GetDesc(*desc) };
                self.desc = unsafe { (**desc).into() };
            },
            windows::Win32::Graphics::Direct3D11::D3D11_RESOURCE_DIMENSION_TEXTURE3D => {
                let tex = &self.resource.as_ref().unwrap().cast::<ID3D11Texture3D>().unwrap();
                let desc: *const *mut D3D11_TEXTURE3D_DESC = std::ptr::null();
                unsafe { tex.GetDesc(*desc) };
                self.desc = unsafe { (**desc).into() };
            },
            _ => ()
        };
    }
    pub unsafe fn get_shader_resource_view_as_slice(&self) -> &[Option<ID3D11ShaderResourceView>] {
        std::slice::from_raw_parts(&self.shader_view, 1)
    }
    pub unsafe fn get_shader_resource_view_as_raw(&self) -> *mut std::ffi::c_void {
        match &self.shader_view {
            Some(v) => v.as_raw(),
            None => std::ptr::null_mut()
        }
    }
}
