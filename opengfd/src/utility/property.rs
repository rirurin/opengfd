use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    kernel::allocator::GfdAllocator,
    utility::{
        mutex::Mutex,
        name::Name,
        reference::Reference
    }
};
use glam::{ U8Vec3, U8Vec4, Vec3A, Vec4 };
use std::{
    error::Error,
    fmt::Display,
    ptr::NonNull
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValueType {
    #[default]
    Invalid = 0,
    Int = 1,
    Float = 2,
    Bool = 3,
    String = 4,
    ByteVector3 = 5,
    ByteVector4 = 6,
    Vector3 = 7,
    Vector4 = 8,
    ByteArray = 9,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropertyAccess {
    Synchronous = 0,
    RDWR = 1,
}

bitflags! {
    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PropertyFlags : u16 {
        const USE_MUTEX = 1 << 0;
    }
}

#[repr(C)]
pub struct Property<A = GfdAllocator> 
where A: Allocator + Clone
{
    flags: PropertyFlags,
    access: PropertyAccess,
    field04: u32,
    head: Option<NonNull<PropertyChunk<A>>>,
    tail: Option<NonNull<PropertyChunk<A>>>,
    mutex: Mutex,
    field1c: u32,
    ref_: Reference,
    _allocator: A
}

impl<A> Property<A> 
where A: Allocator + Clone
{
    pub fn first(&self) -> Option<&PropertyChunk<A>> {
        self.head.map(|v| unsafe { v.as_ref() })
    }
    pub fn last(&self) -> Option<&PropertyChunk<A>> {
        self.tail.map(|v| unsafe { v.as_ref() })
    }
    pub fn first_mut(&mut self) -> Option<&mut PropertyChunk<A>> {
        self.head.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn last_mut(&self) -> Option<&mut PropertyChunk<A>> {
        self.tail.map(|mut v| unsafe { v.as_mut() })
    }

    /// Original function: gfdPropertyGetAccessMode
    pub fn get_access_mode(&self) -> PropertyAccess {
        self.access
    }
    /// Get a reference to the first property chunk with a matching name
    /// Original function: gfdPropertyFind
    pub fn find(&self, name: &str) -> Option<&PropertyChunk<A>> {
        self.into_iter().find(|p| p.name.get_string() == Some(name))
    }
    /// Get a mutable reference to the first property chunk with a matching name
    /// Original function: gfdPropertyFind
    pub fn find_mut(&mut self, name: &str) -> Option<&mut PropertyChunk<A>> {
        self.into_iter().find(|p| p.name.get_string() == Some(name))
    }
    /// Count the number of property chunks inside of this property node
    /// Original function: gfdPropertyGetCount
    pub fn len(&self) -> usize {
        self.into_iter().count()
    }

    pub fn get_property_entries(&self) -> Vec<&PropertyChunk<A>> {
        self.into_iter().collect()
    }
}

pub struct PropertyIterator<'a, A> 
where A: Allocator + Clone
{
    curr: Option<&'a PropertyChunk<A>>,
    curr_rev: Option<&'a PropertyChunk<A>>
}

impl<'a, A> PropertyIterator<'a, A> 
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const PropertyChunk<A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const PropertyChunk<A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, A> Iterator for PropertyIterator<'a, A> 
where A: Allocator + Clone
{
    type Item = &'a PropertyChunk<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            self.curr = match self.collided() {
                false => v.get_next(),
                true => None
            };
            v
        })
    }
}

impl<'a, A> DoubleEndedIterator for PropertyIterator<'a, A> 
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            self.curr = match self.collided() {
                false => v.get_prev(),
                true => None
            };
            v
        })
    }
}

impl<'a, A> IntoIterator for &'a Property<A> 
where A: Allocator + Clone
{
    type Item = &'a PropertyChunk<A>;
    type IntoIter = PropertyIterator<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            curr: self.first(),
            curr_rev: self.last()
        }
    }
}

pub struct PropertyIteratorMut<'a, A> 
where A: Allocator + Clone
{
    curr: Option<&'a mut PropertyChunk<A>>,
    curr_rev: Option<&'a mut PropertyChunk<A>>
}

impl<'a, A> PropertyIteratorMut<'a, A> 
where A: Allocator + Clone
{
    fn collided(&self) -> bool {
        let fwd_ptr = match &self.curr { Some(v) => &raw const **v, None => std::ptr::null() as *const PropertyChunk<A> };
        let bck_ptr = match &self.curr_rev { Some(v) => &raw const **v, None => std::ptr::null() as *const PropertyChunk<A> };
        std::ptr::eq(fwd_ptr, bck_ptr)
    }
}

impl<'a, A> Iterator for PropertyIteratorMut<'a, A> 
where A: Allocator + Clone
{
    type Item = &'a mut PropertyChunk<A>;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|v| {
            let value = unsafe { &mut *(&raw const *v as *mut PropertyChunk<A>) };
            self.curr = match self.collided() {
                false => v.get_next_mut(),
                true => None
            };
            value
        })
    }
}

