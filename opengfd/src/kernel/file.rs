use bitflags::bitflags;

#[repr(C)]
#[derive(Debug)]
pub struct File(usize);

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FileFlags : u32 {
        const NOT_READY = 1 << 31;
    }
}