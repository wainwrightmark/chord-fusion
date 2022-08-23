use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::cluster::Note;

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_initial_shapes(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for note in Note::ALL_NOTES {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        let point = Vec2::new(rng.gen_range(rangex), rng.gen_range(rangey));

        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_shape(&mut commands, SHAPE_SIZE, point, angle, note);
    }
}

pub fn create_shape(
    commands: &mut Commands,
    shape_size: f32,
    position: Vec2,
    angle: f32,
    note: Note,
) {
    let collider_shape = Collider::ball(shape_size / 2.);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Dynamic;

    let mut entity_builder = commands.spawn();
    let name = note.get_name();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                center: Vec2::ZERO,
                radius: shape_size / 2.0,
            },
            bevy_prototype_lyon::prelude::DrawMode::Fill(
                bevy_prototype_lyon::draw::FillMode::color(note.get_color()),
            ),
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape)
        .insert(transform)
        .insert(Name::new(name));

        entity_builder.insert(crate::Orb{
            cluster: note.into(),
        });

    entity_builder.insert(crate::Draggable {
        
    });
}