impl<'a, A> DoubleEndedIterator for PropertyIteratorMut<'a, A> 
where A: Allocator + Clone
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.curr_rev.take().map(|v| {
            let value = unsafe { &mut *(&raw const *v as *mut PropertyChunk<A>) };
            self.curr = match self.collided() {
                false => v.get_prev_mut(),
                true => None
            };
            value
        })
    }
}

impl<'a, A> IntoIterator for &'a mut Property<A> 
where A: Allocator + Clone
{
    type Item = &'a mut PropertyChunk<A>;
    type IntoIter = PropertyIteratorMut<'a, A>;
    fn into_iter(self) -> Self::IntoIter {
        let curr = self.first().map(|v| unsafe { &mut *(&raw const *v as *mut PropertyChunk<A>)});
        let curr_rev = self.last_mut();
        Self::IntoIter { curr, curr_rev }
    }
}

#[derive(Debug)]
pub struct PropertyChunkTypeError(ValueType, ValueType);
impl Error for PropertyChunkTypeError {}
impl Display for PropertyChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected type {:?}, got type {:?} instead", self.0, self.1)
    }
}
impl PropertyChunkTypeError {
    pub fn new(expected: ValueType, actual: ValueType) -> Self {
        Self(expected, actual)
    }
}

const PROPERTY_MAXIMUM_SIZE: usize = 16;

#[repr(C)]
pub struct PropertyChunk<A = GfdAllocator> 
where A: Allocator + Clone
{
    ty: ValueType,
    field04: u32,
    name: Name,
    data: [u8; PROPERTY_MAXIMUM_SIZE],
    size: i32,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    _allocator: A
}

impl<A> PropertyChunk<A> 
where A: Allocator + Clone
{
    pub fn get_next(&self) -> Option<&Self> {
        self.next.map(|v| unsafe { v.as_ref() } )
    }
    pub fn get_prev(&self) -> Option<&Self> {
        self.prev.map(|v| unsafe { v.as_ref() } )
    }
    pub fn get_next_mut(&mut self) -> Option<&mut Self> {
        self.next.map(|mut v| unsafe { v.as_mut() } )
    }
    pub fn get_prev_mut(&mut self) -> Option<&mut Self> {
        self.prev.map(|mut v| unsafe { v.as_mut() } )
    }
    pub fn get_name(&self) -> Option<&str> {
        self.name.get_string()
    }
    pub fn get_property_type(&self) -> ValueType {
        self.ty
    }
    pub fn get_integer_value(&self) -> Result<i32, PropertyChunkTypeError> {
        match self.ty {
            ValueType::Int => Ok(unsafe { *(self.data.as_ptr() as *const i32) }),
            _ => Err(PropertyChunkTypeError::new(ValueType::Int, self.ty))
        }
    }
    pub fn get_float_value(&self) -> Result<f32, PropertyChunkTypeError> {
        match self.ty {
            ValueType::Float => Ok(unsafe { *(self.data.as_ptr() as *const f32) }),
            _ => Err(PropertyChunkTypeError::new(ValueType::Float, self.ty))
        }
    }
    pub fn get_bool_value(&self) -> Result<bool, PropertyChunkTypeError> {
        match self.ty {
            ValueType::Bool => Ok(unsafe { *(self.data.as_ptr() as *const bool) }),
            _ => Err(PropertyChunkTypeError::new(ValueType::Bool, self.ty))
        }
    }
    pub fn get_string_value(&self) -> Result<&str, PropertyChunkTypeError> {
        match self.ty {
            ValueType::String => Ok(unsafe { std::str::from_utf8_unchecked(
                std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.size as usize)) }),
            _ => Err(PropertyChunkTypeError::new(ValueType::String, self.ty))
        }
    }
    pub fn get_byte_vec3_value(&self) -> Result<U8Vec3, PropertyChunkTypeError> {
        match self.ty {
            ValueType::ByteVector3 => Ok(U8Vec3::from_slice(unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, 3 )})),
            _ => Err(PropertyChunkTypeError::new(ValueType::ByteVector3, self.ty))
        }
    }
    pub fn get_byte_vec4_value(&self) -> Result<U8Vec4, PropertyChunkTypeError> {
        match self.ty {
            ValueType::ByteVector4 => Ok(U8Vec4::from_slice(unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, 4 )})),
            _ => Err(PropertyChunkTypeError::new(ValueType::ByteVector4, self.ty))
        }
    }
    pub fn get_vec3_value(&self) -> Result<Vec3A, PropertyChunkTypeError> {
        match self.ty {
            ValueType::Vector3 => Ok(Vec3A::from_slice(unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const f32, 3 )})),
            _ => Err(PropertyChunkTypeError::new(ValueType::Vector3, self.ty))
        }
    }
    pub fn get_vec4_value(&self) -> Result<Vec4, PropertyChunkTypeError> {
        match self.ty {
            ValueType::Vector4 => Ok(Vec4::from_slice(unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const f32, 4 )})),
            _ => Err(PropertyChunkTypeError::new(ValueType::Vector4, self.ty))
        }
    }
    pub fn get_byte_array_value(&self) -> Result<&[u8], PropertyChunkTypeError> {
        match self.ty {
            ValueType::ByteArray => Ok(unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.size as usize)}),
            _ => Err(PropertyChunkTypeError::new(ValueType::ByteArray, self.ty))
        }
    }
}