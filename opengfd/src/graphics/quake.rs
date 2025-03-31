use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use crate::{
    kernel::allocator::GfdAllocator,
    object::camera::Camera,
    utility::reference::{
        GfdRc, GfdRcType, Reference
    }
};
use glam::Vec3A;
use opengfd_proc::GfdRcAuto;

#[repr(C)]
#[derive(Debug)]
pub struct QuakeParams {
    power: f32,
    pitch_weight: f32,
    total_time: f32,
    fade_in_time: f32,
    fade_out_time: f32,
}

impl QuakeParams {
    pub fn new() -> Self {
        Self {
            power: 0.,
            pitch_weight: 0.,
            total_time: 0.,
            fade_in_time: 0.,
            fade_out_time: 0.
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct QuakeFlags: u16 {
        const FINISH = 1 << 0;
        const PLAYING = 1 << 1;
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuakeState {
    Idle = 0,
    Start = 1,
    Playing = 2
}


#[repr(C)]
#[derive(Debug, GfdRcAuto)]
pub struct Quake {
    translate: Vec3A,
    time: f32,
    params: QuakeParams,
    flags: QuakeFlags,
    state: QuakeState,
    ref_: Reference,
}

impl Quake {
    /// Original function: gfdQuakeCreate
    pub fn new() -> GfdRc<Self, GfdAllocator> {
        Self::new_in(GfdAllocator)
    }
    pub fn new_in<A>(alloc: A) -> GfdRc<Self, A> 
    where A: Allocator + Clone {
        GfdRc::new_in(Self {
            translate: Vec3A::ZERO,
            time: 0.,
            params: QuakeParams::new(),
            flags: QuakeFlags::empty(),
            state: QuakeState::Idle,
            ref_: Reference::new(),
        }, alloc.clone())
    }

    /// Original function: gfdQuakeContinue
    pub fn continue_quake(&mut self) {
        if self.state == QuakeState::Idle { return; }
        self.state = QuakeState::Playing;
        self.flags |= QuakeFlags::PLAYING;
    }

    /// Original function: gfdQuakeGetTranslate
    pub fn get_translate(&self) -> Vec3A {
        self.translate
    }

    /// Original function: gfdQuakeStart
    pub fn start(&mut self, params: QuakeParams) {
        self.time = 0.;
        self.params = params;
        self.params.pitch_weight = self.params.pitch_weight.clamp(0., 1.);
        if self.params.total_time == 0. {
            self.flags |= QuakeFlags::FINISH;
        }
        self.state = QuakeState::Start;
    }

    /// Original function: gfdQuakeStop
    pub fn stop(&mut self, fadeout: f32) {
        if fadeout > 0. {
            self.params.fade_out_time = fadeout;
            self.flags &= QuakeFlags::PLAYING;
            self.params.total_time = self.time + fadeout;
        } else {
            self.state = QuakeState::Idle;
            self.flags &= QuakeFlags::PLAYING;
        }
    }

    /*
    /// Original function: gfdQuakeUpdate
    pub fn update(&mut self, camera: &Camera, delta: f32) {
        
        if self.state == QuakeState::Playing {
            if !self.flags.contains(QuakeFlags::PLAYING) {
                self.state = QuakeState::Idle;
            }
        }
        if self.time <= self.params.total_time || self.flags.contains(QuakeFlags::FINISH) {
            if self.state != QuakeState::Idle {
                if self.params.fade_in_time <= self.time {
                    if self.params.total_time - self.params.fade_out_time <= self.time
                } else if self.params.fade_in_time > 0 {

                }
                self.params.power = self.params.power.max(delta * 30. * self.params.power);
                self.params.power *= (1. - 0.);
            }
        } else {
            self.state = QuakeState::Idle;
            self.translate = Vec3A::ZERO;
        }
    }
    */
}