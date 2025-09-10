use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, Write};
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;
use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    graphics::{
        shader::{
            flag:: { 
                Flags0 as ShaderFlag0,
                Flags1 as ShaderFlag1,
                Flags2 as ShaderFlag2
            },
            shader::{ ShaderID, ShaderFlags }
        },
        texture::Texture
    },
    kernel::{
        allocator::GfdAllocator,
        graphics::{ 
            GraphicsFlags,
            GraphicsGlobal
        }
    },
    object::geometry::VertexAttributeFlags,
    utility::{
        name::Name,
        reference::{ GfdRcType, Reference }
    }
};
use glam::Mat4;
use opengfd_proc::GfdRcAuto;
#[cfg(feature = "v2-core")]
use crate::device::ngr::renderer::shader::{PixelShaderPlatform, VertexShaderPlatform};
use crate::kernel::version::GfdVersion;
use crate::object::mesh::Mesh;
use crate::utility::name::{NameSerializationContext, NameSerializationHash};
use crate::utility::stream::{DeserializationHeap, DeserializationStack, DeserializationStrategy, GfdSerializationUserData, GfdSerialize, SerializationSingleAllocator, Stream, StreamIODevice};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum MaterialError {
    UnknownMaterialType(u16),
    UnknownBlendType(u8),
    UnknownMultiplyType(u8),
    UnknownAlphaTestFunc(u16),
    UnknownCullMode(u16),
    UnknownMaterialExtension(u32)
}
impl Error for MaterialError {}
impl Display for MaterialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaterialError: {:?}", self)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Blending {
    ty: BlendType,
    src_color: u8,
    dst_color: u8,
    src_alpha: u8,
    dst_alpha: u8,
    multiple: MultiplyType,
    control: u8,
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for Blending
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: Blending = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl Blending {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.ty = stream.read_u8()?.try_into()?;
        self.src_color = stream.read_u8()?;
        self.dst_color = stream.read_u8()?;
        self.src_alpha = stream.read_u8()?;
        self.dst_alpha = stream.read_u8()?;
        self.multiple = stream.read_u8()?.try_into()?;
        // self.control = stream.read_u8()?;
        Ok(())
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlendType {
    Opaque = 0,
    Semitrans = 1,
    AddTrans = 2,
    SubTrans = 3,
    ModulateTrans = 4,
    Modulate2Trans = 5,
    Advanced = 6
}

impl TryFrom<u8> for BlendType {
    type Error = MaterialError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Opaque),
            1 => Ok(Self::Semitrans),
            2 => Ok(Self::AddTrans),
            3 => Ok(Self::SubTrans),
            4 => Ok(Self::ModulateTrans),
            5 => Ok(Self::Modulate2Trans),
            6 => Ok(Self::Advanced),
            v => Err(MaterialError::UnknownBlendType(v))
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiplyType {
    None = 0,
    Semi = 1,
    Add = 2,
    Mod = 3,
    None4 = 4,
    Sub = 5
}

impl TryFrom<u8> for MultiplyType {
    type Error = MaterialError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Semi),
            2 => Ok(Self::Add),
            3 => Ok(Self::Mod),
            4 => Ok(Self::None4),
            5 => Ok(Self::Sub),
            v => Err(MaterialError::UnknownMultiplyType(v))
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialFlags : u32 {
        const Ambient             = 1 << 0; 
        const Diffuse             = 1 << 1;   
        const Specular            = 1 << 2;   
        const Transparency        = 1 << 3;   
        const VertexColor         = 1 << 4;   
        const Fog                 = 1 << 5;   
        const Diffusivity         = 1 << 6;   
        const UVTransform         = 1 << 7;
        const Emissive            = 1 << 8;   
        const Reflection          = 1 << 9;   
        const Shadow              = 1 << 10;
        const Light               = 1 << 11;  
        const Wireframe           = 1 << 12;  
        const AlphaTest           = 1 << 13;  
        const ShadowReceiver      = 1 << 14;  
        const ShadowCaster        = 1 << 15;  
        const Extension           = 1 << 16;  
        const Outline             = 1 << 17;  
        const SpNormalAlpha       = 1 << 18;  
        const ReflectionCaster    = 1 << 19;  
        const Texture1            = 1 << 20;  
        const Texture2            = 1 << 21;  
        const Texture3            = 1 << 22;  
        const Texture4            = 1 << 23;  
        const Texture5            = 1 << 24;  
        const Texture6            = 1 << 25;  
        const Texture7            = 1 << 26;  
        const Texture8            = 1 << 27;  
        const Texture9            = 1 << 28;  
        const Texture10           = 1 << 29;  
        const SSAO                = 1 << 30;  
        const ConstantColor       = 1 << 31;  
    }
}

bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialFlags2 : u16 {
        const EnableBloom             = 1 << 0;
        const LightMapModulateMode    = 1 << 1;
        const LightMapModulate2       = 1 << 2;
        const Flag3                    = 1 << 3;
        const DisableCharacterOutline = 1 << 5; 
        const Flag6 = 1 << 6;
        const ConstantColor = 1 << 7;
        const Flag8 = 1 << 8;
        const Grayscale = 1 << 9;
        const FogDisable              = 1 << 10;
        const ShadowDisable           = 1 << 11;
        const Flag12                   = 1 << 12;
    }
}


