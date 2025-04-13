use crate::globals;
use opengfd::tpl::{
    file_manager::FileManager,
    sound::player::SoundPlayer
};
use riri_mod_tools_proc::{ riri_hook_fn, riri_hook_static };
use riri_mod_tools_rt::{
    logln, 
    sigscan_resolver 
};
use std::ptr::NonNull;

#[no_mangle]
pub unsafe extern "C" fn set_file_manager_instance_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_indirect_address_long(ofs) {
        Some(v) => v,
        None => return None
    };
    logln!(Information, "got File Manager instance: 0x{:x}", addr.as_ptr() as usize);
    globals::set_file_manager_instance(addr.as_ptr() as *mut *mut FileManager);
    Some(addr)
}
/*
#[riri_hook_static(dynamic_offset(
    signature = "48 8B 2D ?? ?? ?? ?? 45 33 F6 C5 F9 EF C0",
    resolve_type = set_file_manager_instance_hook,
    calling_convention = "microsoft",
))]
riri_static!(FILE_MANAGER_INSTANCE_HOOK, usize);
#[riri_hook_fn(static_offset(0x147c470))]
pub unsafe extern "C" fn tplResourceCreate(mgmt: *mut u8, ret_storage: *mut u8, filename: *mut u8, a4: u32) -> *mut u8 {
    let filename_cast = unsafe { &*(filename as *mut CppString) };
    let mgmt_cast = unsafe { &*(mgmt as *mut FileManager<AllocatorHook>) };
    match mgmt_cast.find_by_key(unsafe { &*(filename as *const CppString<u8, AllocatorHook>) } ) {
        Some(v) => logln!(Information, "tplResourceCreate: GET {}, EXIST 0x{:x}", filename_cast, &raw const *v as usize),
        None => logln!(Information, "tplResourceCreate: GET {}, NOT LOADED", filename_cast),
    }
    let out = original_function!(mgmt, ret_storage, filename, a4);
    // logln!(Information, "FileManager->load_files.len() = {}", mgmt_cast.get_loading_file_list().len());
    /*
    for (i, load_file) in mgmt_cast.get_loading_file_list().iter().enumerate() {
        logln!(Information, "FileManager->load_files[{}] = {}", i, load_file.get().get_filename());
    }
    */
    // for (i, load_file) in mgmt_cast
    // logln!(Information, "tplResourceCreate: loading_file_list: {}")
    // logln!(Information, "tplResourceCreate: load_file size: {}", mgmt_cast.get_loading_file_list().len());
    /*
    if mgmt_cast.get_active_files().size() % 500 == 0 {
        for (i, mgmt_file) in mgmt_cast.get_active_files().iter().enumerate() {
            let resrc = unsafe { &**mgmt_file.get_value() };
            logln!(Information, "FileManager->load_files[{}] = {:?}", i, resrc);
        }
    }
    */
    out
}
*/

/*

#[riri_hook_fn(static_offset(0x147c470))]
pub unsafe extern "C" fn tplResourceCreate(mgmt: *mut u8, ret_storage: *mut u8, filename: *mut u8, _a4: u32, _a5: u32) -> *mut u8 {
    let filename_cast = unsafe { &mut *(filename as *mut CppString<u8, AllocatorHook>) };
    let mgmt_cast = unsafe { &mut *(mgmt as *mut FileManager<AllocatorHook>) };
    let ret = unsafe { &mut *(ret_storage as *mut SharedPtr<Resource<usize, AllocatorHook>, AllocatorHook>) };
    // logln!(Information, "tplResourceCreate: GET {}", filename_cast);
    // logln!(Information, "load list size: {}, active file map size: {}", 
    //        mgmt_cast.get_loading_file_list().len(), mgmt_cast.get_active_files().size());
    match mgmt_cast.find_by_key(unsafe { &*(filename as *const CppString<u8, AllocatorHook>) } ) {
        Some(v) => {
            ret._force_set_ptr(v._force_get_ptr());
            ret._force_set_rep(v._force_get_rep());
        },
        None => {
            let out = original_function!(mgmt, ret_storage, filename, _a4, _a5);
            // let ret_out = unsafe { &*(out as *mut WeakPtr<Resource<usize, AllocatorHook>, AllocatorHook>) };
            // logln!(Information, "strong: {}, weak: {}", ret_out.strong_count(), ret_out.weak_count());
            return out;
            /*
            let new = SharedPtr::into_raw(mgmt_cast.add_resource(filename_cast));
            ret._force_set_ptr(unsafe { (&mut *new).get_data_ptr() });
            ret._force_set_rep(new);
            let ret_out = unsafe { &*(ret as *mut SharedPtr<Resource<usize, AllocatorHook>, AllocatorHook> )}; 
            logln!(Information, "strong: {}, weak: {}, filename: {}", 
                ret_out.strong_count(), ret_out.weak_count(), ret_out.get().get_filename());
            */
        },
    }
    // drop filename
    unsafe { std::ptr::drop_in_place(filename_cast); }
    ret_storage
}
/*
#[riri_hook_fn(static_offset(0x147a9e0))]
pub unsafe extern "C" fn tplSetLoadStateReady(res: *mut u8, a2: usize) {
    let _ = original_function!(res, a2);
    /*
    let ret_out = unsafe { &*(res as *mut Resource<usize, AllocatorHook>) };
    let owner = ret_out.get_owner();
    logln!(Information, "LOAD COMPLETE {}, owner: 0x{:x}, strong: {}, weak: {}", 
        ret_out.get_filename(), &raw const *owner as usize, owner.strong_count(), owner.weak_count());
    */
}

