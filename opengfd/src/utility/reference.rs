use allocator_api2::alloc::{ Allocator, Global };
use std::{
    alloc::Layout,
    fmt::{ Debug, Display },
    mem::ManuallyDrop,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{ AtomicU32, Ordering }
};

#[repr(C)]
#[derive(Debug)]
pub struct Reference(AtomicU32);
impl Reference {
    pub fn new() -> Self { Reference(1.into()) }
    pub fn count(&self) -> u32 { unsafe { *self.0.as_ptr() } }
    pub fn add_ref(&self) -> u32 { self.0.fetch_add(1, Ordering::Relaxed) }
    pub fn release(&self) -> u32 { self.0.fetch_sub(1, Ordering::Relaxed) }
}

pub trait GfdRcType {
    /// (Original function: gfdMutexAddRef)
    fn add_ref(&self) -> u32;
    fn count(&self) -> u32;
    fn release(&self) -> u32;
}

pub struct GfdRc<T, A> 
where T: GfdRcType,
      A: Allocator + Clone
{
    inner: NonNull<T>,
    _allocator: A
}

impl<T> GfdRc<T, Global>
where T: GfdRcType
{
    pub fn new(val: T) -> Self {
        Self::new_in(val, Global)
    }
}

// A wrapper to implement a managed reference-counting data structure for GFD types that contain a
// reference field. Unlike std's Rc<T>, this requires that the inner type stores the reference
// count and implements the GfcRcType trait to allow for GfdRc to manage it.
// This closely follows the API of Rc<T>.
impl<T, A> GfdRc<T, A> 
where T: GfdRcType,
      A: Allocator + Clone
{
    pub fn new_in(val: T, alloc: A) -> Self
    where A: Allocator
    {
        // :ADAHCI:
        assert!(std::mem::size_of::<T>() >= std::mem::size_of::<Reference>(), 
        "GfdRc box must be for a type large enough to hold a reference!");
        let layout = Layout::new::<T>();
        let inner: NonNull<T> = alloc.allocate(layout).unwrap().cast();
        // SAFETY: No existing instance of T lives here
        unsafe { inner.write(val); }
        Self {
            inner,
            _allocator: alloc
        }
    }

    pub fn as_ptr(&self) -> *const T {
        &**self as *const T
    }

    pub fn into_raw(this: Self) -> *const T {
        let this = ManuallyDrop::new(this);
        this.as_ptr()
    }

    pub fn clone_from_raw(ptr: *const T, alloc: A) -> Self {
        let out = Self::from_raw(ptr, alloc);
        (*out).add_ref();
        out
    }

    pub fn from_raw(ptr: *const T, alloc: A) -> Self {
        Self {
            inner: NonNull::new(ptr as *mut T).unwrap(),
            _allocator: alloc
        }
    }

    pub fn count(&self) -> u32 {
        (&**self).count()
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self.count() {
            1 => Some(unsafe { self.get_mut_unchecked() }),
            _ => None
        }
    }

    pub unsafe fn get_mut_unchecked(&mut self) -> &mut T {
        self.inner.as_mut()
    }
}

impl<T, A> Clone for GfdRc<T, A>
where T: GfdRcType,
      A: Allocator + Clone
{
    fn clone(&self) -> Self {
        (*self).add_ref();
        Self { 
            inner: self.inner,
            _allocator: self._allocator.clone()
        }
    }
}

impl<T, A> Deref for GfdRc<T, A> 
where T : GfdRcType,
      A: Allocator + Clone
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl<T, A> Drop for GfdRc<T, A> 
where T: GfdRcType,
      A: Allocator + Clone
{
    fn drop(&mut self) {
        if (*self).release() == 1 {
            // SAFETY: We're the last thing to see this allocation
            unsafe { std::ptr::drop_in_place(self.inner.as_ptr()); }
        }
    }
}

// Check if inner types are equal, even if they may be stored in different allocations
impl<T, A> PartialEq for GfdRc<T, A>
where T: GfdRcType + PartialEq,
      A: Allocator + Clone
{
    fn eq(&self, other: &Self) -> bool {
        &**self == &**other
    }
}

impl<T, A> PartialOrd for GfdRc<T, A>
where T: GfdRcType + PartialOrd,
      A: Allocator + Clone
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (&**self).partial_cmp(&**other)
    }
}

impl<T, A> Debug for GfdRc<T, A>
where T: GfdRcType + Debug,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&**self).fmt(f)
    }
}

impl<T, A> Display for GfdRc<T, A>
where T: GfdRcType + Display,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&**self).fmt(f)
    }
}

#[cfg(test)]
pub mod tests {
    #[allow(unused_imports)]
    use crate::tests::{ OpengfdError, TestReturn };
}
