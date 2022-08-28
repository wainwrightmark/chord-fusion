use bevy::{prelude::*, utils::HashSet};

use crate::*;

pub struct HoverPlugin;
impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(detect_hover.label("detect_hover"));
    }
}

pub fn detect_hover(
    mut cursor_evr: EventReader<CursorMoved>,
    // need to get window dimensions
    windows: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    rapier_context: Res<RapierContext>,
    mut interactables: Query<(Entity, (&mut Interactable, Option<&Dragged>))>,
) {
    if let Some(_ev) = cursor_evr.iter().last() {
        let mut remaining: HashSet<Entity> = interactables
            .iter()
            .filter(|x| x.1 .0.interacting && x.1 .1.is_none())
            .map(|(e, _)| e)
            .collect();

        if let Some(position) = get_cursor_position(windows, q_camera) {
            rapier_context.intersections_with_point(position, default(), |entity| {
                if let Ok((e, (mut interactable, _))) = interactables.get_mut(entity) {
                    if interactable.interacting {
                        //This sound is playing - do not stop it
                        remaining.remove(&e);
                    } else {
                        //info!("Interacting");
                        interactable.interacting = true;
                    }
                }
                true
            });
        }

        for remaining in remaining {
            if let Ok((_, (mut interactable, _))) = interactables.get_mut(remaining) {
                //info!("No longer Interacting");
                interactable.interacting = false;
            }
        }
    }
}
