use crate::state::Inspector;
use imgui::{
    Context as ImContext,
    Condition as ImCond,
    internal::RawWrapper,
    Ui as ImUI
};
use opengfd::kernel::{
    allocator::GfdAllocator,
    task::{
        Task as GfdTask,
        UpdateTask
    }
};
use std::{
    ops::{ Deref, DerefMut },
    sync::Mutex
};

pub static CONTEXT_TEST: Mutex<ImContextWrapper> = Mutex::new(ImContextWrapper(None));

#[allow(dead_code)]
pub struct ImContextWrapper(Option<ImContext>);
impl Deref for ImContextWrapper {
    type Target = Option<ImContext>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for ImContextWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
unsafe impl Send for ImContextWrapper {}
unsafe impl Sync for ImContextWrapper {}

#[no_mangle]
pub unsafe extern "C" fn inspector_reloaded_new_window(ui: *mut ImUI, ctx: *mut <ImContext as RawWrapper>::Raw) {
    if imgui::no_current_context() {
        let mut ctx_test = CONTEXT_TEST.lock().unwrap();
        (*ctx_test) = ImContextWrapper(Some(ImContext::set_current_context(ctx)));
    }
    let ui = &mut *ui;
    let state: Option<&mut GfdTask<GfdAllocator, Inspector>> = GfdTask::find_by_str_mut(Inspector::NAME);
    if let Some(state) = state {
        let ctx = state.get_work_data_mut().unwrap();

        /* 
        let mut block_lock = opengfd::io::keyboard::BLOCK_KEYBOARD_UPDATE.lock().unwrap();
        *block_lock = ui.io().want_capture_keyboard;
        let mut block_lock = opengfd::io::mouse::BLOCK_MOUSE_UPDATE.lock().unwrap();
        *block_lock = ui.io().want_capture_mouse;
        drop(block_lock);
        */
        *crate::globals::get_block_keyboard_focus_unchecked_mut() = ui.io().want_capture_keyboard;
        *crate::globals::get_block_mouse_focus_unchecked_mut() = ui.io().want_capture_mouse;
        // println!("{}, {}", *crate::globals::get_block_keyboard_focus_unchecked_mut(), *crate::globals::get_block_mouse_focus_unchecked_mut());

        let ui_into = unsafe { &mut *(&raw mut *ui) };
        ui.window("GFD Inspector for Metaphor: Refantazio")
            .size([500., 400.], ImCond::FirstUseEver)
            .position([30., 30.], ImCond::FirstUseEver)
            .build(|| {
                if let Some(_) = ui.tab_bar("Inspector Sections") {
                    for panel in ctx.panels.iter_mut() {
                        panel.draw(ui_into);
                    }
                }
            });
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_imgui_allocator(
      alloc: imgui::sys::ImGuiMemAllocFunc,
      free: imgui::sys::ImGuiMemFreeFunc,
      user: *mut std::ffi::c_void   
) { ImContext::set_allocator_functions(alloc, free, user); }

pub fn init() {
    let ver = imgui::dear_imgui_version().as_ptr() as *const i8;
    unsafe { crate::imgui_hook::add_gui_callback(inspector_reloaded_new_window, ver) };
    unsafe { crate::imgui_hook::add_allocator(set_imgui_allocator);}
}