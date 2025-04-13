#![allow(dead_code, unused_imports)]
use crate::device::ngr::renderer::state::{
    BlendKey,
    BlendType,
    BlendTypeOperation,
    BorderColor,
    ComparisonFunc,
    DepthStencilDescriptions,
    DepthStencilKey,
    DepthWriteMask,
    CullMode,
    FillMode,
    FilterMode,
    FilterModeComparison,
    IATopology,
    SamplerKey,
    StencilOperation,
    RasterizerKey,
    TextureAddressMode 
};
use windows::{
    core::BOOL,
    Win32::Graphics::{
        Direct3D::D3D_PRIMITIVE_TOPOLOGY,
        Direct3D11::{
            D3D11_BLEND,
            D3D11_BLEND_DESC,
            D3D11_BLEND_OP,
            D3D11_COMPARISON_FUNC,
            D3D11_CULL_MODE,
            D3D11_DEPTH_STENCIL_DESC,
            D3D11_DEPTH_STENCILOP_DESC,
            D3D11_DEPTH_WRITE_MASK,
            D3D11_FILL_MODE,
            D3D11_FILTER,
            D3D11_RASTERIZER_DESC,
            D3D11_SAMPLER_DESC,
            D3D11_STENCIL_OP,
            D3D11_RENDER_TARGET_BLEND_DESC,
            D3D11_TEXTURE_ADDRESS_MODE
        }
    }
};

// 0x1411b1ff0
impl From<FillMode> for D3D11_FILL_MODE {
    fn from(value: FillMode) -> Self {
        match value {
            FillMode::Wireframe => windows::Win32::Graphics::Direct3D11::D3D11_FILL_WIREFRAME,
            FillMode::Solid => windows::Win32::Graphics::Direct3D11::D3D11_FILL_SOLID 
        }
    }
}

// 0x1411b2010
impl From<CullMode> for D3D11_CULL_MODE {
    fn from(value: CullMode) -> Self {
        match value {
            CullMode::None => windows::Win32::Graphics::Direct3D11::D3D11_CULL_NONE,
            CullMode::Front => windows::Win32::Graphics::Direct3D11::D3D11_CULL_FRONT,
            CullMode::Back => windows::Win32::Graphics::Direct3D11::D3D11_CULL_BACK,
        }
    }
}
// For ngrDX11Renderer::CreateRasterizerState
impl From<RasterizerKey> for D3D11_RASTERIZER_DESC {
    fn from(value: RasterizerKey) -> Self {
        Self {
            FillMode: value.field_mode.into(),
            CullMode: value.cull_mode.into(),
            FrontCounterClockwise: value.is_front_counter_clockwise.into(),
            DepthBias: value.depth_bias as i32,
            DepthBiasClamp: value.depth_bias_clamp,
            SlopeScaledDepthBias: value.slope_scaled_depth_bias,
            DepthClipEnable: value.depth_clip_enable.into(),
            ScissorEnable: value.scissor_enable.into(),
            MultisampleEnable: false.into(),
            AntialiasedLineEnable: value.antialiased_line_enable.into()
        }
    }
}
// 0x1411b1fb0
impl From<BlendType> for D3D11_BLEND {
    fn from(value: BlendType) -> Self {
        match value {
            BlendType::Zero => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_ZERO,
            BlendType::One => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_ONE,
            BlendType::SourceColor => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_SRC_COLOR,
            BlendType::InverseSourceColor => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_SRC_COLOR,
            BlendType::SourceAlpha => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_SRC_ALPHA,
            BlendType::InverseSourceAlpha => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_SRC_ALPHA,
            BlendType::DestinationAlpha => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_DEST_ALPHA,
            BlendType::InverseDestinationAlpha => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_DEST_ALPHA,
            BlendType::DestinationColor => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_DEST_COLOR,
            BlendType::InverseDestinationColor => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_INV_DEST_COLOR,
            BlendType::SourceAlphaSaturate => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_SRC_ALPHA_SAT,
        }
    }
}
// 0x1411b1fd0
impl From<BlendTypeOperation> for D3D11_BLEND_OP {
    fn from(value: BlendTypeOperation) -> Self {
        match value {
            BlendTypeOperation::Add => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_ADD,
            BlendTypeOperation::Subtract => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_SUBTRACT,
            BlendTypeOperation::ReverseSubtract => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_REV_SUBTRACT,
            BlendTypeOperation::Min => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_MIN,
            BlendTypeOperation::Max => windows::Win32::Graphics::Direct3D11::D3D11_BLEND_OP_MAX,
        }
    }
}