pub mod extensions {
    use crate::graphics::material::MaterialError;

    #[repr(u32)]
    #[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
    pub enum ExtensionId {
        // P5 2014
        Toon = 0x10000,
        Edge = 0x10001,
        Outline = 0x10002,
        Water = 0x10003,
        ShadowEdge = 0x10004,
        // P5R
        Type5 = 0x10005,
        Type6 = 0x10006,
        Type7 = 0x10007,
        AlphaCrunch = 0x10008,
        // Metaphor (unused, switched to v2 format early in development)
        #[cfg(not(feature = "v1-core"))]
        Type9 = 0x10009,
        #[cfg(not(feature = "v1-core"))]
        Type10 = 0x1000a,
    }

    impl TryFrom<u32> for ExtensionId {
        type Error = MaterialError;
        fn try_from(value: u32) -> Result<Self, Self::Error> {
            match value {
                0x10000 => Ok(Self::Toon),
                0x10001 => Ok(Self::Edge),
                0x10002 => Ok(Self::Outline),
                0x10003 => Ok(Self::Water),
                0x10004 => Ok(Self::ShadowEdge),
                0x10005 => Ok(Self::Type5),
                0x10006 => Ok(Self::Type6),
                0x10007 => Ok(Self::Type7),
                0x10008 => Ok(Self::AlphaCrunch),
                #[cfg(not(feature = "v1-core"))]
                0x10009 => Ok(Self::Type9),
                #[cfg(not(feature = "v1-core"))]
                0x1000a => Ok(Self::Type10),
                v => Err(MaterialError::UnknownMaterialExtension(v))
            }
        }
    }
}

#[cfg(feature = "v2-core")]
pub mod params {
    use allocator_api2::alloc::Allocator;
    use crate::graphics::{
        material::{ Material, MaterialType },
        shader::attribute::{
            field_v2::Field,
            lambert_v2::Lambert,
            toon_v2::CharacterToon,
            type3_v2::Type3,
            distortion_v2::CharacterDistortion,
            water_v2::Water,
            layered_v2::TwoLayer,
            type7_v2::FourLayer,
            type8_v2::Type8,
            type9_v2::Type9,
            sky_v2::Sky,
            type11_v2::Type11,
            metal_v2::Metal,
            type13_v2::Type13,
            type14_v2::Type14,
            type15_v2::Type15,
            shadow_v2::Shadow
        }
    };
    use std::{
        error::Error,
        fmt::Display,
        mem::ManuallyDrop
    };
    use crate::graphics::material::MaterialError;
    use crate::kernel::allocator::GfdAllocator;

