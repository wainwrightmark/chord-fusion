use crate::{
    cluster::*,
    components::{Orb, PlayingSound},
};
use bevy::{audio::AudioSink, prelude::*};
use bevy_fundsp::prelude::*;
use itertools::Itertools;

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DspPlugin)
            .add_startup_system(init_dsp)
            .init_resource::<CurrentSound>()
            .add_system_to_stage(CoreStage::PostUpdate, stop_sounds.label("stop_sounds"))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                start_sounds.label("start_sounds").after("stop_sounds"),
            );
    }
}

fn pad_sound(hz: f32) -> impl AudioUnit32 {
    (triangle_hz(hz) + square_hz(hz)) >> lowpole_hz(100.0) >> split::<U2>() * 0.2
    // (triangle_hz(hz) | lfo(move |t| xerp11(50.0, 5000.0, fractal_noise(0, 6, 0.5, t * 0.2))))
    //         >> bandpass_q(5.0)
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

//const PLAY_NOTE_FUNCTIONS: [dyn FnDspGraph; 12] = [play_note_0];

fn init_dsp(mut dsp_manager: ResMut<DspManager>) {
    // length is in seconds

    //TODO fix horrible hack
    dsp_manager.add_graph(sound0, 5.0);
    dsp_manager.add_graph(sound1, 5.0);
    dsp_manager.add_graph(sound2, 5.0);
    dsp_manager.add_graph(sound3, 5.0);
    dsp_manager.add_graph(sound4, 5.0);
    dsp_manager.add_graph(sound5, 5.0);
    dsp_manager.add_graph(sound6, 5.0);
    dsp_manager.add_graph(sound7, 5.0);
    dsp_manager.add_graph(sound8, 5.0);
    dsp_manager.add_graph(sound9, 5.0);
    dsp_manager.add_graph(sound10, 5.0);
    dsp_manager.add_graph(sound11, 5.0);
}

#[derive(Default)]
pub struct CurrentSound {
    pub handles: Option<Vec<Handle<AudioSink>>>,
}

fn stop_sounds(
    removals: RemovedComponents<PlayingSound>,
    mut current_sound: ResMut<CurrentSound>,
    mut audio_sinks: ResMut<Assets<AudioSink>>,
) {
    for _entity in removals.iter() {
        if let Some(handles_vec) = &current_sound.handles {
            stop_sound(handles_vec, &mut audio_sinks);
            current_sound.handles = None;
        }
    }
}

fn start_sounds(
    query: Query<&Orb, Added<PlayingSound>>,
    mut current_sound: ResMut<CurrentSound>,

    dsp_assets: Res<DspAssets>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for orb in query.iter() {
        if current_sound.handles.is_none() {
            let handles = play_sound(orb.cluster.clone(), &dsp_assets, &audio, &audio_sinks);
            current_sound.handles = Some(handles);
        }
    }
}

fn play_sound(
    cluster: Cluster,
    dsp_assets: &Res<DspAssets>,
    audio: &Res<Audio>,
    audio_sinks: &Res<Assets<AudioSink>>,
) -> Vec<Handle<AudioSink>> {
    let handles_vec = cluster
        .notes
        .iter()
        .map(|&note| {
            //TODO fix horrible hack
            let audio_source = match note.0 {
                0 => dsp_assets.graph(&sound0),
                1 => dsp_assets.graph(&sound1),
                2 => dsp_assets.graph(&sound2),
                3 => dsp_assets.graph(&sound3),
                4 => dsp_assets.graph(&sound4),
                5 => dsp_assets.graph(&sound5),
                6 => dsp_assets.graph(&sound6),
                7 => dsp_assets.graph(&sound7),
                8 => dsp_assets.graph(&sound8),
                9 => dsp_assets.graph(&sound9),
                10 => dsp_assets.graph(&sound10),
                11 => dsp_assets.graph(&sound11),
                _ => unimplemented!(),
            };

            let weak_handle = audio.play(audio_source);
            let strong_handle = audio_sinks.get_handle(weak_handle);

            strong_handle
        })
        .collect_vec();

    handles_vec
}

fn stop_sound(handles_vec: &Vec<Handle<AudioSink>>, audio_sinks: &mut ResMut<Assets<AudioSink>>) {
    for handle in handles_vec {
        if let Some(audio_sink) = audio_sinks.remove(handle) {
            audio_sink.stop();
        }
    }
}
