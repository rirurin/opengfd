#![allow(unused_imports, dead_code)]
use allocator_api2::{
    alloc::{ Allocator, Global },
    boxed::Box
};
use std::{
    alloc::Layout,
    fmt::{ Debug, Display },
    marker::PhantomPinned,
    ops::{ Deref, DerefMut },
    pin::Pin,
    ptr::NonNull
};

/// A reference counted pointer built using an intrusive doubly linked list. Instances of
/// SmartPointers can (and usually are) stack allocated. To implement this behavior within Rust,
/// the Pin<Ptr> is used, and it's the responsibility of the caller to ensure that stack values are
/// properly pinned and linked.
/// ```ignore
/// let value: String = format!("True...");
/// let mut smart = SmartPointer::new_in(value, Global);
/// let mut smart = unsafe { Pin::new_unchecked(&mut smart) };
/// let mut smart2 = SmartPointer::uninit(Global);
/// let mut smart2 = unsafe { Pin::new_unchecked(&mut smart2) };
/// SmartPointer::link(smart2.as_mut(), smart.as_mut());
/// ```
/// When every SmartPointer pointed to some value are dropped, the value drops.
///
/// When using SmartPointer<T> with hooked code, be wary that initializing new SmartPointers will
/// create a null vtable pointer, since we don't know where the precompiled code stores it's
/// vtables (there's too many to reasonably hook either). If you intend to pass a Rust SmartPointer
/// as a parameter into a C++ function, it should've been made as a clone from a C++ SmartPointer.
#[repr(C)]
pub struct SmartPointer<T, A = Global>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    prev: Option<NonNull<Self>>,
    next: Option<NonNull<Self>>,
    data: Option<NonNull<T>>,
    _allocator: A,
    _pinned: PhantomPinned
}

impl<T> SmartPointer<T, Global> {
    pub fn new(data: T) -> Self { Self::new_in(data, Global) }
    pub fn new_with_vtable(vtable: *const u8, data: T) -> Self { Self::new_in_with_vtable(vtable, data, Global) }
    pub unsafe fn new_ref(data: &T) -> Self { Self::new_ref_in(data, Global) }
    pub unsafe fn new_ref_with_vtable(vtable: *const u8, data: &T) -> Self { Self::new_ref_in_with_vtable(vtable, data, Global) }
    pub unsafe fn new_mut(data: &mut T) -> Self { Self::new_mut_in(data, Global) }
    pub unsafe fn new_mut_with_vtable(vtable: *const u8, data: &mut T) -> Self { Self::new_mut_in_with_vtable(vtable, data, Global) }
    pub unsafe fn new_ptr(data: *mut T) -> Self { Self::new_ptr_in(std::ptr::null(), data, Global) }
    pub unsafe fn new_ptr_with_vtable(vtable: *const u8, data: *mut T) -> Self { Self::new_ptr_in(vtable, data, Global) }
}