    #[repr(u16)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum MaterialId {
        Field = 0,
        Lambert = 1,
        CharacterToon = 2,
        Type3 = 3,
        CharacterDistort = 4,
        Water = 5,
        DualLayer = 6,
        Type7 = 7,
        Type8 = 8,
        Type9 = 9,
        Sky = 10,
        Type11 = 11,
        CharacterMetal = 12,
        Type13 = 13,
        Type14 = 14,
        Type15 = 15,
        Shadow = 16
    }

    impl TryFrom<u16> for MaterialId {
        type Error = MaterialError;
        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Field),
                1 => Ok(Self::Lambert),
                2 => Ok(Self::CharacterToon),
                3 => Ok(Self::Type3),
                4 => Ok(Self::CharacterDistort),
                5 => Ok(Self::Water),
                6 => Ok(Self::DualLayer),
                7 => Ok(Self::Type7),
                8 => Ok(Self::Type8),
                9 => Ok(Self::Type9),
                10 => Ok(Self::Sky),
                11 => Ok(Self::Type11),
                12 => Ok(Self::CharacterMetal),
                13 => Ok(Self::Type13),
                14 => Ok(Self::Type14),
                15 => Ok(Self::Type15),
                16 => Ok(Self::Shadow),
                v => Err(MaterialError::UnknownMaterialType(v))
            }
        }
    }

    #[allow(dead_code)]
    #[repr(C, packed(4))]
    pub union MaterialData<A = GfdAllocator>
    where A: Allocator + Clone {
        pub(super) field: ManuallyDrop<Field<A>>,
        pub(super) lambert: ManuallyDrop<Lambert<A>>,
        pub(super) chara_toon: ManuallyDrop<CharacterToon<A>>,
        pub(super) type3: ManuallyDrop<Type3<A>>,
        pub(super) chara_distort: ManuallyDrop<CharacterDistortion<A>>,
        pub(super) water: ManuallyDrop<Water<A>>,
        pub(super) dual_layer: ManuallyDrop<TwoLayer<A>>,
        pub(super) type7: ManuallyDrop<FourLayer<A>>,
        pub(super) type8: ManuallyDrop<Type8<A>>,
        pub(super) type9: ManuallyDrop<Type9<A>>,
        pub(super) sky: ManuallyDrop<Sky<A>>,
        pub(super) type11: ManuallyDrop<Type11<A>>,
        pub(super) metal: ManuallyDrop<Metal<A>>,
        pub(super) type13: ManuallyDrop<Type13<A>>,
        pub(super) type14: ManuallyDrop<Type14<A>>,
        pub(super) type15: ManuallyDrop<Type15<A>>,
        pub(super) shadow: ManuallyDrop<Shadow<A>>
    }

    #[derive(Debug)]
    pub struct MaterialIdMismatch(MaterialId, MaterialId);
    impl Error for MaterialIdMismatch {}
    impl Display for MaterialIdMismatch {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Expected material ID {:?}, got ID {:?} instead", self.0, self.1)
        }
    }

    impl<A> Material<A>
    where A: Allocator + Clone
    {
        pub fn get_specific_data<M>(&self) -> Result<&M, MaterialIdMismatch>
        where M: MaterialType {
            Err(MaterialIdMismatch(MaterialId::Field, MaterialId::Field))
        }
        pub fn get_data(&self) -> Box<&dyn MaterialType> {
            Box::new(match self.mat_type {
                MaterialId::Field => unsafe { &*(&raw const self.data as *const Field<A>) },
                MaterialId::Lambert => unsafe { &*(&raw const self.data as *const Lambert<A>) },
                MaterialId::CharacterToon => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type3 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::CharacterDistort => unsafe { &*(&raw const self.data as *const CharacterDistortion<A>) },
                MaterialId::Water => unsafe { &*(&raw const self.data as *const Water<A>) },
                MaterialId::DualLayer => unsafe { &*(&raw const self.data as *const TwoLayer<A>) },
                MaterialId::Type7 => unsafe { &*(&raw const self.data as *const FourLayer<A>) },
                MaterialId::Type8 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type9 => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Sky => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
                MaterialId::Type11 => unsafe { &*(&raw const self.data as *const Type11<A>) },
                MaterialId::CharacterMetal => unsafe { &*(&raw const self.data as *const Metal<A>) },
                MaterialId::Type13 => unsafe { &*(&raw const self.data as *const Type13<A>) },
                MaterialId::Type14 => unsafe { &*(&raw const self.data as *const Type14<A>) },
                MaterialId::Type15 => unsafe { &*(&raw const self.data as *const Type15<A>) },
                MaterialId::Shadow => unsafe { &*(&raw const self.data as *const CharacterToon<A>) },
            })
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AlphaTest {
    value: i16,
    func: AlphaTestFunc
}

#[cfg(feature = "serialize")]
impl<AStream, T> GfdSerialize<AStream, T> for AlphaTest
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
{
    fn stream_read(stream: &mut Stream<AStream, T>, _: &mut ()) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: AlphaTest = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl AlphaTest {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        self.value = stream.read_u16()? as i16;
        self.func = stream.read_u16()?.try_into()?;
        Ok(())
    }
}

#[repr(C)]
#[derive(GfdRcAuto)]
pub struct Material<A = GfdAllocator>
where A: Allocator + Clone
{
    #[cfg(feature = "v1-core")]
    lambert: crate::graphics::shader::attribute::lambert_v2::Lambert<A>,
    blend: Blending,
    culling: Culling,
    dirty: u16,
    flags: MaterialFlags,
    texture: NonNull<MaterialTexture<A>>,
    shader: ShaderID,
    #[cfg(feature = "v2-core")]
    vertex_shader: Option<NonNull<VertexShaderPlatform>>,
    #[cfg(not(feature = "v2-core"))]
    vertex_shader: usize,
    #[cfg(feature = "v2-core")]
    pixel_shader: Option<NonNull<PixelShaderPlatform>>,
    #[cfg(not(feature = "v2-core"))]
    pixel_shader: [usize; 2],
    name: Name<A>,
    alpha_test: AlphaTest,
    flags2: MaterialFlags2,
    sort_priority: i16,
    constant: i32,
    field16_0x6c: f32,
    bind_cmd: *mut std::ffi::c_void,
    unbind_cmd: *mut std::ffi::c_void,
    pixel_buffer: Option<NonNull<PixelBuffer>>,
    extension: Option<NonNull<Extension<A>>>,
    #[cfg(feature = "v2-core")]
    data: params::MaterialData<A>,
    #[cfg(feature = "v2-core")]
    mat_type: params::MaterialId,
    field23_0x2de: u16,
    ref_: Reference,
    field25_0x2e4: [u16; 3usize],
    textures: [MaterialTexture<A>; 10usize],
    _allocator: A,
}

#[repr(C)]
#[derive(GfdRcAuto)]
pub struct Extension<A = GfdAllocator>
where A: Allocator + Clone {
    head: Option<NonNull<ExtensionObject<A>>>,
    tail: Option<NonNull<ExtensionObject<A>>>,
    ref_: Reference,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Extension<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        this.stream_read_inner(stream, param)?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Extension<AObject>
where AObject: Allocator + Clone {
    // Original function: gfdExtensionStreamRead
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        let count = stream.read_u32()?;
        for i in 0..count {
            let id: extensions::ExtensionId = stream.read_u32()?.try_into()?;
            let mut context = ExtensionObjectContext::new(id, param.get_heap_allocator().unwrap());
            let mut object = unsafe { NonNull::new_unchecked(match id {
                extensions::ExtensionId::Toon => crate::graphics::shader::attribute::toon_v1::Toon::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Edge => crate::graphics::shader::attribute::edge_v1::Edge::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Outline => crate::graphics::shader::attribute::outline_v1::Outline::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Water => crate::graphics::shader::attribute::water_v1::Water::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::ShadowEdge => crate::graphics::shader::attribute::shadow_edge_v1::ShadowEdge::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Type5 => crate::graphics::shader::attribute::type5_v1::Type5::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Type6 => crate::graphics::shader::attribute::type6_v1::Type6::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::Type7 => crate::graphics::shader::attribute::type7_v1::Type7::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                extensions::ExtensionId::AlphaCrunch => crate::graphics::shader::attribute::alpha_v1::AlphaCrunch::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                #[cfg(not(feature = "v1-core"))]
                extensions::ExtensionId::Type9 => crate::graphics::shader::attribute::type9_v1::Type9::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
                #[cfg(not(feature = "v1-core"))]
                extensions::ExtensionId::Type10 => crate::graphics::shader::attribute::type10_v1::Type10::<AObject>::stream_read(stream, &mut context)?.into_raw().as_ptr() as *mut ExtensionObject<AObject>,
            })};
            match self.head {
                Some(mut head) => {
                    // add to linked list
                    unsafe { head.as_mut() }.prev = Some(object);
                    unsafe { object.as_mut() }.next = Some(head);
                    self.head = Some(object);
                },
                None => {
                    // first entry in the linked list
                    self.head = Some(object);
                    self.tail = Some(object);
                }
            }
        }
        Ok(())
    }
}

#[repr(C)]
pub struct ExtensionObject<A = GfdAllocator>
where A: Allocator + Clone {
    id: extensions::ExtensionId,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A
}

impl<A> ExtensionObject<A>
where A: Allocator + Clone {
    pub fn new(id: extensions::ExtensionId, alloc: A) -> Self {
        Self {
            id,
            prev: None,
            next: None,
            _allocator: alloc
        }
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationStack<Self>, SerializationSingleAllocator<AObject>> for ExtensionObject<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: ExtensionObject<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream, param)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> ExtensionObject<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug
    {
        self.id = stream.read_u32()?.try_into()?;
        Ok(())
    }
}

pub struct ExtensionObjectContext<A> where A: Allocator + Clone {
    _extension_id: extensions::ExtensionId,
    _allocator: A
}

impl<A> ExtensionObjectContext<A> where A: Allocator + Clone {
    pub fn new(_extension_id: extensions::ExtensionId, _allocator: A) -> Self {
        Self { _extension_id, _allocator }
    }
}


impl<A> GfdSerializationUserData<A> for ExtensionObjectContext<A>
where A: Allocator + Clone {
    fn get_heap_allocator(&self) -> Option<A> {
        Some(self._allocator.clone())
    }
}

impl<A> ExtensionObjectContext<A>
where A: Allocator + Clone {
    pub(crate) fn get_id(&self) -> extensions::ExtensionId {
        self._extension_id
    }
}

#[repr(C)]
pub struct PixelBuffer {
    data: [usize; 4]
}


#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlphaTestFunc {
    Never = 0,
    LessOrEqual0 = 1,
    Equal = 2,
    LessOrEqual1 = 3,
    GreaterOrEqual0 = 4,
    NotEqual = 5,
    GreaterOrEqual1 = 6
}

impl TryFrom<u16> for AlphaTestFunc {
    type Error = MaterialError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Never),
            1 => Ok(Self::LessOrEqual0),
            2 => Ok(Self::Equal),
            3 => Ok(Self::LessOrEqual1),
            4 => Ok(Self::GreaterOrEqual0),
            5 => Ok(Self::NotEqual),
            6 => Ok(Self::GreaterOrEqual1),
            v => Err(MaterialError::UnknownAlphaTestFunc(v))
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Culling {
    Backface = 0,
    None = 1
}

impl TryFrom<u16> for Culling {
    type Error = MaterialError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Backface),
            1 => Ok(Self::None),
            v => Err(MaterialError::UnknownCullMode(v))
        }
    }
}

