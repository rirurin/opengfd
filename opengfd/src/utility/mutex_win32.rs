use std::{
    mem::MaybeUninit,
    sync::atomic::{ AtomicU32, Ordering },
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
pub struct RecursiveMutexGuard<'a, T>{
    mutex: &'a mut RecursiveMutex,
    data: &'a mut T
}
impl<'a, T> RecursiveMutexGuard<'a, T> {
    fn new(mutex: &'a mut RecursiveMutex, data: &'a mut T) -> Self {
        unsafe { EnterCriticalSection(&mut mutex.0 as *mut CRITICAL_SECTION) }
        Self { mutex, data }
    }
}
impl<'a, T> Drop for RecursiveMutexGuard<'a, T> {
    fn drop(&mut self) { 
        unsafe { LeaveCriticalSection(&mut self.mutex.0 as *mut CRITICAL_SECTION) };
    }
}
impl<'a, T> Deref for RecursiveMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a, T> DerefMut for RecursiveMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

#[derive(Debug)]
pub struct RecursiveMutex(CRITICAL_SECTION);
impl RecursiveMutex {
    /// (Original function: gfdRecursiveMutexInitialize)
    pub fn new() -> Self {
        let mut platform: MaybeUninit<CRITICAL_SECTION> = MaybeUninit::uninit(); 
        // SAFETY: Valid pointer to stack-allocated platform variable
        // https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-initializecriticalsectionandspincount#return-value
        unsafe { InitializeCriticalSectionAndSpinCount(platform.as_mut_ptr(), 1500).unwrap() };
        Self(unsafe { platform.assume_init() })
    }
    pub fn lock<'a, T>(&'a mut self, data: &'a mut T) -> RecursiveMutexGuard<'a, T> {
        RecursiveMutexGuard::new(self, data)
    }
}

impl Drop for RecursiveMutex {
    fn drop(&mut self) {
        // SAFETY: self.0 can't be accessed after this
        unsafe { DeleteCriticalSection(&mut self.0 as *mut CRITICAL_SECTION); }
    }
}

unsafe impl Send for RecursiveMutex {}
unsafe impl Sync for RecursiveMutex {}

#[derive(Debug)]
pub struct Mutex(AtomicU32);

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

pub(crate) const SPIN_COUNT_BEFORE_YIELDING: usize = 1500;

pub struct MutexGuard<'a, T> 
where T: ?Sized
{
    mutex: &'a mut Mutex,
    data: &'a mut T
}
impl<'a, T> MutexGuard<'a, T> 
where T: ?Sized
{
    fn new(mutex: &'a mut Mutex, data: &'a mut T) -> Self {
        loop {
            for _ in 0..SPIN_COUNT_BEFORE_YIELDING {
                if let Ok(_) = mutex.0.compare_exchange(0, 1, 
                Ordering::Acquire, Ordering::Relaxed) {
                    return Self { mutex, data };
                }
            }
            std::thread::yield_now();
        }
    }
}
impl<'a, T> Drop for MutexGuard<'a, T> 
where T: ?Sized
{
    fn drop(&mut self) {
        self.mutex.0.store(0, Ordering::Release);
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> 
where T: ?Sized
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data
    }
}
impl<'a, T> DerefMut for MutexGuard<'a, T> 
where T: ?Sized
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

impl Mutex {
    pub fn new() -> Self { Self(AtomicU32::new(0)) }
    pub fn lock<'a, T>(&'a mut self, data: &'a mut T) -> MutexGuard<'a, T> 
    where T: ?Sized { 
        MutexGuard::new(self, data) 
    }
}

pub mod ffi {

}
