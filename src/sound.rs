use bevy_oddio::{
    builtins::sine::{self, Sine},
    output::{AudioHandle, AudioSink},
    Audio,
};
use itertools::Itertools;
use oddio::Sample;

use bevy::prelude::*;

use crate::cluster::*;

#[derive(Deref)]
pub struct SineHandle(Handle<Sine>);

struct SineSink(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>);

pub fn init_assets(mut commands: Commands, mut assets: ResMut<Assets<Sine>>) {
    let handle = assets.add(Sine);
    commands.insert_resource(SineHandle(handle));
}

pub fn play_sine(
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

pub fn stop_sine(
    handles_vec: Vec<(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>)>,
    audio_handles: &mut ResMut<Assets<AudioHandle<Sine>>>,
    audio_sinks: &mut ResMut<Assets<AudioSink<Sine>>>,
) {
    for handles in handles_vec {
        audio_handles.remove(handles.0);

        if let Some(mut audio_sink) = audio_sinks.remove(handles.1) {
            audio_sink.control::<oddio::Stop<_>, _>().stop();
        }
    }
}
