use crate::*;

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
    mut er_drag_end: EventReader<DragEndEvent>,
    mut dragged: Query<(Entity, &Draggable, &Dragged, &mut Transform)>,
    mut commands: Commands,
    mut ew_end_drag: EventWriter<DragEndedEvent>,
    mut ew_combine: EventWriter<CombineEvent>,
    mut ew_deconstruct: EventWriter<DeconstructEvent>,
    rapier_context: Res<RapierContext>,
) {
    for event in er_drag_end.iter() {
        dragged
            .iter_mut()
            .filter(|f| f.2.drag_source == event.drag_source)
            .for_each(|(entity, _, _, _)| {
                for contact in rapier_context
                    .contacts_with(entity)
                    .take(1)
                {                    
                    ew_combine.send(CombineEvent(contact.collider1(), contact.collider2()));
                }

                for intersection in rapier_context.intersections_with(entity).take(1){                    
                    ew_deconstruct.send(DeconstructEvent(intersection.1)); //TODO also track victory conditions this way
                }

                commands
                    .entity(entity)
                    .remove::<Dragged>()
                    .remove::<RigidBody>()
                    .insert(RigidBody::Dynamic);

                ew_end_drag.send(DragEndedEvent {});
            });
    }
}

fn drag_move(
    mut er_drag_move: EventReader<DragMoveEvent>,
    mut dragged_entities: Query<(&Dragged, &mut Transform)>,
) {
    for event in er_drag_move.iter() {
        if let Some((dragged, mut rb)) = dragged_entities
            .iter_mut()
            .find(|d| d.0.drag_source == event.drag_source)
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
                    .insert(RigidBody::KinematicPositionBased);

                return false;
            }
            true
        });
    }
}
