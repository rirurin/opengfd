use riri_mod_tools_proc::ensure_layout;

#[ensure_layout(size = 40usize)]
#[derive(Debug)]
pub struct ResBuffer {
    #[field_offset(0usize)]
    pub ptr: *mut ::std::os::raw::c_void,
    #[field_offset(8usize)]
    pub offset: *mut ::std::os::raw::c_void,
    #[field_offset(16usize)]
    pub size: usize,
    #[field_offset(24usize)]
    pub owner: *mut Resources,
}

// #[ensure_layout(size = 0x34)]
#[derive(Debug)]
pub struct Resources {
}
