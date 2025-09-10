use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, SeekFrom, Write};
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
use glam::{Vec2, Vec3, Vec3A, Vec4};
use opengfd_proc::GfdRcAuto;
use std::ptr::NonNull;
use half::f16;
use crate::kernel::version::GfdVersion;
use crate::object::morph::MorphTarget;
use crate::object::object::{CastFromObject, ObjectId};
use crate::utility::misc::RGBA;
use crate::utility::name::{Name, NameSerializationContext, NameSerializationHash};
use crate::utility::stream::{DeserializationHeap, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[derive(Debug)]
pub enum GeometryError {
    InvalidTriangleIndexType(u16),
    InvalidTriangleIndexFormat(u8),
}
impl Error for GeometryError {}
impl Display for GeometryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
pub enum TriangleIndexType {
    None = 0,
    UInt16 = 1,
    UInt32 = 2
}

impl TryFrom<u16> for TriangleIndexType {
    type Error = GeometryError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TriangleIndexType::None),
            1 => Ok(TriangleIndexType::UInt16),
            2 => Ok(TriangleIndexType::UInt32),
            v => Err(GeometryError::InvalidTriangleIndexType(v))
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TriangleIndexFormat {
    OneIndexPerTriangle = 0,
    TwoIndicesPerTriangle = 1,
    OneIndexMinusOnePerTriangle = 2,
    ThreeIndicesPerTriangle = 3,
    OneIndexMinusTwoPerTriangle4 = 4,
    OneIndexMinusTwoPerTriangle5 = 5,
}

impl TryFrom<u8> for TriangleIndexFormat {
    type Error = GeometryError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OneIndexPerTriangle),
            1 => Ok(Self::TwoIndicesPerTriangle),
            2 => Ok(Self::OneIndexMinusOnePerTriangle),
            3 => Ok(Self::ThreeIndicesPerTriangle),
            4 => Ok(Self::OneIndexMinusTwoPerTriangle4),
            5 => Ok(Self::OneIndexMinusTwoPerTriangle5),
            v => Err(GeometryError::InvalidTriangleIndexFormat(v))
        }
    }
}

impl TriangleIndexFormat {
    pub fn from_vertex_count(&self, vertices: u32) -> u32 {
        match self {
            Self::OneIndexPerTriangle => vertices,
            Self::TwoIndicesPerTriangle => vertices / 2,
            Self::OneIndexMinusOnePerTriangle => vertices - 1,
            Self::ThreeIndicesPerTriangle => vertices / 3,
            Self::OneIndexMinusTwoPerTriangle4 => vertices - 2,
            Self::OneIndexMinusTwoPerTriangle5 => vertices - 2,
        }
    }

