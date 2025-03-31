use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    device::ngr::renderer::state::{
        ComparisonFunc, StencilOperation,
        IndexBuffer, VertexBuffer
    },
    graphics::{
        cull::CullObject,
        material::Material,
        resources::ResBuffer
    },
    kernel::allocator::GfdAllocator,
    object::{
        light::LightContainer,
        object::Object
    },
    utility::{
        misc::{ BoundingBox, BoundingSphere },
        reference::{ GfdRcType, Reference }
    }
};
use glam::Vec3A;
use opengfd_proc::GfdRcAuto;
use std::ptr::NonNull;

bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct GeometryFlags: u32 {
        const Skin                  = 1 << 0;
        const Material              = 1 << 1;
        const Triangles             = 1 << 2;
        const BoundingBox           = 1 << 3;
        const BoundingSphere        = 1 << 4;
        const Hidden                = 1 << 5;
        const MorphTargets          = 1 << 6;
        const ReflectionCaster      = 1 << 7; 
        const Billboard             = 1 << 8;
        const BillboardAlignX       = 1 << 9;
        const BillboardAlignY       = 1 << 10;
        const BillboardAlignZ       = 1 << 11;
        const Lod                   = 1 << 12; // 2 floats
        const LodChild              = 1 << 13;
        const Cull                  = 1 << 14; // render flag
        const TransparencyMaskDepth = 1 << 15;
        const TransparencyMask      = 1 << 16;
        const PostTransparencyMask  = 1 << 17;
        const Flag18                    = 1 << 18;
        const Flag19                    = 1 << 19;
        const Flag20                    = 1 << 20;
        const Flag21                    = 1 << 21;
        const Flag22                    = 1 << 22;
        const Flag23                    = 1 << 23;
        const Flag24                    = 1 << 24;
        const Flag25                    = 1 << 25;
        const Flag26                    = 1 << 26;
        const Flag27                    = 1 << 27;
        const BlendState            = 1 << 28;
        const DepthState            = 1 << 29;
        const System                = 1 << 30; // r7 |= 8
        const Command               = 1 << 31;
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TriangleIndexFormat {
    None = 0,
    UInt16 = 1,
    UInt32 = 2
}

bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct VertexAttributeFlags: u32 {
        const Flag0            = 1 << 0;
        const PositionXYZ  = 1 << 1; 
        const PositionXYZW = 1 << 2;
        const Flag3            = 1 << 3;
        const Normal       = 1 << 4; 
        const Flag5            = 1 << 5;
        const DiffuseColor = 1 << 6; // Diffuse vertex color
        const Flag7            = 1 << 7;
        const TexCoord0    = 1 << 8;
        const TexCoord1    = 1 << 9;
        const TexCoord2    = 1 << 10;
        const Color2       = 1 << 11; // RGBA vertex color. used in Metaphor, comes after normals, binormals
        const Flag12           = 1 << 12;
        const Flag13           = 1 << 13;
        const Flag14           = 1 << 14;
        const Flag15           = 1 << 15;
        const Flag16           = 1 << 16;
        const Flag17           = 1 << 17;
        const Flag18           = 1 << 18;
        const Flag19           = 1 << 19;
        const Flag20           = 1 << 20;
        const Flag21           = 1 << 21;
        const Flag22           = 1 << 22;
        const Flag23           = 1 << 23;
        const Flag24           = 1 << 24;
        const Flag25           = 1 << 25;
        const Flag26           = 1 << 26;
        const Flag27           = 1 << 27;
        const Binormal     = 1 << 28;
        const Tangent      = 1 << 29;
        const Color3       = 1 << 30; // P5 beta: TerrainXYZ. RGBA vertex color. 4 bytes, after tex coord 2
        const Flag31           = 1 << 31; // P5 beta: TerrainPoint. 20 bytes, after HasBoundingBox
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GeometryPrimType {
    NoChange = 0,
    DoubleTriCount = 1,
    MinusOneTriCount = 2,
    TripleTriCount = 3,
    PlusTwoTriCount4 = 4,
    PlusTwoTriCount5 = 5
}

#[repr(C)]
#[derive(GfdRcAuto)]
// #[derive(Debug)]
pub struct Geometry<A = GfdAllocator> 
where A: Allocator + Clone
{
    _super: Object,
    flags: GeometryFlags,
    type_: i32,
    lock: i32,
    prim: GeometryPrimType,
    fvf: VertexAttributeFlags,
    num_vertices: i32,
    num_indices: i32,
    num_triangles: i32,
    vertex_buffer: NonNull<GeometryVertexBuffer>,
    vertex_usage: i32,
    index_buffer: Option<NonNull<GeometryIndexBuffer>>, // TriCount > 0
    skin: Option<NonNull<SkinRef>>, // Flags & Skin
    material: Option<NonNull<Material>>, // Flags & Material
    morph_targets: *mut ::std::os::raw::c_void,
    light_container: Option<NonNull<LightContainer>>,
    vertices: [*mut ::std::os::raw::c_void; 2usize],
    indices: *mut ::std::os::raw::c_void,
    bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    local_obb: *mut [Vec3A; 8usize],
    cull: [CullObject; 3usize],
    resources: [GeometryCommand; 3usize],
    reflection_mat: *mut Material,
    outline_mat: *mut Material,
    ssss_mat: *mut Material,
    lod_start: f32,
    lod_end: f32,
    color_mask: u32,
    stencil_enable: bool,
    stencil_fail: StencilOperation,
    stencil_depthfail: StencilOperation,
    stencil_depthpass: StencilOperation,
    stencil_func: ComparisonFunc,
    stencil_ref: u8,
    stencil_mask: u8,
    field18a: u16,
    blend_enable: bool,
    field41_0x18d: u8,
    field42_0x18e: u8,
    field43_0x18f: u8,
    blend_src_color: u16,
    blend_dst_color: u16,
    blend_src_alpha: u16,
    blend_dst_alpha: u16,
    blend_color_op: u16,
    blend_alpha_op: u16,
    field50_0x19c: u16,
    field51_0x19e: u16,
    field52_0x1a0: u8,
    field53_0x1a1: u8,
    job_data: *mut JobData,
    container: *mut ::std::os::raw::c_void,
    ref_: Reference,
    _allocator: A
}

#[repr(C)]
pub struct GeometryIndexBuffer {
    _cpp_vtable: *const u8,
    platform: NonNull<IndexBuffer>,
    vert_size: u16,
    flags: u32,
    num_indices: u32,
    ref_: Reference
}

#[repr(C)]
pub struct GeometryVertexBuffer {
    _cpp_vtable: *const u8,
    platform: NonNull<VertexBuffer>,
    data: [u8; 0x28]
}

#[repr(C)]
pub struct SkinData {
    _cpp_vtable: *const u8,
    platform: *const u8,
    flags: u32,
    count: u32,
    size: u32,
    field1c: u32
}

#[repr(C)]
#[derive(GfdRcAuto)]
pub struct SkinRef {
    data: NonNull<SkinData>,
    ty: u8,
    ref_: Reference
}

impl<A> Geometry<A>
where A: Allocator + Clone
{

    /// Original function: gfdGeometryClearFlags
    pub fn clear_flags(&mut self, flags: GeometryFlags) {
        self.flags &= flags;
    }

    pub fn get_vertex_count(&self) -> usize {
        self.num_vertices as usize
    }
    pub fn get_indice_count(&self) -> usize {
        self.num_indices as usize
    }
    pub fn get_triangle_count(&self) -> usize {
        self.num_triangles as usize
    }

    /// Original function: gfdGeometryGetColorMask
    pub fn get_color_mask(&self) -> u32 {
        self.color_mask
    }

    // pub fn get_indices(&self) -> &[u8]
    /// Original function: gfdGeoemtryGetMaterial
    pub fn get_material(&self) -> Option<&Material> {
        if self.flags.contains(GeometryFlags::Material) {
            /* 
            let res = self.material.map(|v| unsafe { v.as_ref() });
            if res.is_none() {
                // material flag should be disabled!
                self.flags &= GeometryFlags::Material;
            }
            res
            */
            self.material.map(|v| unsafe { v.as_ref() })
        } else {
            None
        }
    }
    pub fn get_reflection_material(&self) -> Option<&Material> {
        if self.flags.contains(GeometryFlags::ReflectionCaster) {
            unsafe { self.reflection_mat.as_ref() }
        } else {
            None
        }
    }
    pub fn get_outline_material(&self) -> Option<&Material> {
        // TODO: Material check outline
        None
    }
    pub fn get_subsurface_material(&self) -> Option<&Material> {
        // TODO: Material subsurface scatter check
        None
    }

    /// Original function: gfdGeometryIsVisible
    pub fn is_culled(&self) -> bool {
        self.flags.contains(GeometryFlags::Cull)
    }

    /// Original function: gfdGeometryIsVisible
    pub fn is_visible(&self) -> bool {
        !self.flags.contains(GeometryFlags::Hidden)
    }

    /// Original function: gfdGeometrySetColorMask
    pub fn set_color_mask(&mut self, mask: u32) {
        self.color_mask = mask;
    }

    /// Original function: gfdGeometrySetLOD
    pub fn set_lod(&mut self, start: f32, end: f32) {
        self.lod_start = start;
        self.lod_end = end;
        self.flags |= GeometryFlags::Lod;
    }

    /// Original function: gfdGeometrySetFlags
    pub fn set_flags(&mut self, flags: GeometryFlags) {
        self.flags |= flags;
    }

    /// Original function: gfdGeometrySetVisible
    pub fn set_hidden(&mut self, hide: bool) {
        self.flags.set(GeometryFlags::Hidden, hide)
    }

    /// Original function: gfdGeometrySetStencilFunc
    pub fn set_stencil_func(&mut self, func: ComparisonFunc, sref: u8, mask: u8) {
        self.stencil_func = func;
        self.stencil_ref = sref;
        self.stencil_mask = mask;
    }

    /// Original function: gfdGeometrySetStencilOp
    pub fn set_stencil_op(&mut self, fail: StencilOperation, 
        depth_fail: StencilOperation, depth_pass: StencilOperation) {
        self.stencil_fail = fail;
        self.stencil_depthfail = depth_fail;
        self.stencil_depthpass = depth_pass;
    }

    /// Original function: gfdGeometrySetStencilTestEnable
    pub fn set_stencil_test_enable(&mut self, enable: bool) {
        self.stencil_enable = enable;
    }
    // pub fn set_stencil_op(&mut self, fail: )
}

#[repr(C)]
pub struct GeometryCommand {
    prepare: *mut ResBuffer,
    render: *mut ResBuffer,
    shadow: *mut ResBuffer,
    reflection: *mut ResBuffer,
}

#[repr(C)]
#[derive(Debug)]
pub struct JobData;