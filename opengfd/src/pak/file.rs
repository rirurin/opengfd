use std::ptr::NonNull;

type CriFsLoaderHn = *mut u8;

#[repr(C)]
pub struct PakFileInfo {
    endian: u32,
    mode: u32,
    filename: [u8; 128],
    size: u32,
    max_idx: u16,
    free_cnt: u16,
    ref_cnt: u16,
    buf: Option<NonNull<u8>>,
    fd: u32,
    loader: CriFsLoaderHn,
    fst_entry: *mut u8,
    fieldb8: [u8; 0x38]
}

#[repr(C)]
pub struct PakFile {
    info: PakFileInfo,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
}