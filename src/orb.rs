use std::{f32::consts::TAU, ops::Range};

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::cluster::{Cluster, Note};

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_initial_orbs(mut commands: Commands) {
    let start_clusters = [
        Note(0).into(),
        Note(2).into(),
        Note(4).into(),
        Note(5).into(),
        Note(7).into(),
        Note(9).into(),
        Note(11).into(),
    ];

    let rangex = -100f32..100f32;
    let rangey = -100f32..100f32;

    for cluster in start_clusters {
        create_orb_near(
            &mut commands,
            SHAPE_SIZE,
            cluster,
            rangex.clone(),
            rangey.clone(),
        );
    }
}

pub fn create_orb_near(
    commands: &mut Commands,
    shape_size: f32,
    cluster: Cluster,
    rangex: Range<f32>,
    rangey: Range<f32>,
) {
    let mut rng = rand::thread_rng();

    let position = Vec2::new(rng.gen_range(rangex), rng.gen_range(rangey));

    let angle = rng.gen_range(0f32..std::f32::consts::TAU);

    create_orb(commands, shape_size, position, angle, cluster)
}

pub fn create_orb(
    commands: &mut Commands,
    shape_size: f32,
    position: Vec2,
    angle: f32,
    cluster: Cluster,
) {
    let collider_shape = Collider::ball(shape_size / 2.);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Dynamic;

    let mut entity_builder = commands.spawn();
    let name = cluster.get_name();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                center: Vec2::ZERO,
                radius: shape_size / 2.0,
            },
            bevy_prototype_lyon::prelude::DrawMode::Fill(
                bevy_prototype_lyon::draw::FillMode::color(Color::GRAY),
            ),
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape.clone())
        .insert(transform)
        .insert(Name::new(name.clone()));

    let child_radius = (shape_size * 0.5 * 0.9) / (cluster.notes.len() as f32);
    let child_distance = if cluster.notes.len() <= 1 {
        Vec2::ZERO
    } else {
        Vec2 {
            x: 0.,
            y: shape_size * 0.25,
        }
    };

    entity_builder.with_children(|f| {
        for (i, note) in cluster.notes.iter().enumerate() {
            let child_angle = (TAU * (i as f32)) / (cluster.notes.len() as f32);
            f.spawn_bundle(GeometryBuilder::build_as(
                &shapes::Circle {
                    center: Vec2::ZERO,
                    radius: child_radius,
                },
                bevy_prototype_lyon::prelude::DrawMode::Fill(
                    bevy_prototype_lyon::draw::FillMode::color(note.get_color()),
                ),
                Transform::from_translation(
                    child_distance
                        .rotate(Vec2::from_angle(child_angle))
                        .extend(1.),
                ),
            ));
        }
    });

    entity_builder.insert(crate::Orb { cluster: cluster });

    entity_builder.insert(crate::Draggable {});
}
