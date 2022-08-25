use bevy::prelude::*;
use itertools::Itertools;
use smallvec::ToSmallVec;

use crate::{cluster::*, components::*};

pub struct ChordTextPlugin;

impl Plugin for ChordTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
        .add_system_to_stage(CoreStage::PostUpdate, change_chord_text);
    }
}

#[derive(Component)]
pub struct ChordTextComponent {}

fn change_chord_text(
    playing_orbs: Query<(Entity, &Orb, &PlayingSound)>,
    removals: RemovedComponents<PlayingSound>,
    additions: Query<&PlayingSound, Added<PlayingSound>>,
    mut text_query: Query<(&ChordTextComponent, &mut Text)>,
) {
    if removals.iter().next().is_some() || additions.iter().next().is_some() {
        //something has changed. Reset chord text
        let notes = playing_orbs
            .iter()
            .flat_map(|x| x.1.cluster.notes.clone())
            .sorted()
            .dedup()
            .collect_vec()
            .to_smallvec();

        let cluster = Cluster { notes };

        for (_, mut text) in text_query.iter_mut() {
            let chord_name_option = cluster.get_chord_name();

            text.sections[0].value = chord_name_option.unwrap_or_default();
            text.sections[1].value = cluster.get_notes_text();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::MIDNIGHT_BLUE,
                }),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::DARK_GRAY,
                }),
            ]) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_LEFT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ChordTextComponent {});
}