impl<T, A> SmartPointer<T, A>
where A: Allocator + Clone
{
    /// Create a new SmartPointer to some value. This will provide reference counted ownership
    /// semantics to the value, only dropping it if every referant is dropped
    pub fn new_in(data: T, alloc: A) -> Self {
        Self::new_in_with_vtable(std::ptr::null(), data, alloc)
    }
    /// Create a new SmartPointer to some value. This will provide reference counted ownership
    /// semantics to the value, only dropping it if every referant is dropped
    pub fn new_in_with_vtable(vtable: *const u8, data: T, alloc: A) -> Self {
        let ptr = alloc.allocate(Self::get_layout()).unwrap().as_ptr() as *mut T;
        unsafe { std::ptr::write(ptr, data) }
        unsafe { Self::new_ptr_in(vtable, ptr, alloc) }
    }
    /// Create a new SmartPointer to an existing shared reference. This is marked unsafe since
    /// SmartPointer assumes that the data was heap-allocated using a matching allocator. Failing
    /// to meet that will cause a panic when dropped
    pub unsafe fn new_ref_in(data: &T, alloc: A) -> Self {
        Self::new_ref_in_with_vtable(std::ptr::null(), data, alloc)
    }
    /// Create a new SmartPointer to an existing shared reference. This is marked unsafe since
    /// SmartPointer assumes that the data was heap-allocated using a matching allocator. Failing
    /// to meet that will cause a panic when dropped
    pub unsafe fn new_ref_in_with_vtable(vtable: *const u8, data: &T, alloc: A) -> Self {
        Self::new_ptr_in(vtable, &raw const *data as *mut T, alloc)
    }
    /// Create a new SmartPointer to an existing mutable reference. This is marked unsafe since
    /// SmartPointer assumes that the data was heap-allocated using a matching allocator. Failing
    /// to meet that will cause a panic when dropped
    pub unsafe fn new_mut_in(data: &mut T, alloc: A) -> Self {
        Self::new_mut_in_with_vtable(std::ptr::null(), data, alloc)
    }
    /// Create a new SmartPointer to an existing mutable reference. This is marked unsafe since
    /// SmartPointer assumes that the data was heap-allocated using a matching allocator. Failing
    /// to meet that will cause a panic when dropped
    pub unsafe fn new_mut_in_with_vtable(vtable: *const u8, data: &mut T, alloc: A) -> Self {
        Self::new_ptr_in(vtable, &raw mut *data, alloc)
    }
    /// Create a new SmartPointer to an existing heap allocation. Ensure that the allocated data
    /// uses the same allocator as this type.
    pub unsafe fn new_ptr_in(vtable: *const u8, data: *mut T, alloc: A) -> Self {
        Self {
            _cpp_vtable: vtable,
            prev: None,
            next: None,
            data: if data != std::ptr::null_mut() {
                Some(unsafe { NonNull::new_unchecked(data) })
            } else { None },
            _allocator: alloc,
            _pinned: PhantomPinned
        }
    }
    /// Create an uninitialized SmartPointer. Ensure that this is linked before using it since
    /// trying to access the value of an uninitialized SmartPointer will cause a panic.
    pub fn uninit(alloc: A) -> Self { unsafe { Self::new_ptr_in(std::ptr::null(), std::ptr::null_mut(), alloc) } }
    /// Create an uninitialized SmartPointer with a defined C++ vtable. Ensure that this is linked before
    /// using it since trying to access the value of an uninitialized SmartPointer will cause a panic.
    pub fn uninit_with_vtable(vtable: *const u8, alloc: A) -> Self { unsafe { Self::new_ptr_in(vtable, std::ptr::null_mut(), alloc) } }
    /// Link an uninitalized pointer at `new` onto the existing referant chain in `src`. `new` will
    /// be inserted after `src`.
    /// Original function: gfw::SmartPointer::SmartPointer and gfw::SmartPointer::operator=
    pub fn link(new: Pin<&mut Self>, src: Pin<&mut Self>) {
        if std::ptr::addr_eq(&raw const *new.as_ref(), &raw const *src.as_ref()) { return }
        if src.is_unique() {            // src.as_mut().next = Some(Self::make_nonnull_from_pinned_ptr(new));
            let src_mut = unsafe { src.get_unchecked_mut() };
            let new_mut = unsafe { new.get_unchecked_mut() };
            src_mut.next = Some(unsafe { NonNull::new_unchecked(&raw mut *new_mut) });
            new_mut.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src_mut) });
            new_mut.data = src_mut.data;
        } else {
            let src_mut = unsafe { src.get_unchecked_mut() };
            let new_mut = unsafe { new.get_unchecked_mut() };
            new_mut.next = src_mut.next;
            new_mut.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src_mut) });
            if let Some(mut n) = new_mut.next {
                unsafe { n.as_mut() }.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *new_mut) });
            }
            src_mut.next = Some(unsafe { NonNull::new_unchecked(&raw mut *new_mut) });
            new_mut.data = src_mut.data;
        }
    }
    /// Like `link`, but with `new` set as an unpinned pointer. This is used if the new
    /// SmartPointer is located inside the heap, such as in an entry in PointerList
    pub fn link_unpin0(&mut self, src: Pin<&mut Self>) {
        if std::ptr::addr_eq(&raw const *self, &raw const *src.as_ref()) { return }
        if src.is_unique() {            // src.as_mut().next = Some(Self::make_nonnull_from_pinned_ptr(new));
            let src_mut = unsafe { src.get_unchecked_mut() };
            src_mut.next = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            self.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src_mut) });
            self.data = src_mut.data;
        } else {
            let src_mut = unsafe { src.get_unchecked_mut() };
            self.next = src_mut.next;
            self.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src_mut) });
            if let Some(mut n) = self.next {
                unsafe { n.as_mut() }.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            }
            src_mut.next = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            self.data = src_mut.data;
        }
    }

    pub fn link_unpin(&mut self, src: &mut Self) {
        if std::ptr::addr_eq(&raw const *self, &raw const *src) { return }
        if src.is_unique() {
            src.next = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            self.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src) });
            self.data = src.data;
        } else {
            self.next = src.next;
            self.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *src) });
            if let Some(mut n) = self.next {
                unsafe { n.as_mut() }.prev = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            }
            src.next = Some(unsafe { NonNull::new_unchecked(&raw mut *self) });
            self.data = src.data;
        }
    }

    fn get_next(&self) -> Option<&Self> {
        unsafe { self.next.map(|p| p.as_ref()) }
    }
    fn get_prev(&self) -> Option<&Self> {
        unsafe { self.prev.map(|p| p.as_ref()) }
    }
    fn get_next_mut(&mut self) -> Option<&mut Self> {
        unsafe { self.next.map(|mut p| p.as_mut()) }
    }
    fn get_prev_mut(&self) -> Option<&mut Self> {
        unsafe { self.prev.map(|mut p| p.as_mut()) }
    }

    // Get a raw pointer to the data. This method is private for now - use the Deref/DerefMut trait
    // or call get_ref()/get_mut() to obtain a reference to the data.
    fn get_data_raw(&self) -> *mut T { self.data.map_or_else(|| std::ptr::null_mut(), |f| f.as_ptr()) }
    /// Get a shared reference to the data stored inside of the SmartPointer. This acts identically
    /// to the Deref trait implementation.
    pub fn get_data(&self) -> &T { unsafe { self.data.map(|f| f.as_ref()).unwrap() } }
    pub fn get_data_checked(&self) -> Option<&T> { self.data.map(unsafe { |f| f.as_ref() }) }
    /// Get a mutable reference to the data stored inside of the SmartPointer. This acts identically
    /// to the DerefMut trait implementation.
    pub fn get_data_mut(&mut self) -> &mut T { unsafe { self.data.map(|mut f| f.as_mut()).unwrap() } }
    pub fn get_data_checked_mut(&mut self) -> Option<&mut T> { self.data.map(unsafe { |mut f| f.as_mut() }) }
    fn get_layout() -> Layout { Layout::new::<T>() }
    /// Is this SmartPointer the only reference to the data?
    pub fn is_unique(&self) -> bool { self.get_next().is_none() && self.get_prev().is_none() }
    pub fn is_initialized(&self) -> bool { self.data.is_some() }

    pub fn set_cpp_vtable(&mut self, vtable: *const u8) {
        self._cpp_vtable = vtable;
    }
    pub fn get_cpp_vtable(&self) -> *const u8 {
        self._cpp_vtable
    }
}

