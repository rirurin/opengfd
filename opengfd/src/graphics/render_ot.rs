use crate::{
    globals,
    graphics::cull::CullObject
};
use std::mem::size_of;

// Render OT linked list
#[repr(C)]
#[derive(Debug)]
pub struct RenderOtListInner {
    head: *mut RenderOt,
    tail: *mut RenderOt,
}

impl RenderOtListInner {
    pub unsafe fn insert_entry(&mut self, new: &mut RenderOt) {
        // Are we the first entry in the linked list?
        if self.head.is_null() {
            self.head = &raw mut *new;
        } else {
            // Append onto the end of the list
            (&mut *self.tail).next = &raw mut *new;
        }
        self.tail = &raw mut *new;
    }

    pub unsafe fn insert_entry_culled(&mut self, new: &mut RenderOt) {
        // Are we the first entry in the linked list?
        if self.head.is_null() {
            self.head = &raw mut *new;
        } else {
            // Append onto the end of the list
            (&mut *self.tail).next_cull = &raw mut *new;
        }
        self.tail = &raw mut *new;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtList {
    field0: [u8; 0x18],
    render: RenderOtListInner,
    cull: RenderOtListInner,
    field38: [u8; 8]
}

type RenderOtReturn<T> = Result<T, ()>;
#[cfg(feature = "v1-core")]
type RenderOtCallback = fn(*mut RenderOt, *mut u8) -> ();
#[cfg(feature = "v2-core")]
type RenderOtCallback = unsafe extern "C" fn(*mut RenderOt, *mut u8, *mut u8) -> ();

pub unsafe trait RenderOtBase {
    fn set_data<T>(&mut self, data: *const T);
    fn set_pre_cb(&mut self, cb: RenderOtCallback);
    fn set_pre_cb_data(&mut self, data: *const u8);
    fn set_post_cb(&mut self, cb: RenderOtCallback);
    fn set_post_cb_data(&mut self, data: *const u8);
    unsafe fn link(&mut self, prio: u32);
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RenderOt {
    data: *mut u8,
    pre_cb_data: *mut u8,
    post_cb_data: *mut u8,
    pre_cb: Option<RenderOtCallback>,
    post_cb: Option<RenderOtCallback>,
    next: *mut RenderOt,
    next_cull: *mut RenderOt,
    geometry_cull: *mut CullObject
}


impl RenderOt {
    /// (Original function: gfdRenderOtSetup)
    pub unsafe fn setup(extra: usize) -> *mut Self {
        let cmd_buffer = globals::get_gfd_global_unchecked_mut().graphics.get_current_cmd_buffer();
        let res = cmd_buffer.alloc_type::<Self>(extra); 
        (&mut *res).geometry_cull = globals::get_gfd_global_unchecked().graphics.get_geometry_cull();
        res
    } 
}

unsafe impl RenderOtBase for RenderOt {
    fn set_data<T>(&mut self, data: *const T) {
        self.data = data as *mut u8;
    }
    fn set_pre_cb(&mut self, cb: RenderOtCallback) {
        self.pre_cb = Some(cb);
    }
    fn set_pre_cb_data(&mut self, data: *const u8) {
        self.pre_cb_data = data as *mut u8;
    }
    fn set_post_cb(&mut self, cb: RenderOtCallback) {
        self.post_cb = Some(cb);
    }
    fn set_post_cb_data(&mut self, data: *const u8) {
        self.post_cb_data = data as *mut u8;
    }
    unsafe fn link(&mut self, prio: u32) {
        let glb = globals::get_gfd_global_unchecked_mut();
        let ot_list = glb.graphics.get_ot_render_list(glb.graphics.get_frame_id(), prio as usize);
        if !self.geometry_cull.is_null() {
            self.next_cull = std::ptr::null_mut();
            ot_list.cull.insert_entry_culled(self);
        } else {
            self.next = std::ptr::null_mut();
            ot_list.render.insert_entry(self);
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug)]
pub struct RenderOtEx<const D: usize = 0> {
    base: RenderOt,
    data: [u8; D],
}

impl<const D: usize> RenderOtEx<D> {
    pub fn new() -> &'static mut Self {
        let cmd_buffer = unsafe { globals::get_gfd_global_unchecked_mut().graphics.get_current_cmd_buffer() };
        let res = unsafe { cmd_buffer.alloc_ex::<Self>() };
        unsafe {
            libc::memset(
                res as *mut RenderOtEx<D> as *mut libc::c_void, 
                0 as libc::c_int, 
                std::mem::size_of::<Self>() as libc::size_t
            );
        } 
        res.base.geometry_cull = unsafe { globals::get_gfd_global_unchecked().graphics.get_geometry_cull() };
        res
    }

    pub unsafe fn get<T: Copy>(&self, offset: usize) -> RenderOtReturn<T> {
        if offset + size_of::<T>() <= D { 
            Ok(*((&raw const self.data[offset]) as *const T))
        } else {
            Err(())
        }
    }

    pub unsafe fn data_raw(&self) -> *const u8 { self.data.as_ptr() }

    pub unsafe fn set<T: Copy>(&mut self, offset: usize, value: T) -> RenderOtReturn<()> {
        if offset + size_of::<T>() <= D { 
            *((&raw mut self.data[offset]) as *mut T) = value;
            Ok(())
        } else {
            Err(())
        } 
    }
}

unsafe impl<const D: usize> RenderOtBase for RenderOtEx<D> {
    fn set_data<T>(&mut self, data: *const T) {
        self.base.set_data(data)
    }
    fn set_pre_cb(&mut self, cb: RenderOtCallback) {
        self.base.set_pre_cb(cb)
    }
    fn set_pre_cb_data(&mut self, data: *const u8) {
        self.base.set_pre_cb_data(data)
    }
    fn set_post_cb(&mut self, cb: RenderOtCallback) {
        self.base.set_post_cb(cb)
    }
    fn set_post_cb_data(&mut self, data: *const u8) {
        self.base.set_post_cb_data(data)
    }
    unsafe fn link(&mut self, prio: u32) {
        self.base.link(prio)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderOtGroup {
    prio_ot: [u32; 7]
}
