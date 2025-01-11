use crate::{
    device::ngr::renderer::platform::d3d::PlatformCmdBuffer,
    globals,
    graphics::draw2d::{ Draw, Im2DVertexG4 },
    utility::misc::RGBA
};
use glam::{ Vec2, Vec3 };
use std::time::{ Duration, Instant };

const LINE_GUIDES_COLOR: u32 = 0x606060ff;
const LOW_POINT_COLOR: u32 = 0xc0c000ff;
const MAX_FPS_LOW: u32 = 50;

#[derive(Debug)]
pub struct PerformanceMeter<'a> {
    bars: Vec<PerformanceBar<'a>>,
    start: Vec2,
    line_gap: f32,
    last_time: Instant
}

impl<'a> PerformanceMeter<'a> {
    pub fn draw(&self) {
        let mut pos = self.start;
        for bar in &self.bars {
            bar.draw_bar(pos);
            pos.y += self.line_gap;
        }
    }

    pub fn new(top_left: Vec2, bars: Vec<PerformanceBar<'a>>) -> Self {
        Self { bars, start: top_left, line_gap: 6., last_time: Instant::now() }
    }
    pub fn get_bar_count(&self) -> usize { self.bars.len() }
    pub fn tick_on_post_render(&mut self) {
        let duration = Instant::now().duration_since(self.last_time);
        for bar in &mut self.bars { bar.tick(duration) }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PerformanceBar<'a> {
    color: RGBA,
    height: f32,
    name: &'a str,

    fps: f32,
    fps_last: f32,
    fps_low: f32,
    fps_low_frames: u32,
}

impl<'a> PerformanceBar<'a> {
    const fn new(color: RGBA, name: &'a str) -> Self { Self::new_inner(color, 2., name) }
    const fn new_inner(color: RGBA, height: f32, name: &'a str) -> Self { Self { 
        color, height, name,
        fps: 0., fps_last: 0., fps_low: 0., fps_low_frames: 0
    } }
    pub fn draw_bar(&self, pos: Vec2) {
        let disp_fps = (self.fps + self.fps_last) / 2.;
        let window_width = 1920;
        let bar_width = window_width as f32 * 0.6;
        let cmd_buffer = unsafe { globals::get_ngr_dx11_renderer_unchecked_mut().get_command_buffer_unchecked_mut() };
        let mut drawer: Draw<0x117, Im2DVertexG4, PlatformCmdBuffer> = Draw::new(cmd_buffer);
        // long line
        drawer.draw_line_solid_color(
            RGBA::from_rgba_u32(LINE_GUIDES_COLOR),
            (pos, 0.).into(),
            Vec3::new(bar_width + pos.x, pos.y, 0.)
        );
        // stepped lines
        for i in 0..5 {
            drawer.draw_line_solid_color(
                RGBA::from_rgba_u32(LINE_GUIDES_COLOR), 
                Vec3::new(pos.x + (bar_width * i as f32 / 4.), pos.y - self.height, 0.), 
                Vec3::new(pos.x + (bar_width * i as f32 / 4.), pos.y + self.height, 0.));
        }
        // performance bar
        drawer.draw_rectangle_solid_color(
            self.color, pos.x + 1., pos.y - self.height, 
            (bar_width * 0.6 * 30.) / disp_fps, self.height * 2.
        );
        // low point 
        drawer.draw_rectangle_solid_color(
            RGBA::from_rgba_u32(LOW_POINT_COLOR),
            ((bar_width * 0.6 * 30.) / self.fps_low) - 2., pos.y - self.height, 
            4., self.height * 2.
        );
    }
    pub fn tick(&mut self, duration: Duration) { 
        let fps = (1000. / duration.as_millis() as f64) as f32;
        self.fps_last = self.fps;
        self.fps = fps;
        // replace fps if new fps is lower or time runs out
        if self.fps < self.fps_low || self.fps_low_frames > MAX_FPS_LOW {
            self.fps_low = if self.fps < self.fps_last { self.fps } else { self.fps_last };
            self.fps_low_frames = 0;
        }
        self.fps_low_frames += 1;
    }
}

pub static PERFORMANCE_METER: std::sync::Mutex<Option<PerformanceMeter<'static>>> = std::sync::Mutex::new(None);

pub fn draw_test() {
    let mut perf_meter = PERFORMANCE_METER.lock().unwrap();
    match (*perf_meter).as_mut() {
        Some(pm) => {
            pm.tick_on_post_render();
            pm.draw();
            pm.last_time = Instant::now();
        },
        None => {
            *perf_meter = Some(PerformanceMeter::new(
                Vec2::new(16., 26.),
                vec![
                    PerformanceBar::new(RGBA::from_rgba_u32(0x57c314ff), "FPS"),
                    PerformanceBar::new(RGBA::from_rgba_u32(0xbd114eff), "MAIN-CPU"),
                    PerformanceBar::new(RGBA::from_rgba_u32(0xc17a2cff), "RENDER-CPU"),
                    PerformanceBar::new(RGBA::from_rgba_u32(0x1050bfff), "GPU"),
                ]
            ));
        }
    };
}