#[repr(C)]
pub struct MaterialTexture<A>
where A: Allocator + Clone {
    tm: Mat4,
    texture: Option<NonNull<Texture<A>>>,
    flags: MaterialTextureFlags,
    min: u8,
    mag: u8,
    wraps: u8,
    wrapt: u8,
    _allocator: A
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationStack<Self>, SerializationSingleAllocator<AObject>> for MaterialTexture<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationStack<Self>, Box<dyn Error>> {
        let mut this: MaterialTexture<AObject> = unsafe { MaybeUninit::zeroed().assume_init() };
        this.stream_read_inner(stream, param)?;
        Ok(this.into())
    }
}

#[cfg(feature = "serialize")]
impl<AObject> MaterialTexture<AObject>
where AObject: Allocator + Clone {
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<(), Box<dyn Error>>
    where
        T: Debug + Read + Write + Seek + StreamIODevice,
        AStream: Allocator + Clone + Debug
    {
        let name = Name::<AObject>::stream_read(stream, &mut NameSerializationContext::new(param.get_heap_allocator().unwrap().clone(), NameSerializationHash))?.into_raw();
        self.flags = MaterialTextureFlags::from_bits_truncate(stream.read_u32()?);
        self.min = stream.read_u8()?;
        self.mag = stream.read_u8()?;
        self.wraps = stream.read_u8()?;
        self.wrapt = stream.read_u8()?;
        self.tm = Mat4::stream_read(stream, &mut ())?.into_raw();
        Ok(())
    }
}

