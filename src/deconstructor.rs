use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;
use itertools::Itertools;

use crate::*;

pub struct DeconstructPlugin;
impl Plugin for DeconstructPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_deconstructor)
            .add_event::<DragEndWithIntersection>()
            .add_system(deconstruct.label("deconstruct"));
    }
}

fn deconstruct(
    mut commands: Commands,
    mut er_deconstruct: EventReader<DragEndWithIntersection>,
    orbs: Query<(Entity, &Transform, &Orb, &Children)>,
    note_circles: Query<(Entity, &NoteCircle, &GlobalTransform)>,
) {
    for ev in er_deconstruct.iter() {
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

fn init_deconstructor(mut commands: Commands) {
    create_deconstructor(&mut commands, SHAPE_SIZE, Vec2 { x: 0., y: 200. }, 0.0);
}

fn create_deconstructor(commands: &mut Commands, shape_size: f32, position: Vec2, angle: f32) {
    let collider_shape = Collider::cuboid(shape_size / 2., shape_size / 2.);
    let transform: Transform = Transform {
        translation: position.extend(1.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Fixed;

    let mut entity_builder = commands.spawn();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Rectangle {
                origin: Default::default(),
                extents: Vec2 {
                    x: shape_size,
                    y: shape_size,
                },
            },
            bevy_prototype_lyon::prelude::DrawMode::Stroke(
                bevy_prototype_lyon::draw::StrokeMode::color(Color::DARK_GRAY),
            ),
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape)
        .insert(Sensor {})
        .insert(transform)
        .insert(Name::new("Deconstructor"));

    entity_builder.insert(crate::Deconstructor {});
}
