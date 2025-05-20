use bitflags::bitflags;
use std::{
    fmt::Display,
    mem::MaybeUninit,
    sync::Mutex
};
use windows::Win32::UI::Input::XboxController::{
    XInputGetState,
    XINPUT_STATE,
    XINPUT_GAMEPAD_DPAD_UP,
    XINPUT_GAMEPAD_DPAD_DOWN,
    XINPUT_GAMEPAD_DPAD_LEFT,
    XINPUT_GAMEPAD_DPAD_RIGHT,
    XINPUT_GAMEPAD_START,
    XINPUT_GAMEPAD_BACK,
    XINPUT_GAMEPAD_LEFT_THUMB,
    XINPUT_GAMEPAD_RIGHT_THUMB,
    XINPUT_GAMEPAD_LEFT_SHOULDER,
    XINPUT_GAMEPAD_RIGHT_SHOULDER,
    XINPUT_GAMEPAD_A,
    XINPUT_GAMEPAD_B,
    XINPUT_GAMEPAD_X,
    XINPUT_GAMEPAD_Y,
    XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE,
    XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE,
    XINPUT_GAMEPAD_BUTTON_FLAGS
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ControllerButton : u16 {
        const BACK = 1 << 0;
        const LEFT_THUMB = 1 << 1;
        const RIGHT_THUMB = 1 << 2;
        const START = 1 << 3;
        const DPAD_UP = 1 << 4;
        const DPAD_RIGHT = 1 << 5;
        const DPAD_DOWN = 1 << 6;
        const DPAD_LEFT = 1 << 7;
        const LEFT_TRIGGER = 1 << 8;
        const RIGHT_TRIGGER = 1 << 9;
        const LEFT_SHOULDER = 1 << 10;
        const RIGHT_SHOULDER = 1 << 11;
        const Y_BUTTON = 1 << 12;
        const B_BUTTON = 1 << 13;
        const A_BUTTON = 1 << 14;
        const X_BUTTON = 1 << 15;
    }
}

pub(crate) const CONTROLLER_STICK_ZERO_POINT: u16 = 0x80;

#[repr(align(2))]
#[derive(Debug, Clone)]
pub struct ControllerStickInput(u16);
impl Display for ControllerStickInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_val = self.0 as i16 - CONTROLLER_STICK_ZERO_POINT as i16;
        write!(f, "{}", fmt_val)
    }
}
impl Default for ControllerStickInput {
    fn default() -> Self {
        Self(CONTROLLER_STICK_ZERO_POINT)
    }
}
impl From<i16> for ControllerStickInput {
    fn from(value: i16) -> Self {
        Self((value as f32 / 256. + CONTROLLER_STICK_ZERO_POINT as f32) as u16)
    }
}


pub const MAX_CONTROLLER_COUNT: usize = 2;
pub const XINPUT_TRIGGER_ANALOG_THRESHOLD: u8 = 30;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ControllerPlatform {
    button: ControllerButton,
    rstick_hori: ControllerStickInput,
    rstick_vert: ControllerStickInput,
    lstick_hori: ControllerStickInput,
    lstick_vert: ControllerStickInput
}

impl Display for ControllerPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ControllerPlatform {{ button: {:?}, lstick: [{}, {}], rstick: [{}, {}] }} ",
        self.button, self.lstick_hori, self.lstick_vert, self.rstick_hori, self.rstick_vert)
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ControllerDetails {
    working_prev: [bool; MAX_CONTROLLER_COUNT],
    working: [bool; MAX_CONTROLLER_COUNT],
}

impl ControllerDetails {
    const fn default_const() -> Self {
        Self {
            working_prev: [false; MAX_CONTROLLER_COUNT],
            working: [false; MAX_CONTROLLER_COUNT],
        }
    }
}

impl Default for ControllerDetails {
    fn default() -> Self { Self::default_const() }
}

static CONTROLLER_DETAILS: Mutex<ControllerDetails> = Mutex::new(ControllerDetails::default_const());

impl ControllerPlatform {

    pub fn clear(&mut self) {
        self.button = ControllerButton::empty();
        self.rstick_vert = ControllerStickInput::default();
        self.rstick_hori = ControllerStickInput::default();
        self.lstick_vert = ControllerStickInput::default();
        self.lstick_hori = ControllerStickInput::default();
    }
    // XInput:
    // hori: [-127: left, 0: center, 127, right]
    // vert: [-127: down, 0: center, 127, up]
    // GFD:
    // hori: [-127: left, 0: center, 127, right]
    // vert: [-127: up, 0: center, 127, down]
    fn convert_stick_value(val: i16, deadzone: XINPUT_GAMEPAD_BUTTON_FLAGS) -> ControllerStickInput {
        let deadzone = deadzone.0 as i16;
        if val.abs() < deadzone { ControllerStickInput::default() }
        else { val.into() }
    }