bitflags! { 
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MaterialTextureFlags : u32 {
        const HasUVTransform  = 1 << 0;
        const Flag1  = 1 << 1;
        const Flag2  = 1 << 2;
        const Flag3  = 1 << 3;
        const Flag4  = 1 << 4;
        const Flag5  = 1 << 5;
        const Flag6  = 1 << 6;
        const Flag7  = 1 << 7;
        const Flag8  = 1 << 8;
        const Flag9  = 1 << 9;
        const Flag10 = 1 << 10;
        const Flag11 = 1 << 11;
        const Flag12 = 1 << 12;
        const Flag13 = 1 << 13;
        const Flag14 = 1 << 14;
        const Flag15 = 1 << 15;
        const Flag16 = 1 << 16;
        const Flag17 = 1 << 17;
        const Flag18 = 1 << 18;
        const Flag19 = 1 << 19;
        const Flag20 = 1 << 20;
        const Flag21 = 1 << 21;
        const Flag22 = 1 << 22;
        const Flag23 = 1 << 23;
        const Flag24 = 1 << 24;
        const Flag25 = 1 << 25;
        const Flag26 = 1 << 26;
        const Flag27 = 1 << 27;
        const Flag28 = 1 << 28;
        const Flag29 = 1 << 29;
        const Flag30 = 1 << 30;
        const Flag31 = 1 << 31;
    }
}

impl<A> Material<A>
where A: Allocator + Clone
{
    /// Original function: gfdMaterialCheckAlphaTest
    pub fn check_alpha_test(&self) -> bool {
        self.flags.contains(MaterialFlags::AlphaTest)
    }
    /// Original function: gfdMaterialClearDirtyBit
    pub fn clear_dirty_bit(&mut self, dirty: u16) {
        self.dirty &= dirty;
    }
    /// Original function: gfdMaterialCheckShadowCaster
    pub fn check_shadow_caster(&self) -> bool {
        let glb = GraphicsGlobal::get_gfd_graphics_global();
        self.flags.contains(MaterialFlags::ShadowCaster)
        && glb.has_flags(GraphicsFlags::ShadowCaster)

    }

    pub fn has_flags(&self, flag: MaterialFlags) -> bool {
        self.flags.contains(flag)
    }
    pub fn has_flags2(&self, flag: MaterialFlags2) -> bool {
        self.flags2.contains(flag)
    }
    pub fn has_any_flag(&self, flag: MaterialFlags) -> bool {
        self.flags.intersects(flag)
    }
    pub fn has_any_flag2(&self, flag: MaterialFlags2) -> bool {
        self.flags2.intersects(flag)
    }

    /// Original function: gfdMaterialClearFlagBit
    pub fn clear_flag_bit(&mut self, flags: MaterialFlags) {
        self.flags &= !flags;
    }
    /// Original function: gfdMaterialGetFlag
    pub fn get_flag(&self) -> MaterialFlags {
        self.flags
    }
    /// Original function: gfdMaterialGetFlag2
    pub fn get_flag2(&self) -> MaterialFlags2 {
        self.flags2
    }
    pub fn get_mat_data_offset() -> usize {
        std::mem::offset_of!(Self, data)
    }
    /// Original function: gfdMaterialSetFlagBit
    pub fn set_flag(&mut self, flag: MaterialFlags, value: bool) {
        self.flags.set(flag, value)
    }
    pub fn set_flag2(&mut self, flag: MaterialFlags2, value: bool) {
        self.flags2.set(flag, value)
    }
    #[inline(always)]
    pub fn flag_on(&mut self, flag: MaterialFlags) {
        self.set_flag(flag, true)
    }
    #[inline(always)]
    pub fn flag2_on(&mut self, flag: MaterialFlags2) {
        self.set_flag2(flag, true)
    }
    #[inline(always)]
    pub fn flag_off(&mut self, flag: MaterialFlags) {
        self.set_flag(flag, false)
    }
    #[inline(always)]
    pub fn flag2_off(&mut self, flag: MaterialFlags2) {
        self.set_flag2(flag, false)
    }
    /// Original function: gfdMaterialSetFlag
    pub fn set_all_flags(&mut self, flag: MaterialFlags) {
        self.flags = flag;
    }
    /// Original function: gfdMaterialSetFlag2
    pub fn set_all_flags2(&mut self, flag: MaterialFlags2) {
        self.flags2 = flag;
    }
}
pub trait MaterialType {
    fn check_billboard_shadow_map(&self) -> bool;
    fn check_inside_14110ba40(&self) -> bool;
    /// Original function: gfdMaterialCheckInvisible
    fn check_invisible(&self) -> bool;
    /// Original function: gfdMaterialCheckOutline
    fn check_outline(&self) -> bool;
    // For gfdGeometryGetRenderPriority
    fn check_render_prio_mod(&self) -> bool;
    fn check_subsurface_scatter(&self) -> bool;
    fn check_toon_flag_8000(&self) -> bool;
    /// Original function: gfdMaterialCheckTranslucently
    fn check_translucency(&self) -> bool;
    fn check_transparent_14107980(&self) -> bool;
    fn get_base_color_opacity(&self) -> f32;
    // Inside of gfdGeometryRender, after gfdMaterialCheckTranslucently
    fn get_shadow_link_func(&self) -> u8;

