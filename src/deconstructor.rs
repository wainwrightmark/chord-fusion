use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;

use crate::orb::SHAPE_SIZE;

pub fn init_deconstructor(mut commands: Commands) {
    create_deconstructor(&mut commands, SHAPE_SIZE, Vec2 { x: 0., y: 200. }, 0.0);
}

pub fn create_deconstructor(commands: &mut Commands, shape_size: f32, position: Vec2, angle: f32) {
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
        //.insert(Sensor{})
        .insert(transform)
        .insert(Name::new("Deconstructor"));

    entity_builder.insert(crate::Deconstructor {});
}