fn get_blank_render_target(a1: u8, a2: u32) -> D3D11_RENDER_TARGET_BLEND_DESC {
     D3D11_RENDER_TARGET_BLEND_DESC {
         BlendEnable: false.into(),
         SrcBlend: D3D11_BLEND(0),
         DestBlend: D3D11_BLEND(0),
         BlendOp: D3D11_BLEND_OP(0),
         SrcBlendAlpha: D3D11_BLEND(0),
         DestBlendAlpha: D3D11_BLEND(0),
         BlendOpAlpha: D3D11_BLEND_OP(0),
         RenderTargetWriteMask: create_render_target_write_mask(a1, a2)
     }
}

fn create_render_target_write_mask(a1: u8, a2: u32) -> u8 {
    let rdx = a2 >> (a1 * 4 & 0x1f);
    rdx as u8 & 0xf
}

// For ngrDX11Renderer::CreateBlendState
impl From<BlendKey> for D3D11_BLEND_DESC {
    fn from(value: BlendKey) -> Self {
        let ind_blend = if !value.blend_state { 
            false
        } else if (value.render_mask & !0xf) == 0 {
            false
        } else {
            true
        };
        let first_render_target = if value.enable_blending {
            D3D11_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: value.source_blend.into(),
                DestBlend: value.dest_blend.into(),
                BlendOp: value.blend_op.into(),
                SrcBlendAlpha: value.source_blend_alpha.into(),
                DestBlendAlpha: value.dest_blend_alpha.into(),
                BlendOpAlpha: value.blend_op_alpha.into(),
                RenderTargetWriteMask: create_render_target_write_mask(0, value.render_mask)
            }
        } else { get_blank_render_target(0, value.render_mask) };
        let render_targets: [D3D11_RENDER_TARGET_BLEND_DESC; 8] = [
            first_render_target,
            get_blank_render_target(1, value.render_mask),
            get_blank_render_target(2, value.render_mask),
            get_blank_render_target(3, value.render_mask),
            get_blank_render_target(4, value.render_mask),
            get_blank_render_target(5, value.render_mask),
            get_blank_render_target(6, value.render_mask),
            get_blank_render_target(7, value.render_mask),
        ];
        Self {
            AlphaToCoverageEnable: false.into(),
            IndependentBlendEnable: ind_blend.into(),
            RenderTarget: render_targets
        }
    }
}

impl From<StencilOperation> for D3D11_STENCIL_OP {
    fn from(value: StencilOperation) -> Self {
        match value {
            StencilOperation::Keep => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_KEEP,
            StencilOperation::Zero => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_ZERO,
            StencilOperation::Replace => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_REPLACE,
            StencilOperation::IncrementClamp => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_INCR_SAT,
            StencilOperation::DecrementClamp => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_DECR_SAT,
            StencilOperation::Invert => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_INVERT,
            StencilOperation::IncrementWrap => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_INCR,
            StencilOperation::DecrementWrap => windows::Win32::Graphics::Direct3D11::D3D11_STENCIL_OP_DECR,
        }
    }
}

impl From<ComparisonFunc> for D3D11_COMPARISON_FUNC {
    fn from(value: ComparisonFunc) -> Self {
        match value {
            ComparisonFunc::Never => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_NEVER,
            ComparisonFunc::Less => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_LESS,
            ComparisonFunc::Equal => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_EQUAL,
            ComparisonFunc::LessEqual => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_LESS_EQUAL,
            ComparisonFunc::Greater => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_GREATER,
            ComparisonFunc::NotEqual => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_NOT_EQUAL,
            ComparisonFunc::GreaterEqual => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_GREATER_EQUAL,
            ComparisonFunc::Always => windows::Win32::Graphics::Direct3D11::D3D11_COMPARISON_ALWAYS,
        }
    }
}

