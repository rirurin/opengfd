use windows::{
    core::PCSTR,
    Win32::System::LibraryLoader
};
const STEAMWORKS_SDK_DLL: &'static str = "steam_api64.dll\0";

pub struct PlatformInfo;
impl PlatformInfo {
    pub fn is_steam() -> bool {
        unsafe { LibraryLoader::GetModuleHandleA(PCSTR(STEAMWORKS_SDK_DLL.as_ptr())).is_ok() }
    }
}