#![allow(dead_code, improper_ctypes)]
// This file was automatically generated from cri-adx-globals.
#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEX3DSOURCE_SETPOSITION.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomex3dsource_setposition(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEX3DSOURCE_SETPOSITION. This checks to see if `set_criatomex3dsource_setposition`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomex3dsource_setposition() -> Option<& 'static u8>;
   /// Like `get_criatomex3dsource_setposition_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomex3dsource_setposition_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomex3dsource_setposition`. This assumes that CRIATOMEX3DSOURCE_SETPOSITION
    /// is set and it's initialized.
    pub(crate) fn get_criatomex3dsource_setposition_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomex3dsource_setposition_mut`. This assumes that CRIATOMEX3DSOURCE_SETPOSITION
    /// is set and it's initialized.
    pub(crate) fn get_criatomex3dsource_setposition_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEX3DSOURCE_SETPOSITION`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomex3dsource_setposition(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXACB_GETCUEINFOBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexacb_getcueinfobyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXACB_GETCUEINFOBYID. This checks to see if `set_criatomexacb_getcueinfobyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexacb_getcueinfobyid() -> Option<& 'static u8>;
   /// Like `get_criatomexacb_getcueinfobyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexacb_getcueinfobyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexacb_getcueinfobyid`. This assumes that CRIATOMEXACB_GETCUEINFOBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_getcueinfobyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexacb_getcueinfobyid_mut`. This assumes that CRIATOMEXACB_GETCUEINFOBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_getcueinfobyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXACB_GETCUEINFOBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexacb_getcueinfobyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXACB_GETCUEINFOBYNAME.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexacb_getcueinfobyname(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXACB_GETCUEINFOBYNAME. This checks to see if `set_criatomexacb_getcueinfobyname`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexacb_getcueinfobyname() -> Option<& 'static u8>;
   /// Like `get_criatomexacb_getcueinfobyname_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexacb_getcueinfobyname_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexacb_getcueinfobyname`. This assumes that CRIATOMEXACB_GETCUEINFOBYNAME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_getcueinfobyname_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexacb_getcueinfobyname_mut`. This assumes that CRIATOMEXACB_GETCUEINFOBYNAME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_getcueinfobyname_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXACB_GETCUEINFOBYNAME`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexacb_getcueinfobyname(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXACB_LOADACBDATA.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexacb_loadacbdata(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXACB_LOADACBDATA. This checks to see if `set_criatomexacb_loadacbdata`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexacb_loadacbdata() -> Option<& 'static u8>;
   /// Like `get_criatomexacb_loadacbdata_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexacb_loadacbdata_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexacb_loadacbdata`. This assumes that CRIATOMEXACB_LOADACBDATA
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_loadacbdata_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexacb_loadacbdata_mut`. This assumes that CRIATOMEXACB_LOADACBDATA
    /// is set and it's initialized.
    pub(crate) fn get_criatomexacb_loadacbdata_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXACB_LOADACBDATA`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexacb_loadacbdata(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMAWB_LOADTOC.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomawb_loadtoc(ptr: *mut u8);
   /// Get a possible reference to CRIATOMAWB_LOADTOC. This checks to see if `set_criatomawb_loadtoc`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomawb_loadtoc() -> Option<& 'static u8>;
   /// Like `get_criatomawb_loadtoc_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomawb_loadtoc_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomawb_loadtoc`. This assumes that CRIATOMAWB_LOADTOC
    /// is set and it's initialized.
    pub(crate) fn get_criatomawb_loadtoc_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomawb_loadtoc_mut`. This assumes that CRIATOMAWB_LOADTOC
    /// is set and it's initialized.
    pub(crate) fn get_criatomawb_loadtoc_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMAWB_LOADTOC`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomawb_loadtoc(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXCATEGORY_GETVOLUMEBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexcategory_getvolumebyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXCATEGORY_GETVOLUMEBYID. This checks to see if `set_criatomexcategory_getvolumebyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexcategory_getvolumebyid() -> Option<& 'static u8>;
   /// Like `get_criatomexcategory_getvolumebyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexcategory_getvolumebyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexcategory_getvolumebyid`. This assumes that CRIATOMEXCATEGORY_GETVOLUMEBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexcategory_getvolumebyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexcategory_getvolumebyid_mut`. This assumes that CRIATOMEXCATEGORY_GETVOLUMEBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexcategory_getvolumebyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXCATEGORY_GETVOLUMEBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexcategory_getvolumebyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXCATEGORY_SETVOLUMEBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexcategory_setvolumebyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXCATEGORY_SETVOLUMEBYID. This checks to see if `set_criatomexcategory_setvolumebyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexcategory_setvolumebyid() -> Option<& 'static u8>;
   /// Like `get_criatomexcategory_setvolumebyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexcategory_setvolumebyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexcategory_setvolumebyid`. This assumes that CRIATOMEXCATEGORY_SETVOLUMEBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexcategory_setvolumebyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexcategory_setvolumebyid_mut`. This assumes that CRIATOMEXCATEGORY_SETVOLUMEBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexcategory_setvolumebyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXCATEGORY_SETVOLUMEBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexcategory_setvolumebyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMCONFIG_GETCATEGORYINDEXBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomconfig_getcategoryindexbyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMCONFIG_GETCATEGORYINDEXBYID. This checks to see if `set_criatomconfig_getcategoryindexbyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomconfig_getcategoryindexbyid() -> Option<& 'static u8>;
   /// Like `get_criatomconfig_getcategoryindexbyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomconfig_getcategoryindexbyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomconfig_getcategoryindexbyid`. This assumes that CRIATOMCONFIG_GETCATEGORYINDEXBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomconfig_getcategoryindexbyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomconfig_getcategoryindexbyid_mut`. This assumes that CRIATOMCONFIG_GETCATEGORYINDEXBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomconfig_getcategoryindexbyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMCONFIG_GETCATEGORYINDEXBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomconfig_getcategoryindexbyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_CREATE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_create(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_CREATE. This checks to see if `set_criatomexplayer_create`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_create() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_create_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_create_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_create`. This assumes that CRIATOMEXPLAYER_CREATE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_create_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_create_mut`. This assumes that CRIATOMEXPLAYER_CREATE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_create_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_CREATE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_create(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_GETSTATUS.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_getstatus(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_GETSTATUS. This checks to see if `set_criatomexplayer_getstatus`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_getstatus() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_getstatus_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_getstatus_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_getstatus`. This assumes that CRIATOMEXPLAYER_GETSTATUS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_getstatus_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_getstatus_mut`. This assumes that CRIATOMEXPLAYER_GETSTATUS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_getstatus_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_GETSTATUS`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_getstatus(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_ISPAUSED.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_ispaused(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_ISPAUSED. This checks to see if `set_criatomexplayer_ispaused`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_ispaused() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_ispaused_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_ispaused_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_ispaused`. This assumes that CRIATOMEXPLAYER_ISPAUSED
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_ispaused_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_ispaused_mut`. This assumes that CRIATOMEXPLAYER_ISPAUSED
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_ispaused_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_ISPAUSED`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_ispaused(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_PAUSE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_pause(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_PAUSE. This checks to see if `set_criatomexplayer_pause`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_pause() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_pause_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_pause_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_pause`. This assumes that CRIATOMEXPLAYER_PAUSE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_pause_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_pause_mut`. This assumes that CRIATOMEXPLAYER_PAUSE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_pause_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_PAUSE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_pause(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_RESETPARAMETERS.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_resetparameters(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_RESETPARAMETERS. This checks to see if `set_criatomexplayer_resetparameters`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_resetparameters() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_resetparameters_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_resetparameters_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_resetparameters`. This assumes that CRIATOMEXPLAYER_RESETPARAMETERS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_resetparameters_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_resetparameters_mut`. This assumes that CRIATOMEXPLAYER_RESETPARAMETERS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_resetparameters_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_RESETPARAMETERS`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_resetparameters(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETAISACCONTROLBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setaisaccontrolbyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETAISACCONTROLBYID. This checks to see if `set_criatomexplayer_setaisaccontrolbyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyid() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setaisaccontrolbyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setaisaccontrolbyid`. This assumes that CRIATOMEXPLAYER_SETAISACCONTROLBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setaisaccontrolbyid_mut`. This assumes that CRIATOMEXPLAYER_SETAISACCONTROLBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETAISACCONTROLBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setaisaccontrolbyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETAISACCONTROLBYNAME.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setaisaccontrolbyname(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETAISACCONTROLBYNAME. This checks to see if `set_criatomexplayer_setaisaccontrolbyname`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyname() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setaisaccontrolbyname_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyname_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setaisaccontrolbyname`. This assumes that CRIATOMEXPLAYER_SETAISACCONTROLBYNAME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyname_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setaisaccontrolbyname_mut`. This assumes that CRIATOMEXPLAYER_SETAISACCONTROLBYNAME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setaisaccontrolbyname_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETAISACCONTROLBYNAME`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setaisaccontrolbyname(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETCATEGORYBYID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setcategorybyid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETCATEGORYBYID. This checks to see if `set_criatomexplayer_setcategorybyid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setcategorybyid() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setcategorybyid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setcategorybyid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setcategorybyid`. This assumes that CRIATOMEXPLAYER_SETCATEGORYBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setcategorybyid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setcategorybyid_mut`. This assumes that CRIATOMEXPLAYER_SETCATEGORYBYID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setcategorybyid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETCATEGORYBYID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setcategorybyid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETCUEID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setcueid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETCUEID. This checks to see if `set_criatomexplayer_setcueid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setcueid() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setcueid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setcueid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setcueid`. This assumes that CRIATOMEXPLAYER_SETCUEID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setcueid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setcueid_mut`. This assumes that CRIATOMEXPLAYER_SETCUEID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setcueid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETCUEID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setcueid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETDATA.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setdata(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETDATA. This checks to see if `set_criatomexplayer_setdata`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setdata() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setdata_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setdata_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setdata`. This assumes that CRIATOMEXPLAYER_SETDATA
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setdata_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setdata_mut`. This assumes that CRIATOMEXPLAYER_SETDATA
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setdata_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETDATA`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setdata(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETFILE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setfile(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETFILE. This checks to see if `set_criatomexplayer_setfile`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setfile() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setfile_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setfile_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setfile`. This assumes that CRIATOMEXPLAYER_SETFILE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setfile_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setfile_mut`. This assumes that CRIATOMEXPLAYER_SETFILE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setfile_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETFILE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setfile(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETFORMAT.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setformat(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETFORMAT. This checks to see if `set_criatomexplayer_setformat`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setformat() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setformat_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setformat_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setformat`. This assumes that CRIATOMEXPLAYER_SETFORMAT
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setformat_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setformat_mut`. This assumes that CRIATOMEXPLAYER_SETFORMAT
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setformat_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETFORMAT`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setformat(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETNUMCHANNELS.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setnumchannels(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETNUMCHANNELS. This checks to see if `set_criatomexplayer_setnumchannels`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setnumchannels() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setnumchannels_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setnumchannels_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setnumchannels`. This assumes that CRIATOMEXPLAYER_SETNUMCHANNELS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setnumchannels_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setnumchannels_mut`. This assumes that CRIATOMEXPLAYER_SETNUMCHANNELS
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setnumchannels_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETNUMCHANNELS`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setnumchannels(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETPANTYPE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setpantype(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETPANTYPE. This checks to see if `set_criatomexplayer_setpantype`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setpantype() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setpantype_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setpantype_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setpantype`. This assumes that CRIATOMEXPLAYER_SETPANTYPE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setpantype_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setpantype_mut`. This assumes that CRIATOMEXPLAYER_SETPANTYPE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setpantype_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETPANTYPE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setpantype(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETSAMPLINGRATE.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setsamplingrate(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETSAMPLINGRATE. This checks to see if `set_criatomexplayer_setsamplingrate`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setsamplingrate() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setsamplingrate_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setsamplingrate_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setsamplingrate`. This assumes that CRIATOMEXPLAYER_SETSAMPLINGRATE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setsamplingrate_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setsamplingrate_mut`. This assumes that CRIATOMEXPLAYER_SETSAMPLINGRATE
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setsamplingrate_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETSAMPLINGRATE`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setsamplingrate(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETVOLUME.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setvolume(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETVOLUME. This checks to see if `set_criatomexplayer_setvolume`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setvolume() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setvolume_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setvolume_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setvolume`. This assumes that CRIATOMEXPLAYER_SETVOLUME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setvolume_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setvolume_mut`. This assumes that CRIATOMEXPLAYER_SETVOLUME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setvolume_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETVOLUME`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setvolume(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_SETWAVEID.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_setwaveid(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_SETWAVEID. This checks to see if `set_criatomexplayer_setwaveid`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_setwaveid() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_setwaveid_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_setwaveid_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_setwaveid`. This assumes that CRIATOMEXPLAYER_SETWAVEID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setwaveid_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_setwaveid_mut`. This assumes that CRIATOMEXPLAYER_SETWAVEID
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_setwaveid_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_SETWAVEID`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_setwaveid(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_START.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_start(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_START. This checks to see if `set_criatomexplayer_start`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_start() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_start_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_start_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_start`. This assumes that CRIATOMEXPLAYER_START
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_start_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_start_mut`. This assumes that CRIATOMEXPLAYER_START
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_start_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_START`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_start(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_STOP.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_stop(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_STOP. This checks to see if `set_criatomexplayer_stop`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_stop() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_stop_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_stop_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_stop`. This assumes that CRIATOMEXPLAYER_STOP
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_stop_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_stop_mut`. This assumes that CRIATOMEXPLAYER_STOP
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_stop_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_STOP`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_stop(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXPLAYER_STOPWITHOUTRELEASETIME.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomexplayer_stopwithoutreleasetime(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXPLAYER_STOPWITHOUTRELEASETIME. This checks to see if `set_criatomexplayer_stopwithoutreleasetime`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomexplayer_stopwithoutreleasetime() -> Option<& 'static u8>;
   /// Like `get_criatomexplayer_stopwithoutreleasetime_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomexplayer_stopwithoutreleasetime_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomexplayer_stopwithoutreleasetime`. This assumes that CRIATOMEXPLAYER_STOPWITHOUTRELEASETIME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_stopwithoutreleasetime_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomexplayer_stopwithoutreleasetime_mut`. This assumes that CRIATOMEXPLAYER_STOPWITHOUTRELEASETIME
    /// is set and it's initialized.
    pub(crate) fn get_criatomexplayer_stopwithoutreleasetime_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXPLAYER_STOPWITHOUTRELEASETIME`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomexplayer_stopwithoutreleasetime(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXTWEEN_RESET.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomextween_reset(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXTWEEN_RESET. This checks to see if `set_criatomextween_reset`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomextween_reset() -> Option<& 'static u8>;
   /// Like `get_criatomextween_reset_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomextween_reset_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomextween_reset`. This assumes that CRIATOMEXTWEEN_RESET
    /// is set and it's initialized.
    pub(crate) fn get_criatomextween_reset_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomextween_reset_mut`. This assumes that CRIATOMEXTWEEN_RESET
    /// is set and it's initialized.
    pub(crate) fn get_criatomextween_reset_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXTWEEN_RESET`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomextween_reset(new: u8);
}

#[link(name = "cri_adx_globals", kind = "raw-dylib")]
unsafe extern "C" {
   /// Set the pointer to the memory location containing the beginning of CRIATOMEXTWEEN_STOP.
    /// This method must only be called once, otherwise it will panic.
    pub(crate) fn set_criatomextween_stop(ptr: *mut u8);
   /// Get a possible reference to CRIATOMEXTWEEN_STOP. This checks to see if `set_criatomextween_stop`
    /// was called previously and if either you or the hooked process have allocated the instance of it.
    pub(crate) fn get_criatomextween_stop() -> Option<& 'static u8>;
   /// Like `get_criatomextween_stop_mut`, but a mutable reference is created instead.
    pub(crate) fn get_criatomextween_stop_mut() -> Option<& 'static mut u8>;
   /// An unchecked version of `get_criatomextween_stop`. This assumes that CRIATOMEXTWEEN_STOP
    /// is set and it's initialized.
    pub(crate) fn get_criatomextween_stop_unchecked() -> & 'static u8;
   /// An unchecked version of `get_criatomextween_stop_mut`. This assumes that CRIATOMEXTWEEN_STOP
    /// is set and it's initialized.
    pub(crate) fn get_criatomextween_stop_unchecked_mut() -> & 'static mut u8;
   /// Change the value of `CRIATOMEXTWEEN_STOP`. Ensure that you've freed the existing data if
    /// it was allocated!
    pub(crate) fn change_criatomextween_stop(new: u8);
}

