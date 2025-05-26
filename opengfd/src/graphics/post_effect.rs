use crate::{
    graphics::texture::Texture,
    utility::mutex::Mutex as GfdMutex
};
use glam::{ Mat4, Vec2 };
use std::{
    ffi::c_void,
    ptr::NonNull
};

#[repr(C)]
pub struct PostEffectColorCorrection {
    pos: Vec2,
    size: Vec2,
    cyan: f32,
    magenta: f32,
    yellow: f32,
    dodge: f32,
    burn: f32,
    alpha: f32,
    texture: Option<NonNull<Texture>>,
}

impl Default for PostEffectColorCorrection {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            size: Vec2::ZERO,
            cyan: 0.,
            magenta: 0.,
            yellow: 0.,
            dodge: 0.,
            burn: 0.,
            alpha: 0.,
            texture: None
        }
    }
}

#[repr(C)]
pub struct PostEffectEntry {
    ty: u32,
    params: *mut c_void,
    data: *mut c_void,
    prev: Option<NonNull<PostEffectEntry>>,
    next: Option<NonNull<PostEffectEntry>>,
}

#[repr(C)]
pub struct RippleFlareParams {
    life: f32,
    radius_start: f32,
    radius_speed: f32,
    v_speed: f32,
    color: u32
}

#[repr(C)]
pub struct RippleFlareEntry {
    tm: Mat4,
    params: RippleFlareParams,
    time: f32,
    vertex: *mut c_void,
    prev: Option<NonNull<RippleFlareEntry>>,
    next: Option<NonNull<RippleFlareEntry>>,
}

#[repr(C)]
pub struct RippleFlareRender {
    entry: NonNull<RippleFlareEntry>,
    prev: Option<NonNull<RippleFlareRender>>,
    next: Option<NonNull<RippleFlareRender>>,
}

#[repr(C)]
pub struct PostEffect {
    texture_opaque_head: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    texture_opaque_tail: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    texture_semi_trans_head: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    texture_semi_trans_tail: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    polygon_head: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    polygon_tail: [Option<NonNull<PostEffectEntry>>; crate::kernel::global_common::RENDER_LISTS],
    field90: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    fielda8: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    fieldc0: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    fieldd8: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    fieldf0: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    field108: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    field120: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    field138: [Option<NonNull<u8>>; crate::kernel::global_common::RENDER_LISTS],
    ripple_flare_entry_head: Option<NonNull<RippleFlareEntry>>,
    ripple_flare_entry_tail: Option<NonNull<RippleFlareEntry>>,
    ripple_flare_render_head: [Option<NonNull<RippleFlareRender>>; crate::kernel::global_common::RENDER_LISTS],
    ripple_flare_render_tail: [Option<NonNull<RippleFlareRender>>; crate::kernel::global_common::RENDER_LISTS],
    mutex: GfdMutex
}