use opengfd::{
    device::ngr::{
        allocator::Allocator,
        renderer::{
            platform::d3d::{
                ngr_142ed6270,
                ngrDX11Renderer
            },
            state::DrawState
        },
        structures::{ ListNodeFreeList, PointerListEntry }
    },
    kernel::global::Global,
    tpl::file_manager::FileManager,
};

use opengfd_proc::create_gfd_static;

// static items
create_gfd_static!(GFD_GLOBAL, Global);
create_gfd_static!(NGR_DRAW_STATE, *mut DrawState);
create_gfd_static!(NGR_ALLOCATOR, *mut Allocator);
create_gfd_static!(NGR_WINDOW, *mut ngr_142ed6270);
create_gfd_static!(NGR_DX11_RENDERER, *mut ngrDX11Renderer);
create_gfd_static!(NGR_POINTER_FREELIST, *mut ListNodeFreeList<PointerListEntry<u8>>);
create_gfd_static!(NGR_STRING_FREELIST, *mut ListNodeFreeList<u8>);
// C++ virtual tables
create_gfd_static!(NGR_CRCHASH_VTABLE, u8);
create_gfd_static!(NGR_RASTERSTATE_VTABLE, u8);
create_gfd_static!(NGR_BLENDSTATE_VTABLE, u8);
create_gfd_static!(NGR_DEPTHSTENCILSTATE_VTABLE, u8);
create_gfd_static!(NGR_SAMPLER_STATE, u8);
create_gfd_static!(NGR_MEMHINT_VTABLE, u8);
create_gfd_static!(NGR_SPINLOCK_VTABLE, u8);
create_gfd_static!(NGR_1422ECAD8_VTABLE, u8); // ?? used in free list
create_gfd_static!(NGR_FREELIST_VTABLE, u8);
// TPL
create_gfd_static!(FILE_MANAGER_INSTANCE, *mut FileManager);
create_gfd_static!(SOUND_PLAYER_SEND_SIGNAL, u8);
// std::shared_ptr vtables
create_gfd_static!(TPL_RESOURCE_SHARED_PTR, u8);
