use riri_mod_tools_proc::riri_init_fn;
use opengfd::platform::utils::PlatformInfo;

#[riri_init_fn()]
fn init_set_steam_check() {
    PlatformInfo::set_platform_steam();
}