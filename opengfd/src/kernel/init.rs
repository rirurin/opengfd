use std::ffi::c_void;
use bitflags::bitflags;
use windows::core::PCSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, HICON};

bitflags! {
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub struct VideoModeFlags : u32 {
        const IS_4K = 1 << 1;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VideoMode {
    flags: VideoModeFlags,
    width: i32,
    height: i32,
    depth: i32,
    ref_rate: i32,
    format: i32,
    #[cfg(feature = "v1-core")]
    field18: i32,
    #[cfg(feature = "v1-core")]
    field1c: i32,
}

impl VideoMode {
    pub fn new(is_4k: bool) -> Self {
        Self {
            flags: if is_4k { VideoModeFlags::IS_4K } else { VideoModeFlags::empty() },
            width: if is_4k { 3840 } else { 1920 },
            height: if is_4k { 2160 } else { 1080 },
            depth: 32,
            ref_rate: 60,
            format: 0,
            #[cfg(feature = "v1-core")]
            field18: 0,
            #[cfg(feature = "v1-core")]
            field1c: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderInitParams {
    resource_size: u32,
    cmd_buffer_size: u32,
    gpu_buffer_size: u32,
    fps_base: u32,
    field10: u32,
    field14: u8,
    field15: u8,
    field16: u8,
    field17: u8,
}

impl RenderInitParams {
    pub const fn new() -> Self {
        Self {
            resource_size: 0x8000000, // 128 MB
            cmd_buffer_size: 0x2000000, // 32 MB
            gpu_buffer_size: 0x2d00000, // 45 MB
            fps_base: 0,
            field10: 0,
            field14: 0,
            field15: 0,
            field16: 0,
            field17: 0,
        }
    }
}

impl Default for RenderInitParams {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceInitParams {
    vidoe_mode: VideoMode,
    render: RenderInitParams
}

impl DeviceInitParams {
    pub fn new(is_4k: bool) -> Self {
        Self {
            vidoe_mode: VideoMode::new(is_4k),
            render: RenderInitParams::new()
        }
    }
}

#[repr(C)]
pub struct ExecuteParams<SArg = *const c_void, AArg = *const c_void> {
    flags: u32,
    stack_size: u32,
    device: DeviceInitParams,
    data_root_path: *const u8,
    data_device: *const u8,
    field48: *const u8,
    init_path: *const u8,
    sys_init_cb: Option<fn(SArg)>,
    sys_shutdown_cb: Option<fn(SArg)>,
    sys_arg: SArg,
    app_init_cb: Option<fn(AArg)>,
    app_update_cb: Option<fn(AArg)>,
    cb_80: Option<fn(AArg)>,
    cb_88: Option<fn(AArg)>,
    cb_90: Option<fn(AArg)>,
    cb_98: Option<fn(AArg)>,
    app_shutdown: Option<fn(AArg)>,
    fielda8: Option<fn(AArg)>,
    app_arg: AArg,
    app_handle: HINSTANCE,
    window_name: *const u8,
    fieldc8: *const u8,
    default_msg_handler: Option<unsafe fn(HWND, u32, WPARAM, LPARAM) -> LRESULT>,
    icon0: HICON,
    icon1: HICON,
    fielde8: *const u8,
    fieldf0: u32,
    fieldf4: u8,
    fieldf5: u8,
    fieldf6: u8,
    fieldf7: u8
}

impl ExecuteParams {
    // Original function: CreateWindow (0x1407af780, Steam Prologue Demo 1.01)
    pub fn new(is_4k: bool) -> Self {
        Self {
            flags: 2,
            stack_size: 0,
            device: DeviceInitParams::new(is_4k),
            data_root_path: ".\\\\\0".as_ptr(),
            data_device: "\"\"\0".as_ptr(),
            field48: std::ptr::null(),
            init_path: "COMMON/init\0".as_ptr(),
            sys_init_cb: None,
            sys_shutdown_cb: None,
            sys_arg: std::ptr::null(),
            app_init_cb: None,
            app_update_cb: None,
            cb_80: None,
            cb_88: None,
            cb_90: None,
            cb_98: None,
            app_shutdown: None,
            fielda8: None,
            app_arg: std::ptr::null(),
            app_handle: unsafe { GetModuleHandleA(None).unwrap().into() },
            window_name: "OPENGFD_MAIN_WINDOW\0".as_ptr(),
            fieldc8: std::ptr::null(),
            default_msg_handler: Some(DefWindowProcW),
            icon0: HICON::default(),
            icon1: HICON::default(),
            fielde8: std::ptr::null(),
            fieldf0: 0,
            fieldf4: 1,
            fieldf5: 0,
            fieldf6: 1,
            fieldf7: 0
        }
    }
}