#[riri_hook_fn(static_offset(0x147ce70))]
pub unsafe extern "C" fn tplSoftFree(_ptr: *mut u8) {
    // no-op this
}

#[riri_hook_fn(static_offset(0x147ce50))]
pub unsafe extern "C" fn tplHardFree(_ptr: *mut u8) {
    // no-op this
}

#[riri_hook_fn(static_offset(0x147a150))]
pub unsafe extern "C" fn tplResourceGetStream(_ptr: *mut u8) -> *mut u8 {
    let out = original_function!(_ptr);
    let resrc = unsafe { &*(_ptr as *mut Resource<usize, AllocatorHook>) };
    logln!(Information, "tplResourceGetStream: got buffer 0x{:x}", out as usize);
    out
}
*/

/*
#[riri_hook_fn(static_offset(0x1479e40))]
pub unsafe extern "C" fn tplResourceInitialize(a1: *mut u8, a2: *mut u8, a3: i32, a4: i32) -> *mut u8 {
    logln!(Information, "tplResourceInitialize: a3: {}, a4: {}", a3, a4);
    let out = original_function!(a1, a2, a3, a4);
    let outkast = unsafe { &mut*(out as *mut Resource<usize, AllocatorHook>) };
    outkast.zero_timestamp();
    out
}
*/

#[no_mangle]
pub unsafe extern "C" fn set_tpl_resource_shared_ptr_hook(ofs: usize) -> Option<std::ptr::NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    let addr = match sigscan_resolver::get_indirect_address_long_abs(addr.add(0x57).as_ptr()) {
        Some(v) => v,
        None => return None
    };
    globals::set_tpl_resource_shared_ptr(addr.as_ptr() as *mut u8);
    logln!(Information, "got std::shared_ptr<TPL::Resource> vtable: 0x{:x}", addr.as_ptr() as usize);
    Some(addr)
}
// 0x1411b0ce0, inside ngrInitFreeList
#[riri_hook_static(dynamic_offset(
    signature = "48 89 5C 24 ?? 48 89 6C 24 ?? 48 89 4C 24 ?? 56 57 41 54 41 56 41 57 48 83 EC 50 49 8B D9 49 8B F8 4C 8B E2 4C 8B F1 33 ED B9 A0 00 00 00",
    resolve_type = set_tpl_resource_shared_ptr_hook,
    calling_convention = "microsoft",
))]
riri_static!(TPL_RESOURCE_SHARED_PTR_HOOK, usize);

