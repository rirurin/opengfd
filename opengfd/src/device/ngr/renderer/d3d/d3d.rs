#![allow(non_snake_case, unused_imports)]
use allocator_api2::alloc::{ Allocator, Global };
use bitflags::bitflags;
use crate::{
        device::ngr::{
        hint::MemHint,
        renderer::state::{ RasterizerKey, RasterizerState },
        structures::{ CriticalSection, CrcHash, PointerList }
    },
    globals
};
use std::alloc::Layout;
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
                D3D11_CREATE_DEVICE_FLAG,
                D3D11_RASTERIZER_DESC,
                D3D11CreateDeviceAndSwapChain,
                ID3D11Device,
                ID3D11DeviceContext,
                ID3D11Texture2D
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
pub struct ngrDX11Renderer {
    vtable: *mut ::std::os::raw::c_void,
    flags0: Flags0,
    field2_0xc: i32,
    deferred_context: *mut DeferredContextSpecial,
    pub blends: PointerList<usize>,
    pub depth_stencils: PointerList<usize>,
    pub rasterizers: PointerList<RasterizerState>,
    pub samplers: PointerList<usize>,
    pub mutexes: [*mut CriticalSection; 4],
    field13_0xf8: ngr_1420f21d0,
    cmdBuffer: *mut ngrCmdBuffer,
    unk0: [u8; 0x18],
    field39_0x128: ngr_1420f21d0,
    device: Option<ID3D11Device>,
    device_context: Option<ID3D11DeviceContext>,
    swapchain: Option<IDXGISwapChain>,
    factory: IDXGIFactory,
    feature_level: D3D_FEATURE_LEVEL,
    unk1: [u8; 0xc],
    flags: Flags1,
    presentSyncInterval: i32,
    field59_0x170: i32,
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
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM ,
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

impl ngrDX11Renderer {
    pub unsafe fn new_in<A: Allocator>(params: &InitParams, alloc: A) -> &'static mut Self {
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
        let res = out.create_resource_views(alloc);
        println!("ngr: Created DirectX11 renderer");
        out
    }
    pub unsafe fn create_resource_views<A: Allocator>(&mut self, alloc: A) {
        // TODO!
        let tex2d = self.swapchain.as_ref().unwrap().GetBuffer::<ID3D11Texture2D>(2).unwrap();
        let rtv = self.device.as_ref().unwrap().CreateRenderTargetView(&tex2d, None, None).unwrap();
    }
    pub fn try_get_rasterizer_state(&mut self, key: &RasterizerKey) -> Option<&RasterizerState> {
        let hash = CrcHash::new(key);
        self.rasterizers.find_by_predicate(|f| f == key && f == &hash)
    }
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
pub struct ngrCmdBuffer {
    pub vtable: *mut ::std::os::raw::c_void,
    pub field1_0x8: *mut ::std::os::raw::c_void,
    pub bufStart: *mut ::std::os::raw::c_void,
    pub bufSize: i32,
}

#[ensure_layout(size = 0xa78)]
pub struct ngr_142ed6270 {
    #[field_offset(0x8)] count0: usize,
    #[field_offset(0x10)] count1: usize,
    #[field_offset(0xa30)] hint: MemHint,
    #[field_offset(0xa48)] hwnd: HWND,
}
