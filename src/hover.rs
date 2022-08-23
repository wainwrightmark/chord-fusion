use std::collections::HashMap;

use bevy::prelude::*;

use crate::*;

use bevy_oddio::{
    builtins::sine::Sine,
    output::{AudioHandle, AudioSink},
    Audio,
};
use oddio::Sample;

pub struct HoverPlugin;
impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(detect_hover.label("detect_hover"));
    }
}

pub fn detect_hover(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
    orbs: Query<(Entity, &Orb, Option<&PlayingSound>)>,
    current_sounds: Query<(Entity, &PlayingSound)>,
    mut audio: ResMut<Audio<Sample, Sine>>,
    noise: Res<SineHandle>,
    mut audio_handles: ResMut<Assets<AudioHandle<Sine>>>,
    mut audio_sinks: ResMut<Assets<AudioSink<Sine>>>,
) {
    if let Some(_ev) = cursor_evr.iter().last() {
        let mut c_sounds: HashMap<Entity, &PlayingSound> = current_sounds.into_iter().collect();

        if let Some(position) = get_cursor_position(windows, q_camera) {
            rapier_context.intersections_with_point(position, default(), |entity| {
                if let Ok((e, o, playing)) = orbs.get(entity) {
                    if playing.is_none() {
                        //start playing this sound
                        let handles =
                            play_sine(o.cluster.clone(), &mut commands, &mut audio, noise.clone());

                        commands.entity(e).insert(PlayingSound { handles });
                    } else {
                        //This sound is playing - do not stop it
                        c_sounds.remove(&e);
                    }
                }
                true
            });
        }

        for (e, ps) in c_sounds {
            stop_sine(ps.handles.clone(), &mut audio_handles, &mut audio_sinks);
            commands.entity(e).remove::<PlayingSound>();
        }
    }
}
