use std::f32::consts::TAU;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::TransformRotateZLens;
use bevy_tweening::Tween;
use bevy_tweening::*;
use itertools::Itertools;
use smallvec::ToSmallVec;

use crate::chord::*;
use crate::cluster::*;
use crate::*;

pub struct ObjectivePlugin;
impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_system(rotate_objectives)
            //.add_startup_system(init_objectives)
            .add_system(
                check_for_completions
                    .label("check_for_completions")
                    .after("drag_end"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                set_objective_colors.after("track_notes_playing_changes"),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_met_objectives.label("update_met_objectives"),
            );
    }
}

#[derive(Component)]
pub struct Objective {
    pub filter: Option<Chord>,
    pub is_complete: bool,
    pub is_hovered: bool,
}

#[derive(Component)]
pub struct CompletingObjective {
    pub objective: Entity,
}

fn rotate_objectives(
    mut commands: Commands,
    objectives: Query<(Entity, &Objective, &Interactable, &Transform), Changed<Interactable>>,
) {
    for (e, _, i, t) in objectives.iter() {
        if i.interacting {
            //info!("Start rot: {}", t.rotation);
            let animator = Animator::new(Tween::new(
                EaseFunction::SineIn,
                TweeningType::Loop,
                Duration::from_secs(ANIMATION_SECONDS * 20),
                TransformRotateZLens {
                    start: t.rotation.to_axis_angle().0.z,
                    end: t.rotation.to_axis_angle().0.z + (TAU * 2.0),
                },
            ));

            commands.entity(e).insert(animator);
        } else {
            commands.entity(e).remove::<Animator<Transform>>();
        }
    }
}

fn set_objective_colors(
    mut er: EventReader<NotesPlayingChangedEvent>,
    mut objectives_query: Query<(&Objective, &mut DrawMode)>,
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
        let chord_option = cluster.get_chord();

        for (objective, mut draw_mode) in objectives_query.iter_mut() {
            if !objective.is_complete {
                let excited: bool = if let Some(chord) = chord_option {
                    if let Some(filter) = objective.filter {
                        filter == chord.1
                    } else {
                        true
                    }
                } else {
                    false
                };

                if excited {
                    if draw_mode.ne(&incomplete_excited_objective_draw_mode()) {
                        *draw_mode = incomplete_excited_objective_draw_mode();
                    }
                } else if draw_mode.ne(&incomplete_objective_draw_mode()) {
                    *draw_mode = incomplete_objective_draw_mode();
                }
            }
        }
    }
}

fn check_for_completions(
    mut commands: Commands,
    mut er_dragend: EventReader<DragEndWithIntersection>,
    orbs: Query<&Orb>,
    mut objectives: Query<(&mut Objective, &mut DrawMode)>,
) {
    for event in er_dragend.iter() {
        if let Ok((mut objective, mut draw_mode)) = objectives.get_mut(event.target) {
            if !objective.is_complete {
                if let Ok(orb) = orbs.get(event.dragged) {
                    //info!("Checking Orb");
                    if let Some((_, chord)) = orb.cluster.get_chord() {
                        //info!("Orb has chord {}", chord);

                        let meets_filter = if let Some(filter) = objective.filter {
                            //info!("Objective has filter {}", filter);
                            chord == filter
                        } else {
                            true
                        };

                        if meets_filter {
                            //info!("Filter met");
                            objective.is_complete = true;
                            *draw_mode = complete_objective_draw_mode();
                            commands
                                .entity(event.dragged)
                                .insert(CompletingObjective {
                                    objective: event.target,
                                })
                                .insert(RigidBody::Fixed);
                        }
                    }
                }
            }
        }
    }
}

fn update_met_objectives(
    removals: RemovedComponents<CompletingObjective>,
    complete_objectives: Query<&CompletingObjective>,
    mut objectives: Query<(Entity, &mut Objective, &mut DrawMode)>,
) {
    if removals.iter().next().is_some() {
        for (obj, mut objective, mut draw_mode) in objectives.iter_mut() {
            if objective.is_complete && !complete_objectives.iter().any(|x| x.objective == obj) {
                objective.is_complete = false;
                *draw_mode = incomplete_objective_draw_mode();
            }
        }
    }
}

fn complete_objective_draw_mode() -> DrawMode {
    DrawMode::Outlined {
        fill_mode: bevy_prototype_lyon::prelude::FillMode::color(COMPLETE_OBJECTIVE_FILL),
        outline_mode: StrokeMode::new(FIXED_OBJECT_STROKE, 6.0),
    }
}

fn incomplete_objective_draw_mode() -> DrawMode {
    DrawMode::Outlined {
        fill_mode: bevy_prototype_lyon::prelude::FillMode::color(FIXED_OBJECT_FILL),
        outline_mode: StrokeMode::new(FIXED_OBJECT_STROKE, 3.0),
    }
}

fn incomplete_excited_objective_draw_mode() -> DrawMode {
    DrawMode::Outlined {
        fill_mode: bevy_prototype_lyon::prelude::FillMode::color(EXCITED_OBJECTIVE_FILL),
        outline_mode: StrokeMode::new(FIXED_OBJECT_STROKE, 3.0),
    }
}

pub fn create_objective(
    commands: &mut Commands,
    index: usize,
    total_number: usize,
    chord_option: Option<Chord>,
) {
    let position_x =
        (WINDOW_WIDTH * ((index + 1) as f32) / (total_number as f32 + 1.)) - (WINDOW_WIDTH * 0.5);

    let position = Vec2 {
        x: position_x,
        y: 100.,
    };

    let collider_shape = Collider::cuboid(SHAPE_SIZE / 2., SHAPE_SIZE / 2.);
    let transform: Transform = Transform {
        translation: position.extend(1.0),
        rotation: Quat::default(),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Fixed;

    let mut entity_builder = commands.spawn();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                origin: Default::default(),
                extents: Vec2 {
                    x: SHAPE_SIZE,
                    y: SHAPE_SIZE,
                },
            },
            incomplete_objective_draw_mode(),
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape)
        .insert(Sensor {})
        .insert(transform);

    entity_builder.insert(crate::Objective {
        filter: chord_option,
        is_complete: false,
        is_hovered: false,
    });

    entity_builder.insert(Interactable { interacting: false });

    if let Some(chord) = chord_option {
        let num_children = chord.intervals().len();
        let child_scale = 0.9 / (num_children as f32);
        let child_distance = if num_children <= 1 {
            Vec2::ZERO
        } else {
            Vec2 {
                x: 0.,
                y: SHAPE_SIZE * 0.25,
            }
        };
        let child_scale_vec = Vec3 {
            x: child_scale,
            y: child_scale,
            z: 1.,
        };

        for &interval in chord.intervals().iter() {
            let child_angle = (TAU * (interval as f32)) / 12.;

            let child_translation = child_distance
                .rotate(Vec2::from_angle(child_angle))
                .extend(5.);

            {
                entity_builder.with_children(|f| {
                    f.spawn_bundle(GeometryBuilder::build_as(
                        &shapes::Circle {
                            center: Vec2::ZERO,
                            radius: SHAPE_SIZE * 0.5,
                        },
                        bevy_prototype_lyon::prelude::DrawMode::Fill(
                            bevy_prototype_lyon::draw::FillMode::color(CHORD_COLOR),
                        ),
                        Transform::from_translation(child_translation).with_scale(child_scale_vec),
                    ));
                });
            }
        }
    }
}
