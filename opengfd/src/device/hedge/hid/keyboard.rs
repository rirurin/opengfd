use cpp_types::msvc::vector::Vector as CppVector;
use std::ptr::NonNull;
use allocator_api2::alloc::Global;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::{HMODULE, HWND};
use crate::device::hedge::{
    fnd::base_object::ReferencedObject,
    ut::array::{Array, InplaceArray}
};

#[repr(C)]
pub struct InputDevice {
    _super: ReferencedObject
}

#[repr(C)]
pub struct Keyboard {
    _super: InputDevice,
    unk1: u8,
    keyboard_active: bool,
    active_inputs: InplaceArray<u32, 16>,
    field78: usize,
    unk4: InplaceArray<u32, 4>,
    fielda8: usize,
    keys_pressed: [u8; 0x100]
}

#[repr(C)]
#[cfg(target_os = "windows")]
pub struct KeyboardWin32 {
    _super: Keyboard,
    executable_handle: HMODULE,
    field1b8: usize,
    window_handle: HWND,
    field1c8: [u8; 1024],
    dik_to_keys: [u32; 256],
    unk106: Array<NonNull<u8>, u32>,
    allocator: NonNull<u8>,
    field9e8: [u32; 5]
}

#[cfg(target_os = "windows")]
impl KeyboardWin32 {
    pub fn check_key(&self, button: DirectInputKey) -> u8 {
        self._super.keys_pressed[(self.dik_to_keys[button as usize] & u16::MAX as u32) as usize]
    }
}

#[repr(C)]
#[cfg(target_os = "windows")]
pub struct KeyboardManager {
    dinput8_handle: usize,
    device_notify_handle: usize,
    field10: CppVector<u8, Global>,
    field28: CppVector<u8, Global>,
    field40: CppVector<u8, Global>,
    field58: u32,
    keyboard: NonNull<KeyboardWin32>,
    field68: u16,
    field6c: u32,
    field70: u8
}



#[cfg(target_os = "windows")]
impl KeyboardManager {
    pub fn get_keyboard(&self) -> &KeyboardWin32 {
        unsafe { self.keyboard.as_ref() }
    }
    pub fn get_keyboard_raw(&self) -> *mut *mut KeyboardWin32 {
        (&raw const self.keyboard) as *mut *mut KeyboardWin32
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum DirectInputKey {
    Sleep 	= 223,
    Stop 	= 149,
    Convert 	= 121,
    Decimal 	= 83,
    X 	= 45,
    Y 	= 21,
    Escape 	= 1,
    Circumflex 	= 144,
    PageDown 	= 209,
    DownArrow 	= 208,
    RightArrow 	= 205,
    LeftArrow 	= 203,
    PageUp 	= 201,
    UpArrow 	= 200,
    RightAlt 	= 184,
    CapsLock 	= 58,
    LeftAlt 	= 56,
    BackSpace 	= 14,
    MediaSelect 	= 237,
    Mail 	= 236,
    MyComputer 	= 235,
    WebBack 	= 234,
    WebForward 	= 233,
    WebStop 	= 232,
    WebRefresh 	= 231,
    WebFavorites 	= 230,
    WebSearch 	= 229,
    Wake 	= 227,
    Power 	= 222,
    Apps 	= 221,
    RightWindows 	= 220,
    LeftWindows 	= 219,
    End 	= 207,
    Home 	= 199,
    SysRq 	= 183,
    Divide 	= 181,
    NumPadComma 	= 179,
    WebHome 	= 178,
    VolumeUp 	= 176,
    VolumeDown 	= 174,
    MediaStop 	= 164,
    PlayPause 	= 162,
    Calculator 	= 161,
    Mute 	= 160,
    RightControl 	= 157,
    NumPadEnter 	= 156,
    NextTrack 	= 153,
    Unlabeled 	= 151,
    AX 	= 150,
    Kanji 	= 148,
    Underline 	= 147,
    Colon 	= 146,
    At 	= 145,
    NumPadEquals 	= 141,
    AbntC2 	= 126,
    Yen 	= 125,
    NoConvert 	= 123,
    AbntC1 	= 115,
    Kana 	= 112,
    F15 	= 102,
    F14 	= 101,
    F13 	= 100,
    F12 	= 88,
    F11 	= 87,
    OEM102 	= 86,
    NumPad0 	= 82,
    NumPad3 	= 81,
    NumPad2 	= 80,
    NumPad1 	= 79,
    NumPad6 	= 77,
    NumPad5 	= 76,
    NumPad4 	= 75,
    Subtract 	= 74,
    NumPad9 	= 73,
    NumPad8 	= 72,
    NumPad7 	= 71,
    Scroll 	= 70,
    Numlock 	= 69,
    F10 	= 68,
    F9 	= 67,
    F8 	= 66,
    F7 	= 65,
    F6 	= 64,
    F5 	= 63,
    F4 	= 62,
    F3 	= 61,
    F2 	= 60,
    F1 	= 59,
    Space 	= 57,
    Multiply 	= 55,
    RightShift 	= 54,
    Slash 	= 53,
    Period 	= 52,
    Comma 	= 51,
    M 	= 50,
    N 	= 49,
    B 	= 48,
    V 	= 47,
    C 	= 46,
    Z 	= 44,
    BackSlash 	= 43,
    LeftShift 	= 42,
    Grave 	= 41,
    Apostrophe 	= 40,
    SemiColon 	= 39,
    L 	= 38,
    K 	= 37,
    J 	= 36,
    H 	= 35,
    G 	= 34,
    F 	= 33,
    D 	= 32,
    S 	= 31,
    A 	= 30,
    LeftControl 	= 29,
    Return 	= 28,
    RightBracket 	= 27,
    LeftBracket 	= 26,
    P 	= 25,
    O 	= 24,
    I 	= 23,
    U 	= 22,
    T 	= 20,
    R 	= 19,
    E 	= 18,
    W 	= 17,
    Tab 	= 15,
    Equals 	= 13, 	// Returns a value indicating whether this instance is equal to a specified object.
    Minus 	= 12,
    D0 	= 11,
    D9 	= 10,
    D8 	= 9,
    D7 	= 8,
    D6 	= 7,
    D5 	= 6,
    D4 	= 5,
    D3 	= 4,
    D2 	= 3,
    D1 	= 2,
    Insert 	= 210, 	// Inserts an element into the collection at the specified index.
    Pause 	= 197,
    Add 	= 78,
    Delete 	= 211,
    Q 	= 16,
}