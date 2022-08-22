
use bevy_oddio::{
    builtins::sine::{self, Sine},
    output::{AudioHandle, AudioSink},
    Audio, AudioPlugin,
};
use oddio::Sample;

use bevy::prelude::*;

#[derive(Deref)]
pub struct SineHandle(Handle<Sine>);

struct SineSink(Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>);

pub fn init_assets(mut commands: Commands, mut assets: ResMut<Assets<Sine>>) {
    let handle = assets.add(Sine);
    commands.insert_resource(SineHandle(handle));
}

pub fn play_sine(
    frequency_hz: f32,
    mut commands: Commands,
    mut audio: ResMut<Audio<Sample, Sine>>,
    noise: Res<SineHandle>,
) {
    // Note is in A4.
    let handles = audio.play(
        noise.clone(), 
    sine::Settings::new(0.0, frequency_hz));
    commands.insert_resource(SineSink(handles.0, handles.1));
}