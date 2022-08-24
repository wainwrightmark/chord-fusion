use bevy_oddio::{
    builtins::sine::{self, Sine},
    output::{AudioHandle, AudioSink},
    Audio,
};
use itertools::Itertools;
use oddio::Sample;

use bevy::prelude::*;

use crate::{
    cluster::*,
    components::{Orb, PlayingSound},
};

pub struct SoundPlugin;
impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_assets)
            .init_resource::<CurrentSound>()
            .add_system_to_stage(CoreStage::PostUpdate, stop_sounds.label("stop_sounds"))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                start_sounds.label("start_sounds").after("stop_sounds"),
            );
    }
}

#[derive(Deref)]
pub struct SineHandle(Handle<Sine>);

struct SineSink(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>);

#[derive(Default)]
pub struct CurrentSound {
    pub handles: Option<Vec<(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>)>>,
}

fn init_assets(mut commands: Commands, mut assets: ResMut<Assets<Sine>>) {
    let handle = assets.add(Sine);
    commands.insert_resource(SineHandle(handle));
}

fn stop_sounds(
    removals: RemovedComponents<PlayingSound>,
    mut current_sound: ResMut<CurrentSound>,
    mut audio_handles: ResMut<Assets<AudioHandle<Sine>>>,
    mut audio_sinks: ResMut<Assets<AudioSink<Sine>>>,
) {
    for _entity in removals.iter() {
        if let Some(handles_vec) = &current_sound.handles {
            stop_sine(&handles_vec, &mut audio_handles, &mut audio_sinks);
            current_sound.handles = None;
        }
    }
}

fn start_sounds(
    mut commands: Commands,
    query: Query<&Orb, Added<PlayingSound>>,
    mut current_sound: ResMut<CurrentSound>,
    mut audio: ResMut<Audio<Sample, Sine>>,
    noise: Res<SineHandle>,
) {
    for orb in query.iter() {
        if current_sound.handles.is_none() {
            let handles = play_sine(
                orb.cluster.clone(),
                &mut commands,
                &mut audio,
                noise.clone(),
            );
            current_sound.handles = Some(handles);
        }
    }
}

fn play_sine(
    cluster: Cluster,
    commands: &mut Commands,
    audio: &mut ResMut<Audio<Sample, Sine>>,
    noise: Handle<Sine>,
) -> Vec<(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>)> {
    let handles_vec = cluster
        .notes
        .iter()
        .map(|note| {
            let handles = audio.play(
                noise.clone(),
                sine::Settings::new(0.0, note.get_frequency()),
            );
            let cloned_handles = (handles.0.clone_weak(), handles.1.clone_weak());
            commands.insert_resource(SineSink(handles.0, handles.1));
            cloned_handles
        })
        .collect_vec();

    handles_vec
}

fn stop_sine(
    handles_vec: &Vec<(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>)>,
    audio_handles: &mut ResMut<Assets<AudioHandle<Sine>>>,
    audio_sinks: &mut ResMut<Assets<AudioSink<Sine>>>,
) {
    for handles in handles_vec {
        audio_handles.remove(&handles.0);

        if let Some(mut audio_sink) = audio_sinks.remove(handles.1.clone()) {
            audio_sink.control::<oddio::Stop<_>, _>().stop();
        }
    }
}
