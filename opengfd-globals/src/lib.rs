#![allow(dead_code)]

//! Stores GFD engine's global state. In a standalone application, it's the responsibility of the
//! executable crate to manage these resources properly. When hooking into an existing GFD
//! application, the hooker is responsible for obtaining the correct addresses for each of the 
//! required globals.
//!
//! This repo contains a sample for hooking onto a GFD app using the Reloaded-II mod loader
//! (opengfd-reloaded).
//!
//! This crate is separate from the main OpenGFD crate to allow for it to output as it's own
//! dynamic library which opengfd, and any other DLL that wishes to use opengfd functionality can
//! link to this so that global pointers only have to be defined once. When compiling as a
//! standalone application, this will be statically linked into the OpenGFD executable.

#[cfg(all(feature = "v1-core", feature = "v2-core"))]
compile_error!("v1-core and v2-core are mutually exclusive!");
pub struct UnsafePtr<T>(*mut T);
unsafe impl<T> Send for UnsafePtr<T> {}
unsafe impl<T> Sync for UnsafePtr<T> {}

impl<T> std::fmt::Debug for UnsafePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO!")
    }
}

#[cfg(feature = "v1-core")]
#[path = "globals_xrd744.rs"]
pub mod globals;

#[cfg(feature = "v2-core")]
#[path = "globals_xrd759.rs"]
pub mod globals;