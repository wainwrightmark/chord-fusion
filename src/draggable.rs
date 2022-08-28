use crate::*;
use bevy::utils::HashSet;
use itertools::*;

pub struct DragPlugin;
impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            drag_start.label("drag_start").after("mousebutton_listener"), //.after("touch_listener"),
        )
        .add_system(
            drag_move.label("drag_move").after("mousebutton_listener"), //.after("touch_listener"),
        )
        .add_system(
            drag_end.label("drag_end").after("mousebutton_listener"), //.after("touch_listener"),
        );
    }
}

fn drag_end(
    mut commands: Commands,

    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(Entity, &Draggable, &Dragged, &mut Transform)>,

    mut ew_combine: EventWriter<CombineEvent>,
    mut ew_deconstruct: EventWriter<DragEndWithIntersection>,
    rapier_context: Res<RapierContext>,
) {
    for event in er_drag_end.iter() {
        dragged
            .iter_mut()
            .filter(|f| f.2.drag_source == event.drag_source)
            .for_each(|(entity, _, _, _)| {
                let all_contacts = std::iter::once(entity)
                    .chain(
                        rapier_context
                            .contacts_with(entity)
                            .filter(|x| x.has_any_active_contacts())
                            .flat_map(|x| [x.collider1(), x.collider2()]),
                    )
                    .sorted()
                    .dedup()
                    .collect_vec();

                if all_contacts.len() > 1 {
                    ew_combine.send(CombineEvent(all_contacts));
                } else if let Some(point) = event.position {
                    rapier_context.intersections_with_point(
                        point,
                        QueryFilter::exclude_collider(
                            QueryFilter::exclude_solids(QueryFilter::default()),
                            entity,
                        ),
                        |e| {
                            ew_deconstruct.send(DragEndWithIntersection {
                                dragged: entity,
                                target: e,
                            });
                            false
                        },
                    );
                }

                commands
                    .entity(entity)
                    .remove::<Dragged>()
                    .remove::<RigidBody>()
                    .insert(RigidBody::Dynamic);
            });
    }
}

fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(Entity, &Dragged, &mut Transform)>,
    rapier_context: Res<RapierContext>,
    mut interactables: Query<(Entity, &mut Interactable, Option<&Dragged>)>,
) {
    for event in er_drag_move.iter() {
        if let Some((entity, dragged, mut rb)) = dragged_entities
            .iter_mut()
            .find(|d| d.1.drag_source == event.drag_source)
        {
            let max_x: f32 = crate::WINDOW_WIDTH / 2.0; //You can't leave the game area
            let max_y: f32 = crate::WINDOW_HEIGHT / 2.0;

            let min_x: f32 = -max_x;
            let min_y: f32 = -max_y;

            let clamped_position = bevy::math::Vec2::clamp(
                event.new_position,
                Vec2::new(min_x, min_y),
                Vec2::new(max_x, max_y),
            );

            let new_position = dragged.offset + clamped_position.extend(0.0); // clamped_position;

            rb.translation = new_position;

            let mut remaining_undragged_actors: HashSet<_> = interactables
                .iter_mut()
                .filter(|x| x.1.interacting && x.2.is_none())
                .map(|(e, _, _)| e)
                .collect();

            let all_contacts = rapier_context
                .contacts_with(entity)
                .filter(|x| x.has_any_active_contacts())
                .flat_map(|x| [x.collider1(), x.collider2()])
                .sorted()
                .dedup()
                .collect_vec();

            for c_entity in all_contacts {
                if c_entity != entity && !remaining_undragged_actors.remove(&c_entity) {
                    //this entity was not previously interacting
                    if let Ok((_, mut interactable, _)) = interactables.get_mut(c_entity) {
                        //info!("draggable interacting");
                        interactable.interacting = true;
                    }
                }
            }
            for rem in remaining_undragged_actors {
                //This entity is not in contact but is playing sound, remove the thingy
                if let Ok((_, mut interactable, _)) = interactables.get_mut(rem) {
                    //info!("draggable no longer interacting");
                    interactable.interacting = false;
                }
            }
        }
    }
}

fn drag_start(
    mut commands: Commands,
    mut er_drag_start: EventReader<DragStartEvent>,
    rapier_context: Res<RapierContext>,
    draggables: Query<(&Draggable, &Transform)>,
) {
    for event in er_drag_start.iter() {
        rapier_context.intersections_with_point(event.position, default(), |entity| {
            if let Ok((_, rb)) = draggables.get(entity) {
                let origin = rb.translation;
                let offset = origin - event.position.extend(0.0);

                commands
                    .entity(entity)
                    .insert(Dragged {
                        origin,
                        offset,
                        drag_source: event.drag_source,
                    })
                    .remove::<RigidBody>()
                    .remove::<CompletingObjective>()
                    .insert(RigidBody::KinematicPositionBased);

                return false;
            }
            true
        });
    }
}
