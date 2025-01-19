#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from opengfd-globals.
use opengfd :: { device :: ngr :: { allocator :: Allocator , renderer :: { platform :: d3d :: { ngr_142ed6270 , ngrDX11Renderer } , state :: DrawState } , structures :: { ListNodeFreeList , PointerListEntry } } , kernel :: global :: Global } ;
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

