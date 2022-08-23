use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;
use itertools::Itertools;
use rand::Rng;
use smallvec::smallvec;

use crate::cluster::{Cluster, Note};

pub const SHAPE_SIZE: f32 = 60f32;

pub fn create_initial_shapes(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let start_clusters = [
        Note(0).into(),
        Note(4).into(),
        Note(7).into(),
        Cluster {
            notes: smallvec![Note(0), Note(4)],
        },
        Cluster {
            notes: smallvec![Note(0), Note(7)],
        },
        Cluster {
            notes: smallvec![Note(0), Note(4), Note(7)],
        },
        Cluster {
            notes: smallvec![Note(11), Note(2), Note(7)],
        },
    ];

    for cluster in start_clusters {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        let point = Vec2::new(rng.gen_range(rangex), rng.gen_range(rangey));

        let angle = rng.gen_range(0f32..std::f32::consts::TAU);

        create_orb(&mut commands, SHAPE_SIZE, point, angle, cluster);
    }
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

    let child_radius = (shape_size * 0.5 * 0.9) / (cluster.notes.len() as f32);
    let child_distance = if cluster.notes.len() <= 1 {
        Vec2::ZERO
    } else {
        Vec2 {
            x: 0.,
            y: shape_size * 0.25,
        }
    };

    let children =
    cluster
        .notes
        .iter()
        .enumerate()
        .map(|(i, note)| {
            let child_angle = (TAU * (i as f32)) / (cluster.notes.len() as f32);
            let child = commands
                .spawn_bundle(GeometryBuilder::build_as(
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
                ))
                .id();
            child
        })
        .collect_vec();

    let mut entity_builder = commands.spawn();
    let name = cluster.get_name();

    entity_builder
        .insert_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                center: Vec2::ZERO,
                radius: shape_size / 2.0,
            },
            bevy_prototype_lyon::prelude::DrawMode::Stroke(
                bevy_prototype_lyon::draw::StrokeMode::color(Color::DARK_GRAY),
            ), // bevy_prototype_lyon::prelude::DrawMode::Fill(
            //     bevy_prototype_lyon::draw::FillMode::color(note.get_color()),
            // )
            Transform::default(),
        ))
        .insert(rbb)
        .insert(collider_shape)
        .insert(transform)
        .insert(Name::new(name));

    entity_builder.insert(crate::Orb { cluster: cluster });

    entity_builder.insert(crate::Draggable {});

    for child in children{
        entity_builder.add_child(child);
    }
}
