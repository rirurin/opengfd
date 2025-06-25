// #![allow(dead_code)]

use std::{
    mem::MaybeUninit,
    // ptr::NonNull
};

#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Time {
    sec: u16,
    min: u16,
    hour: u16,
    mday: u16,
    mon: u16,
    year: u32,
    wday: u16
}

impl Time {
    #[cfg(target_os = "windows")]
    pub fn new() -> Time {
        // https://en.cppreference.com/w/c/chrono/time
        let time = unsafe { libc::time(0 as *mut libc::time_t) };
        let mut local_time: MaybeUninit<libc::tm> = MaybeUninit::uninit();
        // https://en.cppreference.com/w/c/chrono/localtime
        unsafe { libc::localtime_s(local_time.as_mut_ptr(), &time as *const libc::time_t) };
        let local_time = unsafe { local_time.assume_init() };
        Self {
            sec: local_time.tm_sec as u16,
            min: local_time.tm_min as u16,
            hour: local_time.tm_hour as u16,
            mday: local_time.tm_mday as u16,
            mon: local_time.tm_mon as u16,
            year: local_time.tm_year as u32 + 1900,
            wday: local_time.tm_wday as u16
        }
    }
    #[cfg(target_family = "unix")]
    pub fn new() -> Time {
        panic!("TODO: Implement gfdTime for Unix")
    }

    pub fn create_seed(&self) -> u32 {
        ((self.hour as u32 + ((self.year - 1900) * 365 + self.mon as u32 * 31 + self.mday as u32) * 24)
        * 60 + self.min as u32) * 60 + self.sec as u32
    }
    pub fn get_sec(&self) -> u32 { self.sec as u32 }
    pub fn get_min(&self) -> u32 { self.min as u32 }
    pub fn get_hour(&self) -> u32 { self.hour as u32 }
    pub fn get_mday(&self) -> u32 { self.mday as u32 }
    pub fn get_mon(&self) -> u32 { self.mon as u32 }
    pub fn get_year(&self) -> u32 { self.year }
    pub fn get_wday(&self) -> u32 { self.wday as u32 }
}

const RAND_TO_F32: f32 = 0.000000059604645f32;
const RAND_F32_SUB: f32 = 0.5f32;
const RAND_UNIT_MASK: u32 = 0xffffff;

/// A psuedo-random value generator used throughout GFD and in game for logic that requires
/// randomization.
/// Original struct: gfdRandSOL
pub trait Random {
    /// (gfdRandInitialize)
    fn new() -> Self;
    /// (gfdSRandEx)
    fn new_seed(seed: u32) -> Self;
    /// (gfdRand, gfdRandEx)
    fn rand(&mut self) -> u32;
    /// (gfdUnitRandEx)
    fn rand_f32_unit(&mut self) -> f32 {
        (self.rand() & RAND_UNIT_MASK) as f32 * RAND_TO_F32
    }
    /// (gfdSymmetricRandEx)
    fn rand_f32_symmetric(&mut self) -> f32 {
        self.rand_f32_unit() - RAND_F32_SUB
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RandomUnaligned([u32; 4]);

impl Random for RandomUnaligned {
    // 0x141059950
    fn new() -> Self {
        Self::new_seed(Time::new().create_seed())
    }

    fn new_seed(seed: u32) -> Self {
        let v0 = seed ^ 0xaed1a0c;
        let v1 = v0.rotate_right(8);
        let v2 = (v1 ^ 0xaa5a02fe).rotate_right(8);
        let v3 = (v2 ^ 0x11be81c7).rotate_right(8);
        Self([v0, v1, v2, v3])
    }

    fn rand(&mut self) -> u32 {
        self.0.rotate_right(1);
        self.0[0] = (self.0[2] << 2 | self.0[1] >> 0x1e) ^ (self.0[0] << 1 | self.0[3] >> 0x1f);
        self.0[0]
    }
}

#[repr(align(16))]
#[derive(Debug, Clone, PartialEq)]
pub struct RandomAligned([u32; 4]);

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    use std::arch::x86_64::__m128i;
    impl super::RandomAligned {
        pub unsafe fn rand_platform(&mut self) -> u32 {
            let simd = std::mem::transmute::<[u32; 4], __m128i>(self.0);
            let rotated = std::arch::x86_64::_mm_shuffle_epi32::<0x55>(simd);
            self.0 = std::mem::transmute::<__m128i, [u32; 4]>(rotated);
            self.0[0] = (self.0[2] << 2 | self.0[1] >> 0x1e) ^ (self.0[0] << 1 | self.0[3] >> 0x1f);
            self.0[0]
        }
    }
}

impl Random for RandomAligned {
    fn new() -> Self {
        let time = Time::new();
        Self::new_seed(((time.hour as u32 + ((time.year - 1900) * 365 + time.mon as u32 * 31 + time.mday as u32) * 24)
            * 60 + time.min as u32) * 60 + time.sec as u32)
    }

