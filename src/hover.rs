use bevy::{prelude::*, utils::HashMap};

use crate::*;

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
    orbs: Query<(Entity, Option<&PlayingSound>), With<Orb>>,
    current_sounds: Query<(Entity, Option<&Dragged>), With<PlayingSound>>,
) {
    if let Some(_ev) = cursor_evr.iter().last() {
        let mut c_sounds: HashMap<Entity, Option<&Dragged>> = current_sounds.into_iter().collect();

        if let Some(position) = get_cursor_position(windows, q_camera) {
            rapier_context.intersections_with_point(position, default(), |entity| {
                if let Ok((e, playing)) = orbs.get(entity) {
                    if playing.is_none() {
                        commands.entity(e).insert(PlayingSound {});
                    } else {
                        //This sound is playing - do not stop it
                        c_sounds.remove(&e);
                    }
                }
                true
            });
        }

        for (e, dragged) in c_sounds {
            if dragged.is_none() {
                commands.entity(e).remove::<PlayingSound>(); //Don't stop the sound if the entity is being dragged
            }
        }
    }
}
