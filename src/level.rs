use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::*;
use bevy_tweening::*;
use itertools::Itertools;
use rand::Rng;

use rand::SeedableRng;
use strum::EnumCount;

use crate::chord::Chord;
use crate::cluster::*;
use crate::objective::*;
use crate::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
            .add_system_to_stage(
                CoreStage::PreUpdate,
                check_for_completion, //.after("update_met_objectives"),
            )
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
                flex_direction: FlexDirection::Column,
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
            .insert(LevelText { is_header: false });

            f.spawn_bundle(
                TextBundle::from_sections([TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: BIG_TEXT_COLOR,
                })]) // Set the alignment of the Text
                .with_text_alignment(TextAlignment::CENTER),
            )
            .insert(LevelText { is_header: true });
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

    for (entity, lt, mut text) in level_text.iter_mut() {
        let new_text = if lt.is_header {
            format!("{: ^60}", level.header)
        } else {
            format!("{: ^36}", level.name)
        };

        text.sections[0].value = new_text;
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

    for (i, objective) in level.objectives.iter().enumerate() {
        create_objective(&mut commands, i, level.objectives.len(), objective.clone());
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
pub struct LevelText {
    is_header: bool,
}

#[derive(Default)]
pub struct CurrentLevel(pub usize);

pub struct GameLevel {
    pub header: &'static str,
    pub name: &'static str,
    pub objectives: Vec<Option<Chord>>, //change this
    pub notes: Vec<Note>,
}

impl GameLevel {
    fn random_level(i: usize) -> GameLevel {
        let mut rng: rand::rngs::StdRng = SeedableRng::seed_from_u64(i as u64);

        let mut objectives = Vec::<Option<Chord>>::new();
        let mut notes = Vec::<Note>::new();

        for _ in 0..2 {
            let chord_i = rng.gen_range(0..Chord::COUNT);
            let root_i = rng.gen_range(0..12) as u8;

            let chord = Chord::from_repr(chord_i).unwrap();
            let root = Note(root_i);
            objectives.push(Some(chord));
            let chord_notes = chord.get_notes(root);
            for n in chord_notes {
                notes.push(n);
            }
        }

        let header = "";

        let name = match i % 5 {
            0 => "Levels are Random now",
            1 => "You can stop!",
            2 => "No really, you can!",
            3 => "Hey, you're getting good at this!",
            4 => "Is it sight reading, or right seeding?",

            _ => unimplemented!(),
        };

        GameLevel {
            name,
            header,
            objectives,
            notes,
        }
    }

    pub fn get_level(i: usize) -> GameLevel {
        match i {
            1 => GameLevel {
                name: "Harmonious Materials",
                header: "I",
                objectives: vec![Some(Chord::Major)],
                notes: vec![Note::C, Note::E, Note::G],
            },
            2 => GameLevel {
                header: "ii",
                name: "Piano Down a Mine Shaft",
                objectives: vec![Some(Chord::Minor)],
                notes: vec![Note::AB, Note::B, Note::C, Note::EB],
            },
            3 => GameLevel {
                header: "iii",
                name: "Interval Training",
                objectives: vec![Some(Chord::Major), Some(Chord::Major)],
                notes: vec![Note::C, Note::C, Note::E, Note::G, Note::F, Note::A],
            },
            4 => GameLevel {
                header: "IV",
                name: "Invariant Ringlet",
                objectives: vec![Some(Chord::Suspended4), Some(Chord::Minor)],
                notes: vec![Note::C, Note::C, Note::E, Note::G, Note::F, Note::A],
            },
            5 => GameLevel {
                header: "V",
                name: "Dissonant Constonants",
                objectives: vec![Some(Chord::Diminished), Some(Chord::Diminished)],
                notes: vec![Note::D, Note::B, Note::F, Note::DB, Note::GB, Note::E],
            },
            6 => GameLevel {
                header: "vio",
                name: "Auganized Chaos",
                objectives: vec![Some(Chord::Augmented), Some(Chord::Augmented)],
                notes: vec![Note::A, Note::B, Note::DB, Note::EB, Note::F, Note::G],
            },
            7 => GameLevel {
                header: "VIIΔ",
                name: "Try Tone Substitution",
                objectives: vec![Some(Chord::Dominant7), Some(Chord::Dominant7)],
                notes: vec![
                    Note::A,
                    Note::DB,
                    Note::G,
                    Note::E,
                    Note::EB,
                    Note::DB,
                    Note::G,
                    Note::BB,
                ],
            },

            8 => GameLevel {
                header: "VIII",
                name: "I'm too young to Diatonic",
                objectives: vec![Some(Chord::Major7), Some(Chord::Minor7)],
                notes: vec![
                    Note::C,
                    Note::C,
                    Note::D,
                    Note::E,
                    Note::F,
                    Note::G,
                    Note::A,
                    Note::B,
                ],
            },

            9 => GameLevel {
                header: "IX",
                name: "Chromatic Tac Toe",
                objectives: vec![None, None, None],
                notes: (0..12).map(|x| Note(x)).collect_vec(),
            },

            _ => Self::random_level(i),
        }
    }
}
