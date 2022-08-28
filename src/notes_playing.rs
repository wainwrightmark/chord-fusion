use crate::components::*;
use crate::events::*;
use bevy::prelude::*;
use itertools::Itertools;

pub struct NotesPlayingPlugin;

impl Plugin for NotesPlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            track_notes_playing_changes.label("track_notes_playing_changes"),
        );
    }
}

pub fn track_notes_playing_changes(
    all_orbs: Query<(&Orb, &Interactable)>,
    changed_interactables: Query<Changed<Interactable>>,
    mut ew: EventWriter<NotesPlayingChangedEvent>,
) {
    if !changed_interactables.is_empty() {
        let notes = all_orbs
            .iter()
            .filter(|x| x.1.interacting)
            .flat_map(|x| x.0.cluster.notes.clone())
            .collect_vec();

        ew.send(NotesPlayingChangedEvent { notes })
    }
}