impl<T, A> Debug for SmartPointer<T, A>
where T: Debug,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartPointer {{ prev: 0x{:x}, next: 0x{:x}, data: {:?} }}",
        match self.prev { Some(v) => v.as_ptr() as usize, None => 0 },
        match self.next { Some(v) => v.as_ptr() as usize, None => 0 },
        self.get_data())
    }
}

impl<T, A> PartialEq for SmartPointer<T, A>
where A: Allocator + Clone
{
    /// Original function: gfw::SmartPointer::operator==
    /// Check that two SmartPointers are referring to the same object.
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T, A> SmartPointer<T, A>
where T: Debug,
      A: Allocator + Clone
{
    fn internal_print_chain(&self) {
        let mut chain = Some(self);
        while let Some(c) = chain {
            match c.get_prev() {
                Some(v) => chain = Some(v),
                None => break
            }
        }
        while let Some(c) = chain {
            println!("0x{:x}: {:?}", &raw const *c as usize, c);
            match c.get_next() {
                Some(v) => chain = Some(v),
                None => break
            }
        }
    }
    fn internal_get_count(&self) -> usize {
        let mut chain = Some(self);
        while let Some(c) = chain {
            match c.get_prev() {
                Some(v) => chain = Some(v),
                None => break
            }
        }
        let mut count = 0;
        while let Some(c) = chain {
            count += 1;
            match c.get_next() {
                Some(v) => chain = Some(v),
                None => break
            }
        }
        count
    }
}

impl<T, A> Drop for SmartPointer<T, A>
where A: Allocator + Clone
{
    fn drop(&mut self) {
        if self.is_unique() {
            // last reference, so drop data
            let data = self.data.unwrap().as_ptr();
            unsafe { std::ptr::drop_in_place(data) }
            unsafe { self._allocator.deallocate(self.data.unwrap().cast(), Self::get_layout()); }
        } else {
            // unlink from chain
            let prev = self.prev;
            let next = self.next;
            if let Some(p) = self.get_prev_mut() { p.next = next; }
            if let Some(n) = self.get_next_mut() { n.prev = prev; }
        }
    }
}

impl<T, A> Display for SmartPointer<T, A>
where T: Display,
      A: Allocator + Clone
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_data())
    }
}