    fn get_tex1_name(&self) -> &'static str { "Texture 1" } // register(t0)
    fn get_tex2_name(&self) -> &'static str { "Texture 2" } // register(t1)
    fn get_tex3_name(&self) -> &'static str { "Texture 3" } // register(t2)
    fn get_tex4_name(&self) -> &'static str { "Texture 4" } // register(t3)
    fn get_tex5_name(&self) -> &'static str { "Texture 5" } // register(t4)
    fn get_tex6_name(&self) -> &'static str { "Texture 6" } // register(t5)
    fn get_tex7_name(&self) -> &'static str { "Texture 7" } // register(t6)
    fn get_tex8_name(&self) -> &'static str { "Texture 8" } // register(t7)
    fn get_tex9_name(&self) -> &'static str { "Texture 9" } // register(t8)
    fn get_tex10_name(&self) -> &'static str { "Texture 10" } // register(t9)

    // See gfdMaterialGetShaderFlags
    fn set_shader_flags(&self, vtx: VertexAttributeFlags, flags: &mut ShaderFlags);

    // fn set_ambient_color(&self, )
    // diffuse, emissive, specular, diffusivity, reflectivity
    /// Original function: gfdMaterialUpdate
    fn update(&mut self);
    // Material->MapType also called from 
    // - inside gfdThJobGeometryUpdate
    // TODO: StreamRead/StreamWrite, create material

}

pub(crate) const TEX_BIT_SIZE: usize = 3;
pub(crate) const TEX_UV_ID_MAX: usize = 2;
pub(crate) const TEX_BIT_MAX: usize = (1 << TEX_BIT_SIZE) - 1;

impl<A> Material<A>
where A: Allocator + Clone
{
    pub fn set_texture_map_flags(&self, tex_id: usize, flags: &mut ShaderFlags) {
        // const Texture1 = 1 << 20;
        let texflag = MaterialFlags::from_bits_retain(1 << (0x14 + tex_id));
        let texin = self.shader.get_texcoord_in().get_slot(tex_id).unwrap();
        let texout = self.shader.get_texcoord_in().get_slot(tex_id).unwrap();
        if self.flags.contains(texflag) && texin <= TEX_UV_ID_MAX as u32 {
            // const FLAG1_TEXTURE1 = 1 << 0x16;
            *flags |= ShaderFlag1::from_bits_retain(1 << (0x16 + tex_id));
            // const FLAG0_TEXCOORD0IN = 1 << 0x10 and const FLAG0_TEXCOORD0OUT = 1 << 0x13;
            *flags |= ShaderFlag0::from_bits_retain(1 << (0x10 + texin) | 1 << (0x13 + texout));
            if self.flags.contains(MaterialFlags::UVTransform) &&
            self.textures[tex_id].flags.contains(MaterialTextureFlags::HasUVTransform) {
                // const FLAG1_MATERIAL_UV0TRANSFORM = 1 << 0xc;
                *flags |= ShaderFlag1::from_bits_retain(1 << (0xc + texout));
                
            }
        }
    }

    // Original function: 0x141071d70 (Metaphor Prologue Demo)
    pub fn get_shader_flags(&self, map_id: u16, vtx: VertexAttributeFlags, flags: &mut ShaderFlags) {
        let param = self.get_data();
        let glb = GraphicsGlobal::get_gfd_graphics_global();
        flags.reset_flag0(ShaderFlag0::FLAG0_ALWAYS_ENABLED);
        flags.reset_flag1(ShaderFlag1::FLAG1_MATERIAL_AMBDIFF);
        flags.reset_flag2(ShaderFlag2::empty());
        // Light info
        if self.flags.contains(MaterialFlags::Light) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_LIGHT;
            if self.flags.contains(MaterialFlags::Texture9) { // light map
                if !self.flags2.contains(MaterialFlags2::LightMapModulateMode) {
                    if self.flags2.contains(MaterialFlags2::LightMapModulate2) {
                        *flags |= ShaderFlag1::FLAG1_LIGHTMAP_MODULATE2;
                    }
                } else {
                    *flags |= ShaderFlag2::FLAG2_LIGHTMAP_MODULATE;
                }
            }
        }
        if self.flags.contains(MaterialFlags::AlphaTest) {
            *flags |= match self.alpha_test.func {
                AlphaTestFunc::Never => ShaderFlag2::FLAG2_ATEST_NEVER,
                AlphaTestFunc::LessOrEqual0 |
                AlphaTestFunc::LessOrEqual1 => ShaderFlag2::FLAG2_ATEST_LESS_LEQUAL,
                AlphaTestFunc::Equal => ShaderFlag2::FLAG2_ATEST_EQUAL,
                AlphaTestFunc::NotEqual => ShaderFlag2::FLAG2_ATEST_NOTEQUAL,
                AlphaTestFunc::GreaterOrEqual0 |
                AlphaTestFunc::GreaterOrEqual1 => ShaderFlag2::FLAG2_ATEST_GREATER_GEQUAL,
            };
        }
        if param.check_translucency() {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_TRANSPARENCY;
        } else if self.flags.contains(MaterialFlags::Diffusivity) {
            *flags |= ShaderFlag0::FLAG0_TEMPERARE;
        }
        if self.flags.contains(MaterialFlags::VertexColor) 
        && vtx.contains(VertexAttributeFlags::DiffuseColor) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_VERTEXCOLOR;
        }
        if self.flags.contains(MaterialFlags::Fog) {
            if glb.has_flags(GraphicsFlags::Fog) {
                *flags |= ShaderFlag1::FLAG1_MATERIAL_FOG;
            }
            if glb.has_flags(GraphicsFlags::HeightFog) {
                *flags |= ShaderFlag1::FLAG1_MATERIAL_HEIGHTFOG;
            }
            if self.blend.ty == BlendType::AddTrans
            || self.blend.ty == BlendType::Modulate2Trans {
                *flags |= ShaderFlag2::FLAG2_MATERIAL_MODULATE_FOG;
            }
        }
        if self.flags.contains(MaterialFlags::SSAO) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_OCCLUSION;
        }
        if self.flags.contains(MaterialFlags::Emissive) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_EMISSIVE;
        }
        if self.flags.contains(MaterialFlags::ShadowReceiver) 
        && glb.has_flags(GraphicsFlags::ShadowCaster) {
            *flags |= ShaderFlag1::FLAG1_MATERIAL_SHADOW;
        }
        if self.flags.contains(MaterialFlags::Texture5) {
            match self.blend.multiple {
                MultiplyType::Semi => *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_SEMI,
                MultiplyType::Add => *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_ADD,
                MultiplyType::Mod => *flags |= ShaderFlag1::FLAG1_MATERIAL_MULTIPLE_MOD,
                MultiplyType::Sub => *flags |= ShaderFlag2::FLAG2_MATERIAL_MULTIPLE_SUB,
                _ => {}
            }
        }
        for i in 0..10 {
            self.set_texture_map_flags(i, flags);
        }
        if self.flags.contains(MaterialFlags::SpNormalAlpha) {
            *flags |= ShaderFlag2::FLAG2_SPECULAR_NORMALMAPALPHA;
        }
        if map_id == 4 {
            *flags |= ShaderFlag0::FLAG0_INFINITE;
        }
        param.set_shader_flags(vtx, flags);
        if self.flags.contains(MaterialFlags::ReflectionCaster) {
            *flags |= ShaderFlag2::FLAG2_REFLECTION_CASTER;
        }
        if self.flags2.contains(MaterialFlags2::ConstantColor) {
            *flags |= ShaderFlag0::FLAG0_CONSTANTCOLOR;
        }
        if self.flags2.contains(MaterialFlags2::Grayscale) {
            *flags |= ShaderFlag0::FLAG0_CONSTANTCOLOR | ShaderFlag0::FLAG0_GRAYSCALE;
        }
    }
}