#[riri_hook_fn(static_offset(0x147d420))]
pub unsafe extern "C" fn TplFileManagerThreadEventLoop(p_mgmt: *mut u8) {
    logln!(Information, "Start TPL File Manager Thread");
    original_function!(p_mgmt)
    /*
    let mgmt = unsafe { &mut *(p_mgmt as *mut FileManager<AllocatorHook>) };
    mgmt.set_running(true);
    while mgmt.get_running() {
        let mut files_to_initialize: Vector<SharedPtr<Resource<usize, AllocatorHook>, AllocatorHook>, AllocatorHook> = Vector::new_in(AllocatorHook);
        let mut mutex = mgmt.lock_file_mutex();
        if (&*mutex).get_loading_file_list().is_empty() {
            (&mut* mutex).disable_queue();
            while !(&*mutex).check_queue() {
                drop(mutex);
                logln!(Information, "Waiting for game thread to request resource...");
                mgmt.wait_for_file_to_load();
                if let Some(f) = mgmt.get_loading_file_list().last() {
                    logln!(Information, "Got file load request from game thread: {}", f.value().get().get_filename());
                }
                mutex = mgmt.lock_file_mutex();
            }
        } else {
            let mut remove_indices: Vec<usize> = vec![];
            for (i, file_ref) in (&*mutex).get_loading_file_list().iter().enumerate() {
                let file = file_ref.clone();
                // logln!(Information, "In loading file list: {}", file.get().get_filename());
                // logln!(Information, "stream type: {}, load state: {:?}", file.get().get_stream_type(), file.get().get_load_state());
                if file.get().get_stream_type() == StreamType::None
                && file.get().get_load_state() == LoadState::Uninitialized {
                    files_to_initialize.push(file);
                } else {
                // } else if file.get().get_load_state() == LoadState::Ready {
                    if file.get().get_stream_type() == StreamType::SpriteAPKEntry {
                        tplResourceLoadSubfile(&raw const *file.get() as *const u8 as *mut u8);
                    }
                    if file.get().get_load_state() == LoadState::Ready {
                        remove_indices.push(i);
                    }
                }
            }
            let mut removed_count = 0;
            for loaded in remove_indices {
                // logln!(Information, "Remove index {}, len {}", loaded - removed_count, (&*mutex).get_loading_file_list().len());
                match (&mut *mutex).get_loading_file_list_mut().remove_checked(loaded - removed_count) {
                    Some(v) => {
                        logln!(Information, "Loaded {}, type {:?}", v.get().get_filename(), v.get().get_stream_type());
                        removed_count += 1;
                    },
                    None => ()
                };
            }
        }
        drop(mutex);
        for file in &files_to_initialize {
            let mut start_load = file.clone();
            let start_load_native = &raw mut start_load as *mut u8;
            tplStartLoadingResource(start_load_native);
        }
        tplLoadFileFromCrifs();
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
    */
}

#[riri_hook_fn(static_offset(0x1482970))]
// pub unsafe extern "C" fn tplStartLoadingResource(resrc: &SharedPtr<Resource<usize, AllocatorHook>, AllocatorHook>) {
pub unsafe extern "C" fn tplStartLoadingResource(resrc: *mut u8) {
    // logln!(Information, "tplStartLoadingResource: 0x{:x}", resrc as usize);
    original_function!(resrc)
}

#[riri_hook_fn(static_offset(0x202cb330))]
pub unsafe extern "C" fn tplLoadFileFromCrifs() {
    // logln!(Information, "tplLoadFileFromCrifs");
    original_function!()
}

#[riri_hook_fn(static_offset(0x2018fdf0))]
pub unsafe extern "C" fn tplResourceLoadSubfile(resrc: *mut u8) {
    original_function!(resrc)
}

*/


#[riri_hook_fn(static_offset(0x14870c0))]
#[allow(non_snake_case)]
pub unsafe extern "C" fn tplSoundPlayerPlayCue(p_snd_player: *mut u8, id: i32) {
    let snd_player = &mut *(p_snd_player as *mut SoundPlayer);
    // logln!(Verbose, "Player 0x{:x} - Play Cue {}", p_snd_player as usize, id);
    snd_player.play_cue(id);
    // original_function!(p_snd_player, id)
}

#[no_mangle]
pub unsafe extern "C" fn set_sound_player_send_signal_hook(ofs: usize) -> Option<NonNull<u8>> { 
    let addr = match sigscan_resolver::get_address_may_thunk(ofs) {
        Some(v) => v,
        None => return None
    };
    // logln!(Information, "got TPL::SoundPlayer::SendSignal: 0x{:x}", addr.as_ptr() as usize);
    globals::set_sound_player_send_signal(addr.as_ptr());
    Some(addr)
}
#[riri_hook_static(dynamic_offset(
    signature = "49 89 E3 49 89 5B ?? 57 48 81 EC 80 00 00 00 48 8B 05 ?? ?? ?? ?? 48 31 E0 48 89 44 24 ?? 48 89 CF 80 39 00",
    resolve_type = set_sound_player_send_signal_hook,
    calling_convention = "microsoft",
))]
riri_static!(SOUND_PLAYER_SEND_SIGNAL_HOOK, usize);