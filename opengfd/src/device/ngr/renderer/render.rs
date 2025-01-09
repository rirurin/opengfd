#![allow(unused_imports)]
use crate::{
    device::ngr::renderer::state::{
        BufferFlags,
        ComparisonFunc,
        DepthWriteMask,
        FilterMode,
        TextureAddressMode
    },
    globals
};

// see gfdDeviceRenderEffectBrushStrokeFiltering

// 0x141090af0
pub unsafe fn set_depth_stencil_key_less_equal(buf_id: usize, set_depth_stencil: bool, set_depth_write_mask: DepthWriteMask) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    let buffer = draw_state.basicBuffers.get_unchecked_mut(buf_id);
    buffer.set_depth_stencil_depth_enable(set_depth_stencil);
    buffer.set_depth_stencil_depth_func(ComparisonFunc::LessEqual);
    buffer.set_depth_stencil_depth_write_mask(set_depth_write_mask);
    buffer.set_depth_stencil_stencil_enable(false);
    let global = globals::get_gfd_global_unchecked_mut();
    if buf_id == 3 {
        *global.graphics.render_state_current.get_unchecked_mut(1) = set_depth_stencil as usize;
        *global.graphics.render_state_current.get_unchecked_mut(3) = set_depth_write_mask as usize;
    }
}
// 0x141090ce0
pub unsafe fn set_sampler_key_values(buf_id: usize, sampler_id: usize, a3: bool, a4: bool, addru: TextureAddressMode, addrv: TextureAddressMode) {
    let draw_state = globals::get_ngr_draw_state_unchecked_mut();
    let buffer = draw_state.basicBuffers.get_unchecked_mut(buf_id);
    buffer.set_sampler_mask(sampler_id);
    let filter: FilterMode = if a3 {
        if a4 == a3 {
            FilterMode::Anisotropic
        } else if !a4 {
            FilterMode::MinLinearMagMipPoint
        } else {
            FilterMode::MinPointMagMipLinear
        }
    } else if !a3 && !a4 {
        FilterMode::CmpMinMagMipPoint
    } else {
        FilterMode::MinPointMagMipLinear
    };
    buffer.set_sampler_filter(sampler_id, filter);
    buffer.set_sampler_address2d(sampler_id, addru, addrv);
}
