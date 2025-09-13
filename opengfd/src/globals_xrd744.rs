#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from opengfd-globals.
use crate :: kernel :: global :: Global ;
#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of GFD_GLOBAL.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_gfd_global(ptr: *mut Global);
   /// Get a possible reference to GFD_GLOBAL. This checks to see if `set_gfd_global`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_gfd_global() -> Option<& 'static Global>;
   /// Like `get_gfd_global_mut`, but a mutable reference is created instead.
    pub(crate) fn get_gfd_global_mut() -> Option<& 'static mut Global>;
   /// An unchecked version of `get_gfd_global`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_gfd_global_unchecked() -> & 'static Global;
   /// An unchecked version of `get_gfd_global_mut`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) fn get_gfd_global_unchecked_mut() -> & 'static mut Global;
   /// Change the value of `GFD_GLOBAL`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_gfd_global(new: Global);
}

