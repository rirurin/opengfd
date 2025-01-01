#![allow(dead_code)]

//! Stores GFD engine's global state. In a standalone application, it's the responsibility of the
//! executable crate to manage these resources properly. When hooking into an existing GFD
//! application, the hooker is responsible for obtaining the correct addresses for each of the
//! required globals.
use crate::{
    device::ngr::renderer::state::DrawState,
    kernel::global::Global
};
use std::ops::{ Deref, DerefMut };
use std::{
    ptr::NonNull,
    sync::OnceLock
};

pub struct UnsafePtr<T>(*mut T);
unsafe impl<T> Send for UnsafePtr<T> {}
unsafe impl<T> Sync for UnsafePtr<T> {}

impl<T> std::fmt::Debug for UnsafePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO!")
    }
}

#[derive(Debug)]
pub struct UnsafeStatic<T: 'static>(&'static mut T);
unsafe impl<T> Send for UnsafeStatic<T> {}
unsafe impl<T> Sync for UnsafeStatic<T> {}

pub static GFD_GLOBAL: OnceLock<UnsafeStatic<Global>> = OnceLock::new();
pub static NGR_DRAW_STATE: OnceLock<UnsafePtr<*mut DrawState>> = OnceLock::new();

pub fn get_gfd_global() -> *mut Global { // verified
    GFD_GLOBAL.get().unwrap().0 as *const Global as *mut Global
}

pub fn set_gfd_global(ptr: *mut Global) {
    unsafe { GFD_GLOBAL.set(UnsafeStatic(ptr.as_mut().unwrap())).unwrap(); }
}

pub fn set_ngr_draw_state(ptr: *mut *mut DrawState) {
    NGR_DRAW_STATE.set(UnsafePtr(ptr)).unwrap();
}

pub fn get_ngr_draw_state() -> Option<NonNull<DrawState>> { // verified
    let pptr = NGR_DRAW_STATE.get().unwrap().0;
    if !pptr.is_null() {
        Some(unsafe { NonNull::new_unchecked(*pptr) })
    } else {
        None
    }
}

pub unsafe fn get_ngr_draw_state_unchecked() -> NonNull<DrawState> {
    NonNull::new_unchecked(*NGR_DRAW_STATE.get().unwrap().0)
}
/*
pub unsafe fn get_gfd_global_unchecked() -> NonNull<Global> {
    NonNull::new_unchecked(GFD_GLOBAL.get().unwrap().0 as *mut Global)
}
*/
