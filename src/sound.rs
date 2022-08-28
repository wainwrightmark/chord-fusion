use crate::{cluster::*, events::*};
use bevy::{audio::AudioSink, prelude::*};
use bevy_fundsp::prelude::*;
use itertools::Itertools;

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DspPlugin)
            .add_startup_system(init_dsp.label("init_dsp"))
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                start_all_sounds.label("start_all_sounds"),
            )
            .init_resource::<NoteHandles>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                set_sounds
                    .label("set_sounds")
                    .after("track_notes_playing_changes"),
            );
    }
}

fn pad_sound(hz: f32) -> impl AudioUnit32 {
    (triangle_hz(hz) + sine_hz(hz * 2.)) >> lowpole_hz(100.0) >> (split::<U2>() * 0.2)
}

fn pad_sound_note(note: Note) -> impl AudioUnit32 {
    pad_sound(note.get_frequency())
}

//TODO fix horrible hack
fn sound0() -> impl AudioUnit32 {
    pad_sound_note(Note(0))
}
fn sound1() -> impl AudioUnit32 {
    pad_sound_note(Note(1))
}
fn sound2() -> impl AudioUnit32 {
    pad_sound_note(Note(2))
}
fn sound3() -> impl AudioUnit32 {
    pad_sound_note(Note(3))
}
fn sound4() -> impl AudioUnit32 {
    pad_sound_note(Note(4))
}
fn sound5() -> impl AudioUnit32 {
    pad_sound_note(Note(5))
}
fn sound6() -> impl AudioUnit32 {
    pad_sound_note(Note(6))
}
fn sound7() -> impl AudioUnit32 {
    pad_sound_note(Note(7))
}
fn sound8() -> impl AudioUnit32 {
    pad_sound_note(Note(8))
}
fn sound9() -> impl AudioUnit32 {
    pad_sound_note(Note(9))
}
fn sound10() -> impl AudioUnit32 {
    pad_sound_note(Note(10))
}
fn sound11() -> impl AudioUnit32 {
    pad_sound_note(Note(11))
}

fn init_dsp(mut dsp_manager: ResMut<DspManager>) {
    // length is in seconds

    let len = 5.0; //Note the length has a big impact on the loading time
                   //TODO fix horrible hack
    dsp_manager
        .add_graph(sound0, len)
        .add_graph(sound1, len)
        .add_graph(sound2, len)
        .add_graph(sound3, len)
        .add_graph(sound4, len)
        .add_graph(sound5, len)
        .add_graph(sound6, len)
        .add_graph(sound7, len)
        .add_graph(sound8, len)
        .add_graph(sound9, len)
        .add_graph(sound10, len)
        .add_graph(sound11, len);
}

fn start_all_sounds(
    dsp_assets: Res<DspAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    mut note_handles: ResMut<NoteHandles>,
) {
    let settings = PlaybackSettings {
        repeat: true,
        volume: 0.0,
        ..Default::default()
    };

    //Fix horrible hack
    let handles = [
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound0), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound1), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound2), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound3), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound4), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound5), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound6), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound7), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound8), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound9), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound10), settings.clone())),
        audio_sinks
            .get_handle(audio.play_with_settings(dsp_assets.graph(&sound11), settings)),
    ];

    note_handles.handles = Some(handles);
}

#[derive(Default)]
pub struct NoteHandles {
    pub handles: Option<[Handle<AudioSink>; 12]>,
}

fn set_sounds(
    mut er: EventReader<NotesPlayingChangedEvent>,
    note_handles: Res<NoteHandles>,
    audio_sinks: ResMut<Assets<AudioSink>>,
) {
    if let Some(ev) = er.iter().last() {
        if let Some(handles) = &note_handles.handles {
            //something has changed. Reset all volumes
            let counts = ev.notes.iter().counts();

            let total: usize = counts.values().sum();

            for n in Note::ALL_NOTES {
                let c = *counts.get(&n).unwrap_or(&0);
                let vol = (c as f32) / total as f32;

                let handle = &handles[n.0 as usize];

                if let Some(sink) = audio_sinks.get(handle) {
                    sink.set_volume(vol);
                }
            }
        }
    }
}