#[cfg(feature = "serialize")]
impl<AStream, AObject, T> GfdSerialize<AStream, T, AObject, DeserializationHeap<Self, AObject>, SerializationSingleAllocator<AObject>> for Material<AObject>
where T: Debug + Read + Write + Seek + StreamIODevice,
      AStream: Allocator + Clone + Debug,
      AObject: Allocator + Clone
{
    fn stream_read(stream: &mut Stream<AStream, T>, param: &mut SerializationSingleAllocator<AObject>) -> Result<DeserializationHeap<Self, AObject>, Box<dyn Error>> {
        let mut this = DeserializationHeap::<Self, AObject>::zeroed(param);
        // Original function: gfdMaterialInitialize (0x14106bf90, Steam Prologue Demo 1.01)
        this.flags = MaterialFlags::Ambient | MaterialFlags::Diffuse;
        this.alpha_test.func = AlphaTestFunc::GreaterOrEqual0;
        this.blend.src_color = 1;
        this.texture = unsafe { NonNull::new_unchecked((&mut this.textures).as_mut_ptr()) };
        this.blend.src_alpha = 1;
        this.blend.multiple = MultiplyType::Semi;
        this.constant = -1;
        this.field16_0x6c = 1.;
        this.ref_ = Reference::new();
        this.stream_read_inner(stream, param.get_heap_allocator().unwrap())?;
        Ok(this)
    }
}