    fn new_seed(seed: u32) -> Self {
        let mut out: Self = Self([0, 0, 0, 0]);
        let v0 = seed ^ 0xaed1a0c;
        let v1 = v0.rotate_right(8);
        let v2 = (v1 ^ 0xaa5a02fe).rotate_right(8);
        let v3 = (v2 ^ 0x11be81c7).rotate_right(8);
        out.0 = [v0, v1, v2, v3];
        out
    }

    fn rand(&mut self) -> u32 {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            self.rand_platform()
        }
    }
}

pub struct EPLRandom;
impl EPLRandom {
    pub fn rand(seed: u32) -> u32 {
        seed * 0xfd43fd + 0xc39ec3
    }
    pub fn rand_f32(seed: &mut u32) -> f32 {
        *seed = Self::rand(*seed);
        *seed as f32 * RAND_TO_F32
    }
    pub fn rand_f32_pm(seed: &mut u32) -> f32 {
        Self::rand_f32(seed) - RAND_F32_SUB
    }
}

#[allow(non_snake_case)]
pub mod ffi {
    use super::{ Time, Random, RandomUnaligned, EPLRandom };
    #[no_mangle]
    pub unsafe extern "C" fn gfdEPLRand(pSeed: *mut u32) -> u32 {
        *pSeed = EPLRandom::rand(*pSeed);
        *pSeed
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdEPLRandFloat(pSeed: *mut u32) -> f32 {
        EPLRandom::rand_f32(pSeed.as_mut().unwrap())
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdEPLRandFloatPM(pSeed: *mut u32) -> f32 {
        EPLRandom::rand_f32_pm(pSeed.as_mut().unwrap())
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdRandEx(pSOL: *mut RandomUnaligned) -> u32 {
        pSOL.as_mut().unwrap().rand()
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdSRandEx(pSOL: *mut RandomUnaligned, seed: u32) {
        // set random state in place
        let v0 = seed ^ 0xaed1a0c;
        let v1 = v0.rotate_right(8);
        let v2 = (v1 ^ 0xaa5a02fe).rotate_right(8);
        let v3 = (v2 ^ 0x11be81c7).rotate_right(8);
        pSOL.as_mut().unwrap().0 = [v0, v1, v2, v3]
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdSRandeByTime(pSOL: *mut RandomUnaligned) {
        gfdSRandEx(pSOL, Time::new().create_seed());
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdSymmetricRandEx(pSOL: *mut RandomUnaligned) -> f32 {
        pSOL.as_mut().unwrap().rand_f32_symmetric()
    }
    #[no_mangle]
    pub unsafe extern "C" fn gfdUnitRandEx(pSOL: *mut RandomUnaligned) -> f32 {
        pSOL.as_mut().unwrap().rand_f32_unit()
    }
}

#[cfg(test)]
pub mod tests {

    use super::{ Random, RandomAligned, RandomUnaligned };
    use std::error::Error;
    type TestReturn = Result<(), Box<dyn Error>>;

    #[test]
    fn init_random_unaligned_from_seed_1() -> TestReturn {
        let seed = 1u32;
        let rng = RandomUnaligned::new_seed(seed);
        let expected: [u32; 4] = [0xaed1a0d, 0xd0aed1a, 0xe4a750ef, 0x28f519d1];
        assert_eq!(rng.0[0], expected[0], "0x{:x} != 0x{:x} (expected value)", rng.0[0], expected[0]);
        assert_eq!(rng.0[1], expected[1], "0x{:x} != 0x{:x} (expected value)", rng.0[1], expected[1]);
        assert_eq!(rng.0[2], expected[2], "0x{:x} != 0x{:x} (expected value)", rng.0[2], expected[2]);
        assert_eq!(rng.0[3], expected[3], "0x{:x} != 0x{:x} (expected value)", rng.0[3], expected[3]);
        Ok(())
    }
    #[test]
    fn init_random_aligned_from_seed_1() -> TestReturn {
        let seed = 1u32;
        let rng = RandomAligned::new_seed(seed);
        let expected: [u32; 4] = [0xaed1a0d, 0xd0aed1a, 0xe4a750ef, 0x28f519d1];
        assert_eq!(rng.0[0], expected[0], "0x{:x} != 0x{:x} (expected value)", rng.0[0], expected[0]);
        assert_eq!(rng.0[1], expected[1], "0x{:x} != 0x{:x} (expected value)", rng.0[1], expected[1]);
        assert_eq!(rng.0[2], expected[2], "0x{:x} != 0x{:x} (expected value)", rng.0[2], expected[2]);
        assert_eq!(rng.0[3], expected[3], "0x{:x} != 0x{:x} (expected value)", rng.0[3], expected[3]);
        Ok(())
    }

    #[test]
    #[ignore]
    fn rand_unaligned_generate_values() -> TestReturn {
        Ok(())
    }

    #[test]
    #[ignore]
    fn rand_aligned_generate_values() -> TestReturn {
        Ok(())
    }
    // #[test]
    // fn init_random_unaligned_from_seed_
}
