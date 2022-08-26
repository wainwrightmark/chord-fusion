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
    playing_orbs: Query<(Entity, &Orb, &PlayingSound)>,
    removals: RemovedComponents<PlayingSound>,
    additions: Query<&PlayingSound, Added<PlayingSound>>,
    mut ew: EventWriter<NotesPlayingChangedEvent>,
) {
    if removals.iter().next().is_some() || additions.iter().next().is_some() {
        let notes = playing_orbs
            .iter()
            .flat_map(|x| x.1.cluster.notes.clone())
            .collect_vec();

        ew.send(NotesPlayingChangedEvent { notes })
    }
}