impl From<DepthStencilKey> for D3D11_DEPTH_STENCIL_DESC {
    fn from(value: DepthStencilKey) -> Self {
        Self {
            DepthEnable: value.depth_enable.into(),
            DepthWriteMask: value.depth_write_mask.into(),
            DepthFunc: value.depth_func.into(),
            StencilEnable: value.stencil_enable.into(),
            StencilReadMask: value.stencil_read_mask,
            StencilWriteMask: value.stencil_write_mask,
            FrontFace: value.front_face.into(),
            BackFace: value.back_face.into(),
        }
    }
}

impl From<DepthStencilDescriptions> for D3D11_DEPTH_STENCILOP_DESC {
    fn from(value: DepthStencilDescriptions) -> Self {
        Self {
            StencilFailOp: value.stencil_fall_op.into(),
            StencilDepthFailOp: value.stencil_depth_fall_op.into(),
            StencilPassOp: value.stencil_pass_op.into(),
            StencilFunc: value.stencil_func.into()
        }
    }
}

impl From<DepthWriteMask> for D3D11_DEPTH_WRITE_MASK {
    fn from(value: DepthWriteMask) -> Self {
        match value {
            DepthWriteMask::MaskNone => windows::Win32::Graphics::Direct3D11::D3D11_DEPTH_WRITE_MASK_ZERO,
            DepthWriteMask::MaskAll => windows::Win32::Graphics::Direct3D11::D3D11_DEPTH_WRITE_MASK_ALL,
        }
    }
}

impl From<TextureAddressMode> for D3D11_TEXTURE_ADDRESS_MODE {
    fn from(value: TextureAddressMode) -> Self {
        match value {
            TextureAddressMode::Wrap => windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => windows::Win32::Graphics::Direct3D11::D3D11_TEXTURE_ADDRESS_MIRROR_ONCE,
        }
    }
}
// 0x1422a8900
impl From<FilterMode> for D3D11_FILTER {
    fn from(value: FilterMode) -> Self {
        match value {
            FilterMode::MinMagMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_MAG_MIP_POINT,
            FilterMode::MinMagPointMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR,
            FilterMode::MinPointMagLinearMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
            FilterMode::MinPointMagMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR,
            FilterMode::MinLinearMagMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT,
            FilterMode::MinLinearMagPointMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
            FilterMode::MinMagLinearMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT,
            FilterMode::MinMagMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            FilterMode::Anisotropic => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_ANISOTROPIC,
        }
    }
}

// 0x1422a8928
impl From<FilterModeComparison> for D3D11_FILTER {
    fn from(value: FilterModeComparison) -> Self {
        match value {
            FilterModeComparison::MinMagMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT,
            FilterModeComparison::MinMagPointMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR,
            FilterModeComparison::MinPointMagLinearMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT,
            FilterModeComparison::MinPointMagMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR,
            FilterModeComparison::MinLinearMagMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT,
            FilterModeComparison::MinLinearMagPointMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
            FilterModeComparison::MinMagLinearMipPoint => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
            FilterModeComparison::MinMagMipLinear => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR,
            FilterModeComparison::Anisotropic => windows::Win32::Graphics::Direct3D11::D3D11_FILTER_COMPARISON_ANISOTROPIC,
        }
    }
}

impl From<SamplerKey> for D3D11_SAMPLER_DESC {
    // 0x141181d30
    fn from(value: SamplerKey) -> Self {
        let border_color = value.border_color.into();
        let filter = if value.comparison == ComparisonFunc::Never {
            value.filter.into()
        } else {
            let cmp_filter: FilterModeComparison = value.filter.into();
            cmp_filter.into()
        };
        Self {
            Filter: filter,
            AddressU: value.address_u.into(),
            AddressV: value.address_v.into(),
            AddressW: value.address_w.into(),
            MipLODBias: value.mip_lod_bias,
            MaxAnisotropy: value.max_anistropy,
            ComparisonFunc: value.comparison.into(),
            BorderColor: border_color,
            MinLOD: value.min_lod,
            MaxLOD: value.max_lod
        }
    }
}
// 0x1422a8840
impl From<IATopology> for D3D_PRIMITIVE_TOPOLOGY {
    fn from(value: IATopology) -> Self {
        match value {
            IATopology::Undefined => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_UNDEFINED,
            IATopology::PointList => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_POINTLIST,
            IATopology::LineList => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_LINELIST,
            IATopology::LineStrip => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_LINESTRIP,
            IATopology::TriangleList => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            IATopology::TriangleStrip => windows::Win32::Graphics::Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
        }
    }
}
