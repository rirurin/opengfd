use cri_adx::{
    acb::Acb,
    atom::ffi::CriAtomFormat,
    awb::Awb,
    player::{
        AtomExPlayer,
        ffi::{
            CriAtomExPlaybackId,
            CriAtomExPlayerStatus
        }
    },
    tween::Tween
};
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct SoundPlayer {
    unk0: [u8; 0x8],
    player: Option<NonNull<AtomExPlayer>>,
    playback_handle: CriAtomExPlaybackId,
    audio_data: Option<NonNull<SoundAudioData>>,
    field20: usize,
    cue_id: u32,
    unk1: [u8; 0x24],
    tween: Option<NonNull<Tween>>,
    is_playing: bool,
    // field58: usize,
    acf_data: Option<NonNull<AcfData>>,
    unk2: [u8; 0x60]
}

impl SoundPlayer {
    pub fn get_status(&self) -> Option<cri_adx::player::ffi::CriAtomExPlayerStatus> {
        self.player.map(|v| unsafe { v.as_ref().get_status() })
    }

    pub fn get_player(&self) -> Option<&AtomExPlayer> {
        self.player.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_player_mut(&mut self) -> Option<&mut AtomExPlayer> {
        self.player.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_audio_data(&self) -> Option<&SoundAudioData> {
        self.audio_data.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_audio_data_mut(&mut self) -> Option<&mut SoundAudioData> {
        self.audio_data.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_tween(&self) -> Option<&Tween> {
        self.tween.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_tween_mut(&mut self) -> Option<&mut Tween> {
        self.tween.map(|mut v| unsafe { v.as_mut() })
    }

    pub fn send_signal(&mut self) {
        let send_signal_fn = unsafe { &raw const *crate::globals::get_sound_player_send_signal_unchecked() };
        let send_signal_fn = unsafe { std::mem::transmute::<_, fn(&mut SoundPlayer) -> ()>(send_signal_fn) };
        send_signal_fn(self);
    }

    // 0x14147d290
    pub fn play_cue(&mut self, cue_id: i32) {
        if self.audio_data.is_none() { return; }
        self.get_tween_mut().unwrap().reset();
        if self.is_playing {
            self.get_player_mut().unwrap().stop_without_release_time();
            self.is_playing = false;
        }
        match self.get_audio_data().unwrap().get_sound_type() {
            SoundType::Cue | SoundType::CueAlt => {
                let acb = unsafe { self.get_audio_data().unwrap().acb.unwrap().as_ref() };
                self.get_player_mut().unwrap().set_cue_id(acb, cue_id);
            },
            SoundType::Wave => {
                let awb = unsafe { self.get_audio_data().unwrap().awb.unwrap().as_ref() };
                let player = self.get_player_mut().unwrap();
                player.set_wave_id(awb, cue_id);
                player.set_format(CriAtomFormat::CRIATOM_FORMAT_HCA_MX);
                player.set_num_channels(1);
                player.set_sample_rate(48000);
            }
        };
        self.playback_handle = self.get_player_mut().unwrap().start();
        self.send_signal();
        self.cue_id = cue_id as u32;
    }

    // 0x14147d5b0
    // Original function: TPL::SoundPlayer::SetAisacControlById
    pub fn set_aisac_control_by_id(&mut self, id: u32, value: f32) {
        self.get_player_mut().unwrap().set_aisac_control_by_id(id, value);
        self.send_signal();
    }

    // 0x14147d8c0
    // Original function: TPL::SoundPlayer::Stop
    pub fn stop(&mut self) {
        self.get_player_mut().unwrap().stop();
        self.get_tween_mut().unwrap().stop();
        self.playback_handle.set_invalid();
        self.cue_id = u32::MAX;
    }

    // 0x14147d200
    // Original function: TPL::SoundPlayer::IsPaused
    pub fn is_paused(&self) -> bool {
        match self.get_player() {
            Some(v) => v.is_paused(),
            None => true
        }
    }

    // Original function: TPL::SoundPlayer::Pause
    pub fn pause(&mut self, sw: bool) {
        self.get_player_mut().unwrap().pause(sw);
    }

    // 0x14147d220
    // Original function: TPL::SoundPlayer::IsPlaying
    pub fn is_playing(&self) -> bool {
        match self.get_player() {
            Some(v) => match v.get_status() {
                CriAtomExPlayerStatus::CRIATOMEXPLAYER_STATUS_PREP |
                CriAtomExPlayerStatus::CRIATOMEXPLAYER_STATUS_PLAYING => true,
                _ => false
            },
            None => false
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundType {
    Cue = 0,
    Wave = 1,
    CueAlt = 2
}

#[repr(C)]
#[derive(Debug)]
pub struct SoundAudioData {
    unk0: [u8; 0x10],
    acb: Option<NonNull<Acb>>,
    awb: Option<NonNull<Awb>>,
    sound_type: SoundType
}
impl SoundAudioData {
    pub fn get_sound_type(&self) -> SoundType {
        self.sound_type
    }
    pub fn get_acb_handle(&self) -> Option<&Acb> {
        self.acb.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_acb_handle_mut(&mut self) -> Option<&mut Acb> {
        self.acb.map(|mut v| unsafe { v.as_mut() })
    }
    pub fn get_awb_handle(&self) -> Option<&Awb> {
        self.awb.map(|v| unsafe { v.as_ref() })
    }
    pub fn get_awb_handle_mut(&mut self) -> Option<&mut Awb> {
        self.awb.map(|mut v| unsafe { v.as_mut() })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AcfData {
    unk: [u8; 0xb8]
}
