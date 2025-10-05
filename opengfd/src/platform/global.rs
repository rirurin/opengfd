use windows::Win32::Foundation::HWND;

#[repr(C)]
pub struct PlatformGlobal {
    window_handle: HWND,
    data: [u8; 0x5688]
}

impl PlatformGlobal {
    pub fn get_hwnd(&self) -> HWND {
        self.window_handle
    }
}