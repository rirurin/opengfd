#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from opengfd-globals.
use crate :: { device :: ngr :: { allocator :: Allocator , renderer :: { platform :: d3d :: { ngr_142ed6270 , ngrDX11Renderer } , state :: DrawState } , structures :: { ListNodeFreeList , PointerListEntry } } , io :: keyboard :: Keyboard , kernel :: { global :: Global , job :: Job } , tpl :: file_manager :: FileManager , } ;
#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of GFD_GLOBAL.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_gfd_global(ptr: *mut Global);
   /// Get a possible reference to GFD_GLOBAL. This checks to see if `set_gfd_global`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_gfd_global() -> Option<& 'static Global>;
   /// Like `get_gfd_global_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_gfd_global_mut() -> Option<& 'static mut Global>;
   /// An unchecked version of `get_gfd_global`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) unsafe fn get_gfd_global_unchecked() -> & 'static Global;
   /// An unchecked version of `get_gfd_global_mut`. This assumes that GFD_GLOBAL
    /// is set and it's initialized.
    pub(crate) unsafe fn get_gfd_global_unchecked_mut() -> & 'static mut Global;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_DRAW_STATE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_draw_state(ptr: *mut * mut DrawState);
   /// Get a possible reference to NGR_DRAW_STATE. This checks to see if `set_ngr_draw_state`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_draw_state() -> Option<& 'static DrawState>;
   /// Like `get_ngr_draw_state_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_draw_state_mut() -> Option<& 'static mut DrawState>;
   /// An unchecked version of `get_ngr_draw_state`. This assumes that NGR_DRAW_STATE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_draw_state_unchecked() -> & 'static DrawState;
   /// An unchecked version of `get_ngr_draw_state_mut`. This assumes that NGR_DRAW_STATE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_draw_state_unchecked_mut() -> & 'static mut DrawState;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_ALLOCATOR.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_allocator(ptr: *mut * mut Allocator);
   /// Get a possible reference to NGR_ALLOCATOR. This checks to see if `set_ngr_allocator`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_allocator() -> Option<& 'static Allocator>;
   /// Like `get_ngr_allocator_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_allocator_mut() -> Option<& 'static mut Allocator>;
   /// An unchecked version of `get_ngr_allocator`. This assumes that NGR_ALLOCATOR
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_allocator_unchecked() -> & 'static Allocator;
   /// An unchecked version of `get_ngr_allocator_mut`. This assumes that NGR_ALLOCATOR
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_allocator_unchecked_mut() -> & 'static mut Allocator;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_WINDOW.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_window(ptr: *mut * mut ngr_142ed6270);
   /// Get a possible reference to NGR_WINDOW. This checks to see if `set_ngr_window`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_window() -> Option<& 'static ngr_142ed6270>;
   /// Like `get_ngr_window_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_window_mut() -> Option<& 'static mut ngr_142ed6270>;
   /// An unchecked version of `get_ngr_window`. This assumes that NGR_WINDOW
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_window_unchecked() -> & 'static ngr_142ed6270;
   /// An unchecked version of `get_ngr_window_mut`. This assumes that NGR_WINDOW
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_window_unchecked_mut() -> & 'static mut ngr_142ed6270;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_DX11_RENDERER.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_dx11_renderer(ptr: *mut * mut ngrDX11Renderer);
   /// Get a possible reference to NGR_DX11_RENDERER. This checks to see if `set_ngr_dx11_renderer`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_dx11_renderer() -> Option<& 'static ngrDX11Renderer>;
   /// Like `get_ngr_dx11_renderer_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_dx11_renderer_mut() -> Option<& 'static mut ngrDX11Renderer>;
   /// An unchecked version of `get_ngr_dx11_renderer`. This assumes that NGR_DX11_RENDERER
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_dx11_renderer_unchecked() -> & 'static ngrDX11Renderer;
   /// An unchecked version of `get_ngr_dx11_renderer_mut`. This assumes that NGR_DX11_RENDERER
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_dx11_renderer_unchecked_mut() -> & 'static mut ngrDX11Renderer;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_POINTER_FREELIST.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_pointer_freelist(ptr: *mut * mut ListNodeFreeList < PointerListEntry < u8 > >);
   /// Get a possible reference to NGR_POINTER_FREELIST. This checks to see if `set_ngr_pointer_freelist`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_pointer_freelist() -> Option<& 'static ListNodeFreeList < PointerListEntry < u8 > >>;
   /// Like `get_ngr_pointer_freelist_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_pointer_freelist_mut() -> Option<& 'static mut ListNodeFreeList < PointerListEntry < u8 > >>;
   /// An unchecked version of `get_ngr_pointer_freelist`. This assumes that NGR_POINTER_FREELIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_pointer_freelist_unchecked() -> & 'static ListNodeFreeList < PointerListEntry < u8 > >;
   /// An unchecked version of `get_ngr_pointer_freelist_mut`. This assumes that NGR_POINTER_FREELIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_pointer_freelist_unchecked_mut() -> & 'static mut ListNodeFreeList < PointerListEntry < u8 > >;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to NGR_STRING_FREELIST.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_string_freelist(ptr: *mut * mut ListNodeFreeList < u8 >);
   /// Get a possible reference to NGR_STRING_FREELIST. This checks to see if `set_ngr_string_freelist`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_string_freelist() -> Option<& 'static ListNodeFreeList < u8 >>;
   /// Like `get_ngr_string_freelist_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_string_freelist_mut() -> Option<& 'static mut ListNodeFreeList < u8 >>;
   /// An unchecked version of `get_ngr_string_freelist`. This assumes that NGR_STRING_FREELIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_string_freelist_unchecked() -> & 'static ListNodeFreeList < u8 >;
   /// An unchecked version of `get_ngr_string_freelist_mut`. This assumes that NGR_STRING_FREELIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_string_freelist_unchecked_mut() -> & 'static mut ListNodeFreeList < u8 >;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_CRCHASH_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_crchash_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_CRCHASH_VTABLE. This checks to see if `set_ngr_crchash_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_crchash_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_crchash_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_crchash_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_crchash_vtable`. This assumes that NGR_CRCHASH_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_crchash_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_crchash_vtable_mut`. This assumes that NGR_CRCHASH_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_crchash_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_RASTERSTATE_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_rasterstate_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_RASTERSTATE_VTABLE. This checks to see if `set_ngr_rasterstate_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_rasterstate_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_rasterstate_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_rasterstate_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_rasterstate_vtable`. This assumes that NGR_RASTERSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_rasterstate_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_rasterstate_vtable_mut`. This assumes that NGR_RASTERSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_rasterstate_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_BLENDSTATE_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_blendstate_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_BLENDSTATE_VTABLE. This checks to see if `set_ngr_blendstate_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_blendstate_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_blendstate_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_blendstate_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_blendstate_vtable`. This assumes that NGR_BLENDSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_blendstate_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_blendstate_vtable_mut`. This assumes that NGR_BLENDSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_blendstate_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_DEPTHSTENCILSTATE_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_depthstencilstate_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_DEPTHSTENCILSTATE_VTABLE. This checks to see if `set_ngr_depthstencilstate_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_depthstencilstate_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_depthstencilstate_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_depthstencilstate_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_depthstencilstate_vtable`. This assumes that NGR_DEPTHSTENCILSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_depthstencilstate_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_depthstencilstate_vtable_mut`. This assumes that NGR_DEPTHSTENCILSTATE_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_depthstencilstate_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_SAMPLER_STATE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_sampler_state(ptr: *mut u8);
   /// Get a possible reference to NGR_SAMPLER_STATE. This checks to see if `set_ngr_sampler_state`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_sampler_state() -> Option<& 'static u8>;
   /// Like `get_ngr_sampler_state_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_sampler_state_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_sampler_state`. This assumes that NGR_SAMPLER_STATE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_sampler_state_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_sampler_state_mut`. This assumes that NGR_SAMPLER_STATE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_sampler_state_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_MEMHINT_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_memhint_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_MEMHINT_VTABLE. This checks to see if `set_ngr_memhint_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_memhint_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_memhint_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_memhint_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_memhint_vtable`. This assumes that NGR_MEMHINT_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_memhint_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_memhint_vtable_mut`. This assumes that NGR_MEMHINT_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_memhint_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_SPINLOCK_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_spinlock_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_SPINLOCK_VTABLE. This checks to see if `set_ngr_spinlock_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_spinlock_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_spinlock_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_spinlock_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_spinlock_vtable`. This assumes that NGR_SPINLOCK_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_spinlock_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_spinlock_vtable_mut`. This assumes that NGR_SPINLOCK_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_spinlock_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_1422ECAD8_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_1422ecad8_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_1422ECAD8_VTABLE. This checks to see if `set_ngr_1422ecad8_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_1422ecad8_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_1422ecad8_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_1422ecad8_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_1422ecad8_vtable`. This assumes that NGR_1422ECAD8_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_1422ecad8_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_1422ecad8_vtable_mut`. This assumes that NGR_1422ECAD8_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_1422ecad8_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of NGR_FREELIST_VTABLE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_ngr_freelist_vtable(ptr: *mut u8);
   /// Get a possible reference to NGR_FREELIST_VTABLE. This checks to see if `set_ngr_freelist_vtable`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_ngr_freelist_vtable() -> Option<& 'static u8>;
   /// Like `get_ngr_freelist_vtable_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_ngr_freelist_vtable_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_ngr_freelist_vtable`. This assumes that NGR_FREELIST_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_freelist_vtable_unchecked() -> & 'static u8;
   /// An unchecked version of `get_ngr_freelist_vtable_mut`. This assumes that NGR_FREELIST_VTABLE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_ngr_freelist_vtable_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to FILE_MANAGER_INSTANCE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_file_manager_instance(ptr: *mut * mut FileManager);
   /// Get a possible reference to FILE_MANAGER_INSTANCE. This checks to see if `set_file_manager_instance`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_file_manager_instance() -> Option<& 'static FileManager>;
   /// Like `get_file_manager_instance_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_file_manager_instance_mut() -> Option<& 'static mut FileManager>;
   /// An unchecked version of `get_file_manager_instance`. This assumes that FILE_MANAGER_INSTANCE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_file_manager_instance_unchecked() -> & 'static FileManager;
   /// An unchecked version of `get_file_manager_instance_mut`. This assumes that FILE_MANAGER_INSTANCE
    /// is set and it's initialized.
    pub(crate) unsafe fn get_file_manager_instance_unchecked_mut() -> & 'static mut FileManager;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of SOUND_PLAYER_SEND_SIGNAL.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_sound_player_send_signal(ptr: *mut u8);
   /// Get a possible reference to SOUND_PLAYER_SEND_SIGNAL. This checks to see if `set_sound_player_send_signal`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_sound_player_send_signal() -> Option<& 'static u8>;
   /// Like `get_sound_player_send_signal_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_sound_player_send_signal_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_sound_player_send_signal`. This assumes that SOUND_PLAYER_SEND_SIGNAL
    /// is set and it's initialized.
    pub(crate) unsafe fn get_sound_player_send_signal_unchecked() -> & 'static u8;
   /// An unchecked version of `get_sound_player_send_signal_mut`. This assumes that SOUND_PLAYER_SEND_SIGNAL
    /// is set and it's initialized.
    pub(crate) unsafe fn get_sound_player_send_signal_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of TPL_RESOURCE_SHARED_PTR.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_tpl_resource_shared_ptr(ptr: *mut u8);
   /// Get a possible reference to TPL_RESOURCE_SHARED_PTR. This checks to see if `set_tpl_resource_shared_ptr`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_tpl_resource_shared_ptr() -> Option<& 'static u8>;
   /// Like `get_tpl_resource_shared_ptr_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_tpl_resource_shared_ptr_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_tpl_resource_shared_ptr`. This assumes that TPL_RESOURCE_SHARED_PTR
    /// is set and it's initialized.
    pub(crate) unsafe fn get_tpl_resource_shared_ptr_unchecked() -> & 'static u8;
   /// An unchecked version of `get_tpl_resource_shared_ptr_mut`. This assumes that TPL_RESOURCE_SHARED_PTR
    /// is set and it's initialized.
    pub(crate) unsafe fn get_tpl_resource_shared_ptr_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of MAIN_THREAD_ID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_main_thread_id(ptr: *mut u32);
   /// Get a possible reference to MAIN_THREAD_ID. This checks to see if `set_main_thread_id`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_main_thread_id() -> Option<& 'static u32>;
   /// Like `get_main_thread_id_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_main_thread_id_mut() -> Option<& 'static mut u32>;
   /// An unchecked version of `get_main_thread_id`. This assumes that MAIN_THREAD_ID
    /// is set and it's initialized.
    pub(crate) unsafe fn get_main_thread_id_unchecked() -> & 'static u32;
   /// An unchecked version of `get_main_thread_id_mut`. This assumes that MAIN_THREAD_ID
    /// is set and it's initialized.
    pub(crate) unsafe fn get_main_thread_id_unchecked_mut() -> & 'static mut u32;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to JOB_LIST.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_job_list(ptr: *mut * mut Job);
   /// Get a possible reference to JOB_LIST. This checks to see if `set_job_list`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_job_list() -> Option<& 'static Job>;
   /// Like `get_job_list_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_job_list_mut() -> Option<& 'static mut Job>;
   /// An unchecked version of `get_job_list`. This assumes that JOB_LIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list_unchecked() -> & 'static Job;
   /// An unchecked version of `get_job_list_mut`. This assumes that JOB_LIST
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list_unchecked_mut() -> & 'static mut Job;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to JOB_LIST1.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_job_list1(ptr: *mut * mut Job);
   /// Get a possible reference to JOB_LIST1. This checks to see if `set_job_list1`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_job_list1() -> Option<& 'static Job>;
   /// Like `get_job_list1_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_job_list1_mut() -> Option<& 'static mut Job>;
   /// An unchecked version of `get_job_list1`. This assumes that JOB_LIST1
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list1_unchecked() -> & 'static Job;
   /// An unchecked version of `get_job_list1_mut`. This assumes that JOB_LIST1
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list1_unchecked_mut() -> & 'static mut Job;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to JOB_LIST2.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_job_list2(ptr: *mut * mut Job);
   /// Get a possible reference to JOB_LIST2. This checks to see if `set_job_list2`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_job_list2() -> Option<& 'static Job>;
   /// Like `get_job_list2_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_job_list2_mut() -> Option<& 'static mut Job>;
   /// An unchecked version of `get_job_list2`. This assumes that JOB_LIST2
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list2_unchecked() -> & 'static Job;
   /// An unchecked version of `get_job_list2_mut`. This assumes that JOB_LIST2
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list2_unchecked_mut() -> & 'static mut Job;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing a pointer to JOB_LIST3.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_job_list3(ptr: *mut * mut Job);
   /// Get a possible reference to JOB_LIST3. This checks to see if `set_job_list3`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_job_list3() -> Option<& 'static Job>;
   /// Like `get_job_list3_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_job_list3_mut() -> Option<& 'static mut Job>;
   /// An unchecked version of `get_job_list3`. This assumes that JOB_LIST3
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list3_unchecked() -> & 'static Job;
   /// An unchecked version of `get_job_list3_mut`. This assumes that JOB_LIST3
    /// is set and it's initialized.
    pub(crate) unsafe fn get_job_list3_unchecked_mut() -> & 'static mut Job;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of KEYCODE_FOR_MOUSE_CLICK.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_keycode_for_mouse_click(ptr: *mut u8);
   /// Get a possible reference to KEYCODE_FOR_MOUSE_CLICK. This checks to see if `set_keycode_for_mouse_click`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_keycode_for_mouse_click() -> Option<& 'static u8>;
   /// Like `get_keycode_for_mouse_click_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_keycode_for_mouse_click_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_keycode_for_mouse_click`. This assumes that KEYCODE_FOR_MOUSE_CLICK
    /// is set and it's initialized.
    pub(crate) unsafe fn get_keycode_for_mouse_click_unchecked() -> & 'static u8;
   /// An unchecked version of `get_keycode_for_mouse_click_mut`. This assumes that KEYCODE_FOR_MOUSE_CLICK
    /// is set and it's initialized.
    pub(crate) unsafe fn get_keycode_for_mouse_click_unchecked_mut() -> & 'static mut u8;

}

#[link(name = "opengfd_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of KEYBOARD_DATA.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) unsafe fn set_keyboard_data(ptr: *mut Keyboard);
   /// Get a possible reference to KEYBOARD_DATA. This checks to see if `set_keyboard_data`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) unsafe fn get_keyboard_data() -> Option<& 'static Keyboard>;
   /// Like `get_keyboard_data_mut`, but a mutable reference is created instead.
    pub(crate) unsafe fn get_keyboard_data_mut() -> Option<& 'static mut Keyboard>;
   /// An unchecked version of `get_keyboard_data`. This assumes that KEYBOARD_DATA
    /// is set and it's initialized.
    pub(crate) unsafe fn get_keyboard_data_unchecked() -> & 'static Keyboard;
   /// An unchecked version of `get_keyboard_data_mut`. This assumes that KEYBOARD_DATA
    /// is set and it's initialized.
    pub(crate) unsafe fn get_keyboard_data_unchecked_mut() -> & 'static mut Keyboard;

}

