use std::alloc::Layout;
use allocator_api2::alloc::{AllocError, Allocator};
use std::ptr::NonNull;
use crate::device::hedge::fnd::{
    mutex::Mutex,
    tlsf_heap::TlsfHeapTemplate
};

#[repr(C)]
pub struct IAllocator(*const u8);

unsafe impl Allocator for IAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let alloc_function = unsafe { std::mem::transmute::<_, fn(&IAllocator, usize, usize) -> *mut u8>(*(self.0 as *const usize).add(1)) };
        let new_allocation = alloc_function(self, layout.size(), layout.align());
        if new_allocation != std::ptr::null_mut() {
            Ok(unsafe { NonNull::new_unchecked(&raw mut *std::slice::from_raw_parts_mut(new_allocation, layout.size())) })
        } else {
            Err(AllocError)
        }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let free_function = unsafe { std::mem::transmute::<_, fn(&IAllocator, *mut u8)>(*(self.0 as *const usize).add(2)) };
        free_function(self, ptr.as_ptr());
    }
}

// vtable:
// 0x0: ~IAllocator
// 0x8: malloc(size, align)
// 0x10: free(ptr)

// See https://github.com/angryzor/rangers-api/blob/main/rangers-api/Hedgehog/Base/System/MemoryRouter.h

// 0x14252c3a0 in P5R
#[repr(C)]
pub struct MemoryRouterAllocator {
    _super: IAllocator,
    heap: Option<NonNull<TlsfHeapTemplate<Mutex>>>,
    unk1: usize
}

unsafe impl Allocator for MemoryRouterAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self._super.allocate(layout)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self._super.deallocate(ptr, layout)
    }
}

#[repr(C)]
pub struct MemoryRouter {
    module: NonNull<MemoryRouterAllocator>,
    debug: NonNull<MemoryRouterAllocator>,
}

unsafe impl Allocator for MemoryRouter {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { self.module.as_ref().allocate(layout) }
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.module.as_ref().deallocate(ptr, layout)
    }
}