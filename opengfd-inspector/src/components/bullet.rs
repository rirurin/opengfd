use imgui::{ Direction, Ui };
use glam::Vec2;

pub fn bullet_ex(ui: &mut Ui, depth: usize) {
    let mut pos = Vec2::from(ui.cursor_screen_pos());
    pos += Vec2::new(
        unsafe { ui.style().frame_padding[0] } + ui.current_font_size() * 0.5,
            (ui.current_font_size() + unsafe { ui.style().frame_padding[1] }) / 2.
    );
    let pos: [f32; 2] = pos.into();
    let col = unsafe { ui.style().colors[imgui::StyleColor::Text as usize] };

    let draw_list = ui.get_window_draw_list();
    draw_list.add_circle(pos, ui.current_font_size() * 0.2, col).filled(true).build();
    unsafe { ui.same_line_with_spacing(0., ui.current_font_size() + ui.style().frame_padding[0] * 2. + 10. * depth as f32) }
}

pub fn arrow_ex(ui: &mut Ui, dir: Direction, depth: usize) {
    let mut pos = Vec2::from(ui.cursor_screen_pos());
    pos += Vec2::new(
        unsafe { ui.style().frame_padding[0] } + ui.current_font_size() * 0.5,
            (ui.current_font_size() + unsafe { ui.style().frame_padding[1] }) / 2.
    );
    let col = unsafe { ui.style().colors[imgui::StyleColor::Text as usize] };

    let draw_list = ui.get_window_draw_list();
    let r = ui.current_font_size() * 0.25;
    let (a, b, c) = match dir {
        Direction::Up | Direction::Down => {
            let r = if dir == Direction::Down { r } else { -r };
            (
                Vec2::new( 0.000f32 * r,      0.750f32 * r),
                Vec2::new(-0.866f32 * r,     -0.750f32 * r),
                Vec2::new( 0.866f32 * r,     -0.750f32 * r),
            )
        },
        Direction::Left | Direction::Right => {
            let r = if dir == Direction::Right { r } else { -r };
            (
                Vec2::new( 0.750f32 * r,      0.000f32 * r),
                Vec2::new(-0.750f32 * r,      0.866f32 * r),
                Vec2::new(-0.750f32 * r,     -0.866f32 * r),
            )
        },
        _ => ( Vec2::ZERO, Vec2::ZERO, Vec2::ZERO )
    };
    let a: [f32; 2] = (pos + a).into();
    let b: [f32; 2] = (pos + b).into();
    let c: [f32; 2] = (pos + c).into();
    draw_list.add_triangle(a, b, c, col).filled(true).build();
    unsafe { ui.same_line_with_spacing(0., ui.current_font_size() + ui.style().frame_padding[0] * 2. + 10. * depth as f32) }
}