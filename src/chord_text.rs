use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;
use smallvec::ToSmallVec;

use crate::{cluster::*, components::*, BIG_TEXT_COLOR, SMALL_TEXT_COLOR};

pub struct ChordTextPlugin;

impl Plugin for ChordTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_to_stage(CoreStage::PostUpdate, set_drawmode_for_playing)
            .add_system_to_stage(CoreStage::PostUpdate, set_drawmode_for_stopped_playing)
            .add_system_to_stage(CoreStage::PostUpdate, change_chord_text)
            
            ;
    }
}

#[derive(Component)]
pub struct ChordTextComponent {}

pub fn set_drawmode_for_playing(mut query:  Query<(&mut DrawMode, &Orb, Added<PlayingSound>)>) {
    for (mut dm, orb, _) in query.iter_mut(){
        *dm =orb.cluster.get_draw_mode(true);
    }    
}

pub fn set_drawmode_for_stopped_playing(mut query:  Query<(&mut DrawMode, &Orb)>,
removals: RemovedComponents<PlayingSound>
) {

    for rem in removals.iter(){
        if let Ok((mut dm, orb)) = query.get_mut(rem){
            *dm =orb.cluster.get_draw_mode(false);
        }
    }
}

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

            if let Some(chord_name) = chord_name_option {
                text.sections[0].value = chord_name;
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
        .spawn_bundle(
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
            .with_text_alignment(TextAlignment::TOP_LEFT)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            }),
        )
        .insert(ChordTextComponent {});
}
