#[repr(C)]
#[derive(Debug)]
pub struct QuakeParams {
    power: f32,
    pitch_weight: f32,
    total_time: f32,
    fade_in_time: f32,
    fade_out_time: f32,
}
