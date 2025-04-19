use bitflags::bitflags;
use crate::graphics::texture::Texture;
use std::{
    ptr::NonNull,
    sync::Mutex
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MouseButton: u16 {
        const LBUTTON = 1 << 0;
        const RBUTTON = 1 << 1;
        const MBUTTON = 1 << 2;
        const XBUTTON0 = 1 << 3;
        const XBUTTON1 = 1 << 4;
    }
}

pub static BLOCK_MOUSE_UPDATE: Mutex<bool> = Mutex::new(false);

#[repr(C)]
#[derive(Debug)]
pub struct WindowMouseState {
    button: MouseButton,
    pos: [u16; 2],
    scroll: [u16; 2],
    fielda: [u8; 0x16]
}

impl WindowMouseState {
    pub fn update_from(&mut self, other: &WindowMouseState) -> bool {
        let block_lock = BLOCK_MOUSE_UPDATE.lock().unwrap(); 
        if !*block_lock {
            self.button = other.button;
        }
        self.pos = other.pos;
        self.scroll = other.scroll;
        self.fielda = other.fielda;
        true
    }
}

#[repr(C)]
pub struct MouseState {
    pos: [u32; 2],
    field8: [u32; 4],
    delta: [u32; 2],
    button_start: u32,
    field24: u32,
    button_hold: u32,
    field2c: u32,
    field30: [u8; 2],
    has_delta: bool,
    on_screen: bool,
    field38: bool,
    accepts_mouse: bool,
    set_cursor_pos: bool,
    field3c: f32,
    button_hold_delta: f32,
    button_hold_short: u32,
    drag_delta: [u32; 2],
    cursor_texture: NonNull<Texture>,
    field58: [u32; 2],
    cursor: [u32; 2],
    field68: [u32; 2]
}