    pub fn update(&mut self, index: u32) -> bool {
        let mut xinput: MaybeUninit<XINPUT_STATE> = MaybeUninit::uninit();
        let result = unsafe { XInputGetState(index, xinput.as_mut_ptr()) };
        let xinput = unsafe { xinput.assume_init() };
        let mut ctrl_lock = CONTROLLER_DETAILS.lock().unwrap();
        (*ctrl_lock).working_prev[index as usize] = (*ctrl_lock).working[index as usize];
        (*ctrl_lock).working[index as usize] = result == 0;
        self.clear();
        if result == 0 {
            // face buttons
            self.button.set(ControllerButton::BACK, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_BACK));
            self.button.set(ControllerButton::LEFT_THUMB, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_LEFT_THUMB));
            self.button.set(ControllerButton::RIGHT_THUMB, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_RIGHT_THUMB));
            self.button.set(ControllerButton::START, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_START));
            self.button.set(ControllerButton::DPAD_UP, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_UP));
            self.button.set(ControllerButton::DPAD_RIGHT, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_RIGHT));
            self.button.set(ControllerButton::DPAD_DOWN, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_DOWN));
            self.button.set(ControllerButton::DPAD_LEFT, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_DPAD_LEFT));
            self.button.set(ControllerButton::LEFT_TRIGGER, xinput.Gamepad.bLeftTrigger > XINPUT_TRIGGER_ANALOG_THRESHOLD);
            self.button.set(ControllerButton::RIGHT_TRIGGER, xinput.Gamepad.bRightTrigger > XINPUT_TRIGGER_ANALOG_THRESHOLD);
            self.button.set(ControllerButton::LEFT_SHOULDER, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_LEFT_SHOULDER));
            self.button.set(ControllerButton::RIGHT_SHOULDER, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_RIGHT_SHOULDER));
            self.button.set(ControllerButton::Y_BUTTON, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_Y));
            self.button.set(ControllerButton::B_BUTTON, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_B));
            self.button.set(ControllerButton::A_BUTTON, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_A));
            self.button.set(ControllerButton::X_BUTTON, xinput.Gamepad.wButtons.contains(XINPUT_GAMEPAD_X));
            // analog sticks
            self.lstick_hori = Self::convert_stick_value(xinput.Gamepad.sThumbLX, XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE);
            self.lstick_vert = Self::convert_stick_value(-xinput.Gamepad.sThumbLY, XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE);
            self.rstick_hori = Self::convert_stick_value(xinput.Gamepad.sThumbRX, XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE);
            self.rstick_vert = Self::convert_stick_value(-xinput.Gamepad.sThumbRY, XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE);
            true
        } else { false }
    }

    pub fn get_button(&self) -> ControllerButton {
        self.button
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ControllerStick {
    horizontal: u8,
    vertical: u8
}

const STICK_ZERO: u8 = 128;

impl ControllerStick {
    pub fn get_horizontal(&self) -> i8 {
        self.horizontal.overflowing_sub(STICK_ZERO).0 as i8
    }
    pub fn get_vertical(&self) -> i8 {
        self.vertical.overflowing_sub(STICK_ZERO).0 as i8
    }
}

impl Display for ControllerStick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stick <{}, {}>", self.get_horizontal(), self.get_vertical())
    }
}

const CONTROLLER_COUNT: usize = 2;

#[repr(C)]
#[derive(Debug)]
pub struct ControllerPad {
    start_press: [ControllerButton; CONTROLLER_COUNT],
    end_press: [ControllerButton; CONTROLLER_COUNT],
    hold_press: [ControllerButton; CONTROLLER_COUNT],
    rstick: [ControllerStick; CONTROLLER_COUNT],
    lstick: [ControllerStick; CONTROLLER_COUNT],
}

impl ControllerPad {
    pub fn get_start_press(&self, id: usize) -> ControllerButton {
        self.start_press[id]
    }
    pub fn get_end_press(&self, id: usize) -> ControllerButton {
        self.end_press[id]
    }
    pub fn get_hold_press(&self, id: usize) -> ControllerButton {
        self.hold_press[id]
    }
    pub fn get_lstick(&self, id: usize) -> ControllerStick {
        self.lstick[id]
    }
    pub fn get_rstick(&self, id: usize) -> ControllerStick {
        self.rstick[id]
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Controller {
    current: ControllerPad,
    default: ControllerPad,
    delta1: f32,
    delta2: f32,
    control_init: bool,
    field31: u8,
    is_matching_control: bool,
    control_id: u32,
    field38: [u32; 2],
    pad3: ControllerPad
}

impl Controller {
    pub fn get_current(&self) -> &ControllerPad {
        &self.current
    }
    pub fn get_control_id(&self) -> usize {
        self.control_id as usize
    }
}