impl<T, A> Deref for SmartPointer<T, A>
where A: Allocator + Clone
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.get_data()
    }
}

impl<T, A> DerefMut for SmartPointer<T, A>
where A: Allocator + Clone
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_data_mut()
    }
}

#[cfg(test)]
pub mod tests {
    use std::{
        error::Error,
        fmt::Debug,
        pin::Pin
    };
    use allocator_api2::alloc::{ Allocator, Global } ;

    use super::SmartPointer;

    type TestReturn = Result<(), Box<dyn Error>>;

    impl<T, A> SmartPointer<T, A>
    where T: Debug + PartialEq,
          A: Allocator + Clone
    {
        fn check_count(&self, assert: usize) {
            assert!(self.internal_get_count() == assert, "There shoudld be {} SmartPointer in the chain", assert)
        }
        fn check_chain(&self, prev: Option<&Self>, next: Option<&Self>) {
            let err_prev = match prev { Some(v) => format!("{:?}", v), None => "empty".to_owned() };
            let err_next = match next { Some(v) => format!("{:?}", v), None => "empty".to_owned() };
            assert!(self.get_prev() == prev, "Previous pointer in chain from smart should be {}", &err_prev);
            assert!(self.get_next() == next, "Next pointer in chain from smart should be {}", &err_next);
        }
        fn check_equality(&self, ptr: &Self) {
            assert!(self.get_data() == ptr.get_data(), "Value inside cloned SmartPointer should be {:?} instead of {:?}",
            ptr.get_data(), self.get_data());
            assert!(self.get_data_raw() == ptr.get_data_raw(), 
            "Pointer to value should match between SmartPointers (0x{:x} vs 0x{:x})",
            self.get_data_raw() as usize, ptr.get_data_raw() as usize);
        }
        fn check_equality_value(&self, val: &T, ptr: Option<&Self>) {
            assert!(self.get_data() == val, "Value inside cloned SmartPointer should be {:?} instead of {:?}",
            val, self.get_data());
            if let Some(p) = ptr {
                assert!(self.get_data_raw() == p.get_data_raw(), 
                "Pointer to value should match between SmartPointers (0x{:x} vs 0x{:x})",
                self.get_data_raw() as usize, p.get_data_raw() as usize);
            }
        }
    }

    #[test]
    pub fn create_single_reference_pointer() -> TestReturn {
        let value: String = format!("test string!");
        let mut smart = SmartPointer::new(value.clone());
        let smart = unsafe { Pin::new_unchecked(&mut smart) };
        smart.check_equality_value(&value, None);
        smart.check_chain(None, None);
        smart.check_count(1);
        Ok(())
    }

    #[test]
    pub fn create_multiple_references() -> TestReturn {
        /*
        let value: String = format!("True...");
        let mut smart = SmartPointer::new_in(value, Global);
        let mut smart = unsafe { Pin::new_unchecked(&mut smart) };
        // Add second entry onto chain
        let mut smart2 = SmartPointer::uninit(Global);
        let mut smart2 = unsafe { Pin::new_unchecked(&mut smart2) };
        SmartPointer::link(smart2.as_mut(), smart.as_mut());
        smart.check_equality(&smart2);
        smart.check_chain(None, Some(&smart2));
        smart2.check_chain(Some(&smart), None);
        smart.check_count(2);
        // Add third entry onto chain
        {
            let mut smart3 = SmartPointer::uninit(Global);
            let mut smart3 = unsafe { Pin::new_unchecked(&mut smart3) };
            SmartPointer::link(smart3.as_mut(), smart.as_mut());
            smart.check_chain(None, Some(&smart3));
            smart3.check_chain(Some(&smart), Some(&smart2));
            smart2.check_chain(Some(&smart3), None);
            smart2.check_count(3);
        }
        // Third entry is dropped, check unlink
        smart.check_chain(None, Some(&smart2));
        smart2.check_chain(Some(&smart), None);
        smart.check_count(2);
        */
        Ok(())
    }
}
