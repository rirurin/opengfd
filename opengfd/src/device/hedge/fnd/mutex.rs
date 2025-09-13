use std::{
    mem::MaybeUninit,
    ops::{ Deref, DerefMut }
};
use windows::Win32::System::Threading::{
    CRITICAL_SECTION,
    DeleteCriticalSection,
    EnterCriticalSection,
    InitializeCriticalSectionAndSpinCount,
    LeaveCriticalSection
};

#[derive(Debug)]
pub struct MutexGuard<'a, T>{
    mutex: &'a mut Mutex,
    data: &'a mut T
}
impl<'a, T> MutexGuard<'a, T> {
    fn new(mutex: &'a mut Mutex, data: &'a mut T) -> Self {
        unsafe { EnterCriticalSection(&mut mutex.0 as *mut CRITICAL_SECTION) }
        Self { mutex, data }
    }
}
impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe { LeaveCriticalSection(&mut self.mutex.0 as *mut CRITICAL_SECTION) };
    }
}
impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

#[derive(Debug)]
pub struct Mutex(CRITICAL_SECTION);
impl Mutex {
    /// (Original function: gfdMutexInitialize)
    pub fn new() -> Self {
        let mut platform: MaybeUninit<CRITICAL_SECTION> = MaybeUninit::uninit();
        // SAFETY: Valid pointer to stack-allocated platform variable
        // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-initializecriticalsectionandspincount#return-value
        unsafe { InitializeCriticalSectionAndSpinCount(platform.as_mut_ptr(), 1500).unwrap() };
        Self(unsafe { platform.assume_init() })
    }
    pub fn lock<'a, T>(&'a mut self, data: &'a mut T) -> MutexGuard<'a, T> {
        MutexGuard::new(self, data)
    }
}

impl Drop for Mutex {
    fn drop(&mut self) {
        // SAFETY: self.0 can't be accessed after this
        unsafe { DeleteCriticalSection(&mut self.0 as *mut CRITICAL_SECTION); }
    }
}

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}