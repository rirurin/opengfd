use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    device::ngr::{
        allocator::AllocatorHook,
        renderer::platform::d3d::TextureResource
    },
    kernel::{
        allocator::GfdAllocator,
        file::FileFlags,
        graphics::GraphicsGlobal,
    },
    utility::{
        name::Name,
        reference::{ GfdRc, GfdRcType, Reference },
    }
};
use opengfd_proc::GfdRcAuto;
use std::{
    error::Error,
    fmt::{ Debug, Display },
    ffi::c_void,
    io::Cursor,
    ptr::NonNull
};

#[cfg(feature = "image_loader")]
use image::ImageReader;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TextureFlags: u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
        const FLAG3 = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
        const FLAG16 = 1 << 16;
        const FLAG17 = 1 << 17;
        const FLAG18 = 1 << 18;
        const FLAG19 = 1 << 19;
        const FLAG20 = 1 << 20;
        const FLAG21 = 1 << 21;
        const FLAG22 = 1 << 22;
        const FLAG23 = 1 << 23;
        const FLAG24 = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const NO_CREATING_RESOURCE = 1 << 30;
        const NO_TEXTURE_LIST = 1 << 31;
    }
}

pub type TexHandle = Option<NonNull<TextureResource<AllocatorHook>>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextureLoadError {
    CantGenerateTextureHandle
}

impl Error for TextureLoadError {}
impl Display for TextureLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture Load Error: {:?}", *self)
    }
}

#[repr(C)]
#[derive(GfdRcAuto)]
pub struct Texture<A = GfdAllocator> 
where A: Allocator + Clone
{
    pub(crate) flags: TextureFlags,
    handle: TexHandle,
    pub(crate) ref_: Reference,
    pub(crate) name: Name<A>,
    pub(crate) min: u8,
    pub(crate) mag: u8,
    pub(crate) wraps: u8,
    pub(crate) wrapt: u8,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    pub(crate) file_flags: FileFlags,
    _allocator: A
}

impl<A> Debug for Texture<A>
where A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture {{ flags: {:?}, handle: 0x{:x}, min: {}, mag: {}, wraps: {}, wrapt: {} }}",
        self.flags, match self.handle { Some(i) => i.as_ptr() as usize, None => 0 },
        self.min, self.mag, self.wraps, self.wrapt)
    }
}

impl<A> Texture<A> 
where A: Allocator + Clone
{
    pub unsafe fn get_handle(&self) -> Option<&TextureResource> { unsafe { self.handle.map(|v| v.as_ref() ) } }
    pub unsafe fn get_handle_mut(&mut self) -> Option<&mut TextureResource> { unsafe { self.handle.map(|mut v| v.as_mut() ) } }

    pub fn get_next(&self) -> Option<&Self> { unsafe { self.next.map(|v| v.as_ref()) } }
    pub fn get_prev(&self) -> Option<&Self> { unsafe { self.prev.map(|v| v.as_ref()) } }
    pub fn get_next_mut(&mut self) -> Option<&mut Self> { unsafe { self.next.map(|mut v| v.as_mut()) } }
    pub fn get_prev_mut(&mut self) -> Option<&mut Self> { unsafe { self.prev.map(|mut v| v.as_mut()) } }

    pub fn get_name(&self) -> &Name<A> { &self.name }

    pub fn get_width(&self) -> Option<u32> { unsafe { self.get_handle().map(|v| v.get_width()) } }
    pub fn get_height(&self) -> Option<u32> { unsafe { self.get_handle().map(|v| v.get_height()) } }
    pub fn get_raw(&self) -> Option<*mut c_void> { unsafe { self.get_handle().map(|v| v.get_raw()) }}

    pub fn get_texture_flags(&self) -> TextureFlags { self.flags }
    pub fn get_file_flags(&self) -> FileFlags { self.file_flags }

    pub fn set_texture_flags(&mut self, flag: TextureFlags) { self.flags = flag }
    pub fn set_file_flags(&mut self, flag: FileFlags) { self.file_flags = flag }

    pub fn is_ready(&self) -> bool { !self.file_flags.contains(FileFlags::NOT_READY) }
}

impl<A> Texture<A> 
where A: Allocator + Clone + Debug
{
    // Original function: 0x14105dc20 (Metaphor Steam Prologue Demo 1.01)
    pub fn from_dds_stream(bytes: &[u8], flags: TextureFlags, allocator: A) -> Result<GfdRc<Self, A>, TextureLoadError> {
        let handle = match flags.contains(TextureFlags::NO_CREATING_RESOURCE) {
            true => None, false => {
                let handle = TextureResource::new_from_dds(bytes).ok_or(TextureLoadError::CantGenerateTextureHandle)?;
                Some(unsafe { NonNull::new_unchecked(GfdRc::into_raw(handle) as *mut TextureResource<AllocatorHook>) })
            }
        };
        let mut new = Self::new_with_handle(handle, flags, allocator);
        if !new.flags.contains(TextureFlags::NO_TEXTURE_LIST) { new.insert_to_texture_list(); }
        Ok(new)
    }

    fn new_with_handle(handle: TexHandle, flags: TextureFlags, allocator: A) -> GfdRc<Self, A> {
        GfdRc::new_in(Self {
            flags,
            handle,
            ref_: Reference::new(),
            name: Name::empty_in(allocator.clone()),
            min: 1,
            mag: 1,
            wraps: 0,
            wrapt: 0,
            prev: None,
            next: None,
            file_flags: FileFlags::empty(),
            _allocator: allocator.clone()
        }, allocator)
    }

    fn insert_to_texture_list(&mut self) {
        let glb = GraphicsGlobal::get_gfd_graphics_global_mut();
        let mut tex = glb.lock_texture_mutex();
        if *tex != std::ptr::null_mut() {
            let list_head = unsafe { &mut *(*tex as *mut Texture<A>) };
            self.next = Some( unsafe { NonNull::new_unchecked(*tex as *mut Texture<A>) });
            list_head.prev = Some( unsafe { NonNull::new_unchecked(self as *mut _) } );
        }
        *tex = self as *mut Texture<A> as *mut _;
    }

}

#[cfg(feature = "image_loader")]
impl<A> Texture<A> 
where A: Allocator + Clone + Debug
{
    pub fn from_image_stream(bytes: &[u8], flags: TextureFlags, allocator: A) -> Result<GfdRc<Self, A>, Box<dyn Error>> {
        let img = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?.decode()?;
        let handle = match flags.contains(TextureFlags::NO_CREATING_RESOURCE) {
            true => None, false => {
                let handle = TextureResource::new_from_image(&img.into_rgba8())?;
                Some(unsafe { NonNull::new_unchecked(GfdRc::into_raw(handle) as *mut TextureResource<AllocatorHook>) })
            }
        };
        let mut new = Self::new_with_handle(handle, flags, allocator);
        if !new.flags.contains(TextureFlags::NO_TEXTURE_LIST) { new.insert_to_texture_list(); }
        Ok(new)
    }
}