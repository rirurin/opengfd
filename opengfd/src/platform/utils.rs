use std::sync::OnceLock;
use riri_mod_tools_rt::logln;
use windows::{
    core::PCSTR,
    Win32::System::LibraryLoader
};
const STEAMWORKS_SDK_DLL: &'static str = "steam_api64.dll\0";
fn steamworks_name() -> &'static str {
    &STEAMWORKS_SDK_DLL[..STEAMWORKS_SDK_DLL.len()-1]
}

pub static IS_PLATFORM_STEAM: OnceLock<bool> = OnceLock::new();

pub struct PlatformInfo;
impl PlatformInfo {
    pub fn is_steam() -> bool {
        *IS_PLATFORM_STEAM.get().unwrap() 
    }

    pub fn set_platform_steam() {
        let is_steam = unsafe { LibraryLoader::GetModuleHandleA(PCSTR(STEAMWORKS_SDK_DLL.as_ptr())).is_ok() };
        if is_steam {
            logln!(Information, "{} found, set platform type to Steam", steamworks_name());
        } else {
            logln!(Information, "{} not found, assuming platform type is Gamepass", steamworks_name());
        }
        
        let _ = IS_PLATFORM_STEAM.set(is_steam);
    }
}