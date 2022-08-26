use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::*;

pub struct DeconstructPlugin;
impl Plugin for DeconstructPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_deconstructor).add_system(
            check_for_deconstructors
                .label("check_for_deconstructors")
                .after("drag_end"),
        );
    }
}

fn check_for_deconstructors(
    mut commands: Commands,
    mut er_dragend: EventReader<DragEndWithIntersection>,
    orbs: Query<(Entity, &Transform, &Orb, &Children)>,
    note_circles: Query<(Entity, &NoteCircle, &GlobalTransform)>,
    deconstructors: Query<&Deconstructor>,
) {
    for ev in er_dragend.iter() {
        if deconstructors.contains(ev.target) {
            if let Ok((e, t, o, children)) = orbs.get(ev.dragged) {
                if o.cluster.notes.len() > 1 {
                    let rangex = (t.translation.x - SHAPE_SIZE).max(-WINDOW_WIDTH / 2.)
                        ..(t.translation.x + SHAPE_SIZE).min(WINDOW_WIDTH / 2.);
                    let rangey = (t.translation.y - SHAPE_SIZE).max(-WINDOW_HEIGHT / 2.)
                        ..(t.translation.y + SHAPE_SIZE).min(WINDOW_HEIGHT / 2.);

                    let mut note_circles = children
                        .iter()
                        .filter_map(|&e| note_circles.get(e).ok())
                        .collect_vec();

                    for &note in o.cluster.notes.iter() {
                        create_orb_near(
                            &mut commands,
                            SHAPE_SIZE,
                            note.into(),
                            rangex.clone(),
                            rangey.clone(),
                            &mut note_circles,
                        )
                    }

                    commands.entity(e).despawn();
                }
            }
        }
    }
}

fn init_deconstructor(mut commands: Commands) {
    let collider_shape = Collider::ball(WINDOW_WIDTH / 2.);
    // Collider::convex_hull(&geo.points.iter().map(|v| Vect::new(v.x, v.y)).collect_vec())
    //     .unwrap();
    let transform: Transform = Transform {
        translation: Vec2 {
            x: 0.,
            y: WINDOW_HEIGHT * 0.66,
        }
        .extend(1.0),
        rotation: Default::default(),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Fixed;

    let mut entity_builder = commands.spawn();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: WINDOW_WIDTH / 2.,
                center: Default::default(),
            },
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(Color::BLACK),
                outline_mode: StrokeMode::new(FIXED_OBJECT_STROKE, 3.0),
            },
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape)
        .insert(Sensor {})
        .insert(transform)
        .insert(Name::new("Deconstructor"));

    entity_builder.insert(crate::Deconstructor {});
}
