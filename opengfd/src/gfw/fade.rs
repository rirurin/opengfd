#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FadeState {
    Waiting = 0,
    Running
}
#[repr(C)]
#[derive(Debug)]
pub struct Fade {
    state: FadeState,
    current_time: f32,
    target_time: f32
}
