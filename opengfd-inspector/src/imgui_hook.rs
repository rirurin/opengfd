#![allow(dead_code, improper_ctypes)]
use imgui::{
    Context as ImContext,
    internal::RawWrapper,
    Ui as ImUI
};

#[link(name = "riri_imgui_hook_reloaded", kind = "raw-dylib")]
unsafe extern "C" {
    pub unsafe fn add_gui_callback(cb: unsafe extern "C" fn(*mut ImUI, *mut <ImContext as RawWrapper>::Raw));
    pub unsafe fn remove_gui_callback(cb: unsafe extern "C" fn(*mut ImUI, *mut <ImContext as RawWrapper>::Raw));
}