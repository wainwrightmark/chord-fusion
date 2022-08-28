use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;
use smallvec::ToSmallVec;

use crate::{cluster::*, components::*, events::*, BIG_TEXT_COLOR, SMALL_TEXT_COLOR};

pub struct ChordTextPlugin;

impl Plugin for ChordTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::PostUpdate, set_drawmode_for_orbs)
            //.add_system_to_stage(CoreStage::PostUpdate, set_drawmode_for_stopped_playing)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                change_chord_text.after("track_notes_playing_changes"),
            );
    }
}

#[derive(Component)]
pub struct ChordTextComponent {}

pub fn set_drawmode_for_orbs(
    changed_orbs: Query<Changed<Interactable>>,
    mut query: Query<(&mut DrawMode, &Orb, &Interactable)>,
) {
    if !changed_orbs.is_empty() {
        for (mut dm, orb, interactable) in query.iter_mut() {
            *dm = orb.cluster.get_draw_mode(interactable.interacting);
        }
    }
}

fn change_chord_text(
    mut er: EventReader<NotesPlayingChangedEvent>,
    mut text_query: Query<(&ChordTextComponent, &mut Text)>,
) {
    if let Some(ev) = er.iter().last() {
        //something has changed. Reset chord text
        let notes = ev
            .notes
            .iter()
            .sorted()
            .dedup()
            .cloned()
            .collect_vec()
            .to_smallvec();

        let cluster = Cluster { notes };

        for (_, mut text) in text_query.iter_mut() {
            let chord_option = cluster.get_chord();

            if let Some((root, chord)) = chord_option {
                text.sections[0].value = format!("{} {}", root.get_name(), chord.nice_name());
                text.sections[1].value = cluster.get_notes_text();
            } else {
                text.sections[0].value = cluster.get_notes_text();
                text.sections[1].value = "".to_string();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                flex_grow: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|f| {
            f.spawn_bundle(
                TextBundle::from_sections([
                    TextSection::from_style(TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: BIG_TEXT_COLOR,
                    }),
                    TextSection::from_style(TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: SMALL_TEXT_COLOR,
                    }),
                ]) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_LEFT), // Set the style of the TextBundle itself.,
            )
            .insert(ChordTextComponent {});
        });
}
