use bitflags::bitflags;
use std::{
    // cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{ Mutex, OnceLock }
};
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyboardState,
    VK_LCONTROL, VK_RCONTROL,
    VK_LSHIFT, VK_RSHIFT,
    VK_LMENU, VK_RMENU,
    VK_LWIN, VK_RWIN,
    VK_UP, VK_DOWN, VK_LEFT, VK_RIGHT,
    VK_BACK, VK_DELETE, VK_SPACE, 
    VK_RETURN, VK_ESCAPE, 
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct KeyboardCombo : u32 {
        const HOLD_CTRL = 1 << 0; // VK_LCONTROL | VK_RCONTROL
        const HOLD_SHIFT = 1 << 1; // VK_LSHIFT | VK_RSHIFT
        const HOLD_ALT = 1 << 2; // VK_LMENU | VK_RMENU
        const HOLD_SUPER = 1 << 3; // VK_LWIN | VK_RWIN
        const HOLD_UP_ARROW = 1 << 4;
        const HOLD_DOWN_ARROW = 1 << 5;
        const HOLD_LEFT_ARROW = 1 << 6;
        const HOLD_RIGHT_ARROW = 1 << 7;
        const HOLD_BACKSPACE = 1 << 8;
        const HOLD_DELETE = 1 << 9;
        const HOLD_ENTER = 1 << 10;
        const HOLD_ESCAPE = 1 << 11;
        const HOLD_SPACEBAR = 1 << 12;
    }
}

const MAX_KEY_COUNT: usize = 6;
static GFD_KEYBOARD_STATE: Mutex<Keyboard> = Mutex::new(Keyboard::new());
// pub static BLOCK_KEYBOARD_UPDATE: Mutex<bool> = Mutex::new(false);
pub static BLOCK_KEYBOARD_UPDATE: OnceLock<bool> = OnceLock::new();

#[repr(C)]
#[derive(Debug)]
pub struct Keyboard {
    key_combo: KeyboardCombo,
    key_count: u32,
    key_ids: [u8; MAX_KEY_COUNT],
    mouse_key: u16
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            key_combo: KeyboardCombo::empty(),
            key_count: 0,
            key_ids: [0; MAX_KEY_COUNT],
            mouse_key: 0
        }
    }
    pub fn get_keys(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.key_ids.as_ptr(), self.key_count as usize) }
    }
    pub fn update_global() -> bool {
        let mut keyboard_lock = GFD_KEYBOARD_STATE.lock().unwrap();
        (*keyboard_lock).update()
    }

    pub fn clear(&mut self) {
        self.key_combo = KeyboardCombo::empty();
        self.key_count = 0;
        self.key_ids = [0; MAX_KEY_COUNT];
        self.mouse_key = 0;
    }

    pub fn get_key_count(&self) -> usize { self.key_count as usize }
    pub fn check_key_combo(&self, combo: KeyboardCombo) -> bool {
        self.key_combo.contains(combo)
    }

    #[cfg(target_os = "windows")]
    fn is_key_down(key: u8) -> bool { key & 0x80 != 0 }
    #[cfg(target_os = "windows")]
    #[allow(dead_code)]
    fn is_key_toggled(key: u8) -> bool { key & 0x1 != 0 }

    // 0x1410ba940 (from Metaphor Prologue Demo Steam)
    /// Original function: gfdDeviceKeyboardGetData
    #[cfg(target_os = "windows")]
    pub fn update(&mut self) -> bool {
        let mut key_state: MaybeUninit<[u8; 256]> = MaybeUninit::uninit();
        unsafe { GetKeyboardState(key_state.assume_init_mut()).unwrap() }
        self.clear();
        // let block_lock = BLOCK_KEYBOARD_UPDATE.lock().unwrap();
        // if *block_lock { return false; }
        if unsafe { *crate::globals::get_block_keyboard_focus_unchecked() } { return false; }
        // set keyboard modifiers
        let key_state = unsafe { key_state.assume_init() };
        if Self::is_key_down(key_state[VK_LCONTROL.0 as usize])
        || Self::is_key_down(key_state[VK_RCONTROL.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_CTRL;
        }
        if Self::is_key_down(key_state[VK_LSHIFT.0 as usize])
        || Self::is_key_down(key_state[VK_RSHIFT.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_SHIFT;
        }
        if Self::is_key_down(key_state[VK_LMENU.0 as usize])
        || Self::is_key_down(key_state[VK_RMENU.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_ALT;
        }
        if Self::is_key_down(key_state[VK_LWIN.0 as usize])
        || Self::is_key_down(key_state[VK_RWIN.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_SUPER;
        }
        if Self::is_key_down(key_state[VK_UP.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_UP_ARROW;
        }
        if Self::is_key_down(key_state[VK_DOWN.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_DOWN_ARROW;
        }
        if Self::is_key_down(key_state[VK_LEFT.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_LEFT_ARROW;
        }
        if Self::is_key_down(key_state[VK_RIGHT.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_RIGHT_ARROW;
        }
        if Self::is_key_down(key_state[VK_BACK.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_BACKSPACE;
        }
        if Self::is_key_down(key_state[VK_DELETE.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_DELETE;
        }
        if Self::is_key_down(key_state[VK_RETURN.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_ENTER;
        }
        if Self::is_key_down(key_state[VK_ESCAPE.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_ESCAPE;
        }
        if Self::is_key_down(key_state[VK_SPACE.0 as usize]) {
            self.key_combo |= KeyboardCombo::HOLD_SPACEBAR;
        }
        let mouse_keycode = unsafe { crate::globals::get_keycode_for_mouse_click_unchecked_mut() };
        if !Self::is_key_down(key_state[VK_LWIN.0 as usize])
        && !Self::is_key_down(key_state[VK_RWIN.0 as usize]) {
            if *mouse_keycode != 0 {
                self.mouse_key = *mouse_keycode as u16;
            }
            *mouse_keycode = 0;
            for i in 8..0xf0 {
                if Self::is_key_down(key_state[i]) {
                    self.key_ids[self.key_count as usize] = i as u8;
                    self.key_count += 1;
                    if self.key_count >= 6 { break; }
                }
            }
            true
        } else { 
            *mouse_keycode = 0;
            false 
        }
    }
}