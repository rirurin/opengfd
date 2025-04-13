use windows::Win32::Foundation::HINSTANCE;

#[repr(C)]
#[derive(Debug)]
pub struct VideoMode {
    flags: u32,
    width: i32,
    height: i32,
    depth: i32,
    ref_rate: i32,
    format: i32
}

#[repr(C)]
pub struct RenderInitParams {
    resource_size: u32,
    cmd_buffer_size: u32,
    gpu_buffer_size: u32,
    fps_base: u32,
    field10: u32,
    field14: u32
}

#[repr(C)]
pub struct DeviceInitParams {
    vidoe_mode: VideoMode,
    render: RenderInitParams
}

#[repr(C)]
pub struct ExecuteParams {
    flags: u32,
    stack_size: u32,
    device: DeviceInitParams,
    data_root_path: *const u8,
    data_device: *const u8,
    field48: *const u8,
    init_path: *const u8,
    sys_init_cb: fn(),
    sys_shutdown_cb: fn(),
    sys_arg: *const u8,
    cb_70: fn(),
    cb_78: fn(),
    cb_80: fn(),
    cb_88: fn(),
    cb_90: fn(),
    cb_98: fn(),
    cb_a0: fn(),
    fielda8: *const u8,
    fieldb0: *const u8,
    app_handle: HINSTANCE,
    window_name: *const u8,
    fieldc8: *const u8,
    default_msg_handler: *const u8,
    fieldd8: *const u8,
    icon: *const u8,
    fielde8: *const u8,
    fieldf0: u32,
    fieldf4: u8
}