    pub fn to_vertex_count(&self, vertices: u32) -> u32 {
        match self {
            Self::OneIndexPerTriangle => vertices,
            Self::TwoIndicesPerTriangle => vertices * 2,
            Self::OneIndexMinusOnePerTriangle => vertices - 1,
            Self::ThreeIndicesPerTriangle => vertices * 3,
            Self::OneIndexMinusTwoPerTriangle4 => vertices + 2,
            Self::OneIndexMinusTwoPerTriangle5 => vertices + 2,
        }
    }
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

#[repr(C)]
#[derive(GfdRcAuto)]
// #[derive(Debug)]
pub struct Geometry<A = GfdAllocator>
where A: Allocator + Clone
{
    super_: Object,
    flags: GeometryFlags,
    type_: i32,
    lock: i32,
    prim: TriangleIndexFormat,
    index: TriangleIndexType,
    fvf: VertexAttributeFlags,
    num_vertices: i32,
    num_indices: i32,
    num_triangles: i32,
    vertex_buffer: NonNull<GeometryVertexBuffer>,
    vertex_usage: i32,
    index_buffer: Option<NonNull<GeometryIndexBuffer>>, // TriCount > 0
    skin: Option<NonNull<SkinRef>>, // Flags & Skin
    material: Option<NonNull<Material<A>>>, // Flags & Material
    morph_targets: *mut std::os::raw::c_void,
    light_container: Option<NonNull<LightContainer>>,
    vertices: [*mut std::os::raw::c_void; 2usize],
    indices: *mut std::os::raw::c_void,
    bounding_box: BoundingBox,
    bounding_sphere: BoundingSphere,
    local_obb: Option<NonNull<[Vec3A; 8usize]>>,
    cull: [CullObject; 3usize],
    resources: [GeometryCommand; 3usize],
    reflection_mat: Option<NonNull<Material<A>>>,
    outline_mat: Option<NonNull<Material<A>>>,
    ssss_mat: Option<NonNull<Material<A>>>,
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
    field1a2: u16,
    job_data: Option<NonNull<JobData>>,
    container: Option<NonNull<std::ffi::c_void>>,
    ref_: Reference,
    _allocator: A
}

impl<A> Geometry<A>
where A: Allocator + Clone
{
    pub fn vertex_sizeof_metaphor(&self) -> usize {
        let mut sizeof = if self.fvf.contains(VertexAttributeFlags::PositionXYZW) {
            size_of::<Vec4>()
        } else if self.fvf.contains(VertexAttributeFlags::PositionXYZ) {
            size_of::<Vec3>()
        } else {
            0
        };
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Normal));
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Binormal));
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Tangent));
        sizeof += size_of::<RGBA>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::DiffuseColor));
        sizeof += size_of::<[f16; 2]>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord0));
        sizeof += size_of::<[f16; 2]>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord1));
        sizeof += size_of::<[f16; 2]>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord2));
        sizeof += size_of::<RGBA>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Color2));
        sizeof
    }

    pub fn vertex_sizeof_p5r(&self) -> usize {
        let mut sizeof = if self.fvf.contains(VertexAttributeFlags::PositionXYZW) {
            size_of::<Vec4>()
        } else if self.fvf.contains(VertexAttributeFlags::PositionXYZ) {
            size_of::<Vec3>()
        } else {
            0
        };
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Normal));
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Binormal));
        sizeof += size_of::<Vec3>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Tangent));
        sizeof += size_of::<RGBA>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::DiffuseColor));
        sizeof += size_of::<Vec2>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord0));
        sizeof += size_of::<Vec2>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord1));
        sizeof += size_of::<Vec2>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::TexCoord2));
        sizeof += size_of::<RGBA>() * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Color3));
        sizeof += 0x14 * Into::<usize>::into(self.fvf.contains(VertexAttributeFlags::Flag31));
        sizeof
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Geometry<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        unsafe { this.super_.set_id(ObjectId::Geometry) };
        this.ref_ = Reference::new();
        for i in 0..3 {
            this.cull[i] = CullObject::new(1, 1, 0, 0.);
        }
        this.field42_0x18e = 1;
        this.blend_dst_color = 1;
        this.color_mask = 0xf;
        this.stencil_func = ComparisonFunc::Always;
        this.stencil_mask = 0xff;
        this.field50_0x19c = 7;
        this.field52_0x1a0 = 1;
        this.field53_0x1a1 = 1;
        this.field1a2 = 3;
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Geometry<AObject>
where AObject: Allocator + Clone
{
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        self.flags = GeometryFlags::from_bits_retain(stream.read_u32()?);
        self.fvf = VertexAttributeFlags::from_bits_retain(stream.read_u32()?);
        let (num_triangles, index_format) = match self.flags.contains(GeometryFlags::Triangles) {
            true => (stream.read_u32()?, stream.read_u16()?.try_into()?),
            false => (0, TriangleIndexType::None)
        };
        self.num_triangles = num_triangles as i32;
        self.num_vertices = stream.read_u32()? as i32;
        /*
        // TODO: replicate this behavior (why is it done like this?)
        let mut vertex_usage = match self.flags.contains(GeometryFlags::Skin) {
            true => 0x10000005u32,
            false => 4
        };
        vertex_usage += 0x10000000 * Into::<u32>::into(self.flags.contains(GeometryFlags::MorphTargets));
        vertex_usage += 0x8 * Into::<u32>::into(self.flags.contains(GeometryFlags::System));
        */
        self.prim = stream
            .has_feature(GfdVersion::EnvAddInfiniteOcean_LUTRecolorParams)
            .map_or::<Result<TriangleIndexFormat, Box<dyn Error>>, _>(
                Ok(TriangleIndexFormat::ThreeIndicesPerTriangle),
                |_| Ok(stream.read_u8()?.try_into()?)
            )?;
        self.type_ = stream
            .has_feature(GfdVersion::GeometryAddGeomType)
            .map_or::<Result<i32, Box<dyn Error>>, _>(
                Ok(0), |_| Ok(stream.read_u32()? as i32)
            )?;
        // Add vertex attributes
        let vertex_sizeof = stream.
            has_feature(GfdVersion::GFDV2).
            map_or_else(
                || self.vertex_sizeof_p5r(),
                |_| self.vertex_sizeof_metaphor()
            );
        // Add vertex weighting
        let vertex_sizeof = vertex_sizeof + match self.flags.contains(GeometryFlags::Skin) {
            true => stream
                .has_feature(GfdVersion::GeometryUseNewVertexWeightFormat)
                .map_or_else(
                    || size_of::<[u32; 0x4]>() + size_of::<[u8; 0x4]>(),
                    |_| size_of::<[f16; 0x8]>() + size_of::<[u16; 0x8]>()
                ),
            false => 0
        };
        // Get skin mask
        stream.seek(SeekFrom::Current(self.num_vertices as i64 * vertex_sizeof as i64))?;
        let weigh_mask = stream
            .has_feature(GfdVersion::GeometryAddMetaphorSkinMask)
            .map_or::<Result<u8, Box<dyn Error>>, _>(
                Ok(u8::MAX), |_| Ok(stream.read_u8()?)
            )?;
        // Read morph targets
        if self.flags.contains(GeometryFlags::MorphTargets) {
            let _ = MorphTarget::<AObject>::stream_read(stream, param);
        }
        // Read triangles
        if self.flags.contains(GeometryFlags::Triangles) {
            match self.index {
                TriangleIndexType::None => (),
                TriangleIndexType::UInt16 => { let _ = stream.seek(SeekFrom::Current(size_of::<u16>() as i64 * self.num_triangles as i64))?; },
                TriangleIndexType::UInt32 => { let _ = stream.seek(SeekFrom::Current(size_of::<u32>() as i64 * self.num_triangles as i64))?; },
            };
        }
        // Read material info
        if self.flags.contains(GeometryFlags::Material) {
            let material_name = Name::<AObject>::stream_read(stream, &mut NameSerializationContext::new(param.get_heap_allocator().unwrap(), NameSerializationHash))?.into_raw();
        }
        // Read bounding box
        if self.flags.contains(GeometryFlags::BoundingBox) {
            self.bounding_box = BoundingBox::stream_read(stream, &mut ())?.into_raw();
        }
        // Read bounding sphere
        if self.flags.contains(GeometryFlags::BoundingSphere) {
            self.bounding_sphere = BoundingSphere::stream_read(stream, &mut ())?.into_raw();
        }
        // Read hidden, cull and reflection caster flags
        /*
        if self.flags.contains(GeometryFlags::Hidden) {

        }
        if self.flags.contains(GeometryFlags::Cull) {

        }
        if self.flags.contains(GeometryFlags::ReflectionCaster) {

        }
        */
        // Read LOD data
        if self.flags.contains(GeometryFlags::Lod) {
            self.lod_start = stream.read_f32()?;
            self.lod_end = stream.read_f32()?;
        }
        Ok(())
    }
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
    pub fn get_material(&self) -> Option<&Material<A>> {
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

    pub fn get_material_mut(&mut self) -> Option<&mut Material<A>> {
        match self.flags.contains(GeometryFlags::Material) {
            true => self.material.map(|mut v| unsafe { v.as_mut() }),
            false => None
        }
    }

    pub fn get_reflection_material(&self) -> Option<&Material<A>> {
        match self.flags.contains(GeometryFlags::ReflectionCaster) {
            true => self.reflection_mat.map(|v| unsafe { v.as_ref() }),
            false => None
        }
    }
    pub fn get_outline_material(&self) -> Option<&Material<A>> {
        // TODO: Material check outline
        None
    }
    pub fn get_subsurface_material(&self) -> Option<&Material<A>> {
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

    // pub fn set_alpha_func(&mut self, ) 

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

impl<A> CastFromObject for Geometry<A>
where A: Allocator + Clone
{
    const OBJECT_ID: ObjectId = ObjectId::Geometry;
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