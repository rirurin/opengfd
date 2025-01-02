#![allow(dead_code)]

//! Stores GFD engine's global state. In a standalone application, it's the responsibility of the
//! executable crate to manage these resources properly. When hooking into an existing GFD
//! application, the hooker is responsible for obtaining the correct addresses for each of the
//! required globals.
use crate::{
    device::ngr::{
        allocator::Allocator,
        renderer::state::DrawState
    },
    kernel::global::Global
};
use opengfd_proc::create_gfd_static;
/*
use std::ops::{ Deref, DerefMut };
use std::{
    ptr::NonNull,
    sync::OnceLock
};
*/

pub struct UnsafePtr<T>(*mut T);
unsafe impl<T> Send for UnsafePtr<T> {}
unsafe impl<T> Sync for UnsafePtr<T> {}

impl<T> std::fmt::Debug for UnsafePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO!")
    }
}

create_gfd_static!(GFD_GLOBAL, Global);
create_gfd_static!(NGR_DRAW_STATE, *mut DrawState);
create_gfd_static!(NGR_ALLOCATOR, *mut Allocator);
