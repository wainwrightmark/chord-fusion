use bevy::prelude::*;
use itertools::Itertools;
use smallvec::ToSmallVec;

use crate::chord::*;
use crate::cluster::*;
use crate::*;

pub struct ObjectivePlugin;
impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app
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
}

#[derive(Component)]
pub struct CompletingObjective {
    pub objective: Entity,
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
                } else {
                    if draw_mode.ne(&incomplete_objective_draw_mode()) {
                        *draw_mode = incomplete_objective_draw_mode();
                    }
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
                    let meets_filter = if let Some(filter) = objective.filter {
                        if let Some((_, chord_name)) = orb.cluster.get_chord() {
                            chord_name == filter
                        } else {
                            false
                        }
                    } else {
                        orb.cluster.get_chord().is_some()
                    };

                    if meets_filter {
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

fn update_met_objectives(
    removals: RemovedComponents<CompletingObjective>,
    complete_objectives: Query<&CompletingObjective>,
    mut objectives: Query<(Entity, &mut Objective, &mut DrawMode)>,
) {
    if removals.iter().next().is_some() {
        for (obj, mut objective, mut draw_mode) in objectives.iter_mut() {
            if objective.is_complete {
                if !complete_objectives.iter().any(|x| x.objective == obj) {
                    objective.is_complete = false;
                    *draw_mode = incomplete_objective_draw_mode();
                }
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
    _chord_option: Option<Chord>,
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
        filter: None,
        is_complete: false,
    });
}
