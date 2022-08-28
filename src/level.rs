use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;

use crate::cluster::*;
use crate::objective::*;
use crate::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
        .add_system_to_stage(
            CoreStage::PostUpdate,
                check_for_completion.after("update_met_objectives"))
            .add_startup_system(setup_level_text)
            .add_startup_system_to_stage(StartupStage::PostStartup, start_next_level);
    }
}

fn setup_level_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                flex_grow: 0.,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|f| {
            f.spawn_bundle(
                TextBundle::from_sections([TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: SMALL_TEXT_COLOR,
                })]) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::CENTER),
            )
            .insert(LevelText {});
        });
}
fn check_for_completion(
    mut commands: Commands,
    added_completions: Query<Added<CompletingObjective>>,
    objectives: Query<(Entity, &Objective)>,
    orbs: Query<(Entity, &Orb)>,
    current_level: ResMut<CurrentLevel>,
    level_text: Query<(Entity, &LevelText, &mut Text)>,
) {
    if !added_completions.is_empty() {
        if objectives.iter().all(|o| o.1.is_complete) {
            for (e, _) in objectives.iter() {
                commands.entity(e).despawn_recursive();
            }
            for (e, _) in orbs.iter() {
                commands.entity(e).despawn_recursive();
            }

            start_next_level(commands, current_level, level_text)
        }
    }
}

fn start_next_level(
    mut commands: Commands,
    mut current_level: ResMut<CurrentLevel>,
    mut level_text: Query<(Entity, &LevelText, &mut Text)>,
) {
    current_level.0 += 1;
    let level = GameLevel::get_level(current_level.0);

    for (entity, _, mut text) in level_text.iter_mut() {
        text.sections[0].value = format!("{: ^36}", level.name);
        commands.entity(entity).insert(Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            TweeningType::Once,
            Duration::from_secs(10),
            TextColorLens {
                section: 0,
                start: SMALL_TEXT_COLOR,
                end: Color::NONE,
            },
        )));
    }

    for i in 0..level.objectives {
        create_objective(&mut commands, i, level.objectives);
    }

    for n in level.notes {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        create_orb_near(
            &mut commands,
            SHAPE_SIZE,
            Cluster {
                notes: smallvec::smallvec![n],
            },
            rangex,
            rangey,
            &mut Default::default(),
        )
    }
}

#[derive(Component)]
pub struct LevelText {}

#[derive(Default)]
pub struct CurrentLevel(pub usize);

pub struct GameLevel {
    pub name: &'static str,
    pub objectives: usize, //change this
    pub notes: Vec<Note>,
}

impl GameLevel {
    pub fn get_level(i: usize) -> GameLevel {
        match i % 8 {
            0 => GameLevel {
                name: "Octave",
                objectives: 3,
                notes: Note::ALL_NOTES.into(),
            },
            1 => GameLevel {
                name: "First Inversion",
                objectives: 1,
                notes: vec![Note(0), Note(4), Note(7)],
            },
            2 => GameLevel {
                name: "Major Second",
                objectives: 2,
                notes: vec![Note(0), Note(4), Note(7), Note(1), Note(5), Note(8)],
            },
            3 => GameLevel {
                name: "Piano Down a Mine Shaft",
                objectives: 1,
                notes: vec![Note(8), Note(3), Note(11)],
            },
            4 => GameLevel {
                name: "Too Young to Diatonic?",
                objectives: 2,
                notes: vec![Note(0), Note(2), Note(4), Note(5), Note(7), Note(11)],
            },
            5 => GameLevel {
                name: "Diminished Fifth",
                objectives: 2,
                notes: vec![Note(0), Note(3), Note(6), Note(1), Note(4), Note(7)],
            },
            6 => GameLevel {
                name: "Auganization",
                objectives: 2,
                notes: vec![Note(0), Note(4), Note(8), Note(1), Note(5), Note(9)],
            },
            7 => GameLevel {
                name: "Seventh Heaven",
                objectives: 2,
                notes: vec![
                    Note(0),
                    Note(4),
                    Note(7),
                    Note(10),
                    Note(1),
                    Note(4),
                    Note(7),
                    Note(11),
                ],
            },

            _ => unimplemented!(),
        }
    }
}