#[cfg(feature = "serialize")]
impl<AObject> Material<AObject>
where AObject: Allocator + Clone {
    // Original function: gfdMaterialStreamRead (0x14106e620 Steam Prologue Demo 1.01)
    fn stream_read_inner<AStream, T>(&mut self, stream: &mut Stream<AStream, T>, alloc: AObject) -> Result<(), Box<dyn Error>>
    where T: Debug + Read + Write + Seek + StreamIODevice,
          AStream: Allocator + Clone + Debug {
        self.mat_type = stream
            .has_feature(GfdVersion::GFDV2)
            .map_or::<Result<params::MaterialId, Box<dyn Error>>, _>(
                Ok(params::MaterialId::Lambert),
                |_| Ok(stream.read_u16()?.try_into()?)
            )?;
        self.name = Name::<AObject>::stream_read(stream, &mut NameSerializationContext::new(alloc.clone(), NameSerializationHash))?.into_raw();
        self.flags = MaterialFlags::from_bits_truncate(stream.read_u32()?);
        println!("{}: {:?}, {:?}", self.name, self.mat_type, self.flags);
        if stream.has_feature(GfdVersion::MaterialDiffusivitySSAONotRequired).is_none() {
            self.flags |= (MaterialFlags::Diffusivity | MaterialFlags::SSAO);
        }
        if stream.has_feature(GfdVersion::MaterialAllowUVTransform).is_none() {
            self.flags.remove(MaterialFlags::UVTransform);
        }
        #[cfg(feature = "v2-core")]
        {
            match self.mat_type {
                params::MaterialId::Field => self.data.field = ManuallyDrop::new(crate::graphics::shader::attribute::field_v2::Field::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Lambert => self.data.lambert = ManuallyDrop::new(crate::graphics::shader::attribute::lambert_v2::Lambert::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::CharacterToon => self.data.chara_toon = ManuallyDrop::new(crate::graphics::shader::attribute::toon_v2::CharacterToon::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type3 => self.data.type3 = ManuallyDrop::new(crate::graphics::shader::attribute::type3_v2::Type3::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::CharacterDistort => self.data.chara_distort = ManuallyDrop::new(crate::graphics::shader::attribute::distortion_v2::CharacterDistortion::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Water => self.data.water = ManuallyDrop::new(crate::graphics::shader::attribute::water_v2::Water::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::DualLayer => self.data.dual_layer = ManuallyDrop::new(crate::graphics::shader::attribute::layered_v2::TwoLayer::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type7 => self.data.type7 = ManuallyDrop::new(crate::graphics::shader::attribute::type7_v2::FourLayer::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type8 => self.data.type8 = ManuallyDrop::new(crate::graphics::shader::attribute::type8_v2::Type8::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type9 => self.data.type9 = ManuallyDrop::new(crate::graphics::shader::attribute::type9_v2::Type9::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Sky => self.data.sky = ManuallyDrop::new(crate::graphics::shader::attribute::sky_v2::Sky::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type11 => self.data.type11 = ManuallyDrop::new(crate::graphics::shader::attribute::type11_v2::Type11::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::CharacterMetal => self.data.metal = ManuallyDrop::new(crate::graphics::shader::attribute::metal_v2::Metal::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type13 => self.data.type13 = ManuallyDrop::new(crate::graphics::shader::attribute::type13_v2::Type13::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type14 => self.data.type14 = ManuallyDrop::new(crate::graphics::shader::attribute::type14_v2::Type14::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Type15 => self.data.type15 = ManuallyDrop::new(crate::graphics::shader::attribute::type15_v2::Type15::<AObject>::stream_read(stream, &mut ())?.into_raw()),
                params::MaterialId::Shadow => self.data.shadow = ManuallyDrop::new(crate::graphics::shader::attribute::shadow_v2::Shadow::<AObject>::stream_read(stream, &mut ())?.into_raw()),
            }
        }
        #[cfg(feature = "v1-core")]
        {
            self.lambert = crate::graphics::shader::attribute::field_v2::Field::<AObject>::stream_read(stream, &mut ())?.into_raw();
        }
        self.blend = Blending::stream_read(stream, &mut ())?.into_raw();
        self.alpha_test = AlphaTest::stream_read(stream, &mut ())?.into_raw();
        self.flags2 = stream
            .has_feature(GfdVersion::MaterialAddSecondFlags)
            .map_or::<Result<MaterialFlags2, Box<dyn Error>>, _>(
                Ok(MaterialFlags2::EnableBloom),
                |_| Ok(MaterialFlags2::from_bits_truncate(stream.read_u16()?))
            )?;
        self.sort_priority = (match stream.has_feature(GfdVersion::MaterialAddSecondFlags) {
            Some(_) => stream.read_u16()? as i16,
            None => stream.read_u32()? as i16
        }).min(16);
        self.shader = ShaderID::stream_read(stream, &mut ())?.into_raw();
        self.culling = stream.read_u16()?.try_into()?;
        self.constant = stream
            .has_feature(GfdVersion::MaterialFlagsAllowConstantColor)
            .map_or::<Result<i32, Box<dyn Error>>, _>(
                Ok(0),
                |_| Ok(stream.read_u32()? as i32)
            )?;
        self.field16_0x6c = stream
            .has_feature(GfdVersion::MaterialAddField6C)
            .map_or::<Result<f32, Box<dyn Error>>, _>(
                Ok(0.),
                |_| Ok(stream.read_f32()?)
            )?;
        for i in 0..10 {
            if self.flags.contains(MaterialFlags::from_bits_truncate(1 << (20 + i))) {
                let tex_info: MaterialTexture<AObject> = MaterialTexture::stream_read(stream, &mut SerializationSingleAllocator::new(alloc.clone()))?.into_raw();
            }
        }
        // GFD extensions (v1 only)
        #[cfg(feature = "v1-core")]
        {

        }
        // println!("{:?}", unsafe { &self.data.chara_toon });
        // println!("{:?}", self.blend);
        // println!("{:?}, {:?}, {:?}", self.alpha_test, self.flags2, self.shader);
        Ok(())
    }
}