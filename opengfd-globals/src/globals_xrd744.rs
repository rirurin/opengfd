use opengfd::{
    device::hedge::hid::keyboard::KeyboardManager,
    io::controller::ControllerPlatformManager,
    kernel::global::Global,
    platform::global::PlatformGlobal,
};
use opengfd_proc::create_gfd_static;

// static items
create_gfd_static!(GFD_GLOBAL, Global);
create_gfd_static!(PLATFORM_GLOBAL, PlatformGlobal);
// io
create_gfd_static!(KEYBOARD_INSTANCE, *mut KeyboardManager);
create_gfd_static!(PAD_INSTANCE, *mut ControllerPlatformManager);
// platform
create_gfd_static!(IS_STEAM, bool);