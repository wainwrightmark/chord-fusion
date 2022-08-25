use std::{f32::consts::TAU, ops::Range, time::Duration};

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::GeometryBuilder, shapes};
use bevy_rapier2d::prelude::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformScaleLens},
    *,
};
use rand::Rng;

use crate::{
    cluster::{Cluster, Note},
    components::NoteCircle,
};

pub const SHAPE_SIZE: f32 = 60f32;
pub const ANIMATION_SECONDS: u64 = 1;

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
            &mut Default::default(),
        );
    }
}

pub fn create_orb_near(
    commands: &mut Commands,
    shape_size: f32,
    cluster: Cluster,
    rangex: Range<f32>,
    rangey: Range<f32>,
    existing_note_circles: &mut Vec<(Entity, &NoteCircle, &GlobalTransform)>,
) {
    let mut rng = rand::thread_rng();

    let position = Vec2::new(rng.gen_range(rangex), rng.gen_range(rangey));

    let angle = rng.gen_range(0f32..std::f32::consts::TAU);

    create_orb(
        commands,
        shape_size,
        position,
        angle,
        cluster,
        existing_note_circles,
    )
}

pub fn create_orb(
    commands: &mut Commands,
    shape_size: f32,
    position: Vec2,
    angle: f32,
    cluster: Cluster,
    existing_note_circles: &mut Vec<(Entity, &NoteCircle, &GlobalTransform)>,
) {
    let collider_shape = Collider::ball(shape_size / 2.);
    let transform: Transform = Transform {
        translation: position.extend(0.0),
        rotation: Quat::from_rotation_x(angle),
        scale: Vec3::ONE,
    };

    let rbb = RigidBody::Dynamic;

    let mut entity_builder = commands.spawn();
    //let name = cluster.get_name();

    let num_children = cluster.notes.len();

    let stroke_color = if num_children > 1 {
        Color::DARK_GRAY
    } else {
        Color::rgba(1., 1., 1., 0.)
    };

    entity_builder.insert_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            center: Vec2::ZERO,
            radius: shape_size / 2.0,
        },
        bevy_prototype_lyon::prelude::DrawMode::Stroke(
            bevy_prototype_lyon::draw::StrokeMode::color(stroke_color),
        ),
        transform,
    ));

    entity_builder
        .insert(rbb)
        .insert(collider_shape);
        //.insert(Name::new(name));

    let child_scale = 1. / (num_children as f32);
    let child_distance = if num_children <= 1 {
        Vec2::ZERO
    } else {
        Vec2 {
            x: 0.,
            y: shape_size * 0.25,
        }
    };
    let child_scale_vec = Vec3 {
        x: child_scale,
        y: child_scale,
        z: 1.,
    };

    for (i, note) in cluster.notes.iter().enumerate() {
        let child_angle = (TAU * (i as f32)) / (num_children as f32);

        let child_translation = child_distance
            .rotate(Vec2::from_angle(child_angle))
            .extend(1.);

        if let Some(index) = existing_note_circles.iter().position(|x| &x.1.note == note) {
            let (entity, _, gt) = existing_note_circles.remove(index);

            let start_translation = gt.translation() - transform.translation;

            let child_transform_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_secs(ANIMATION_SECONDS),
                TransformPositionLens {
                    start: start_translation,
                    end: child_translation,
                },
            );

            let child_scale_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_secs(ANIMATION_SECONDS),
                TransformScaleLens {
                    start: gt.compute_transform().scale,
                    end: child_scale_vec,
                },
            );

            entity_builder.add_child(entity);
            entity_builder
                .commands()
                .entity(entity)
                .insert(Animator::new(Tracks::new([
                    child_transform_tween,
                    child_scale_tween,
                ])));
        } else {
            entity_builder.with_children(|f| {
                f.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        center: Vec2::ZERO,
                        radius: shape_size * 0.5,
                    },
                    bevy_prototype_lyon::prelude::DrawMode::Fill(
                        bevy_prototype_lyon::draw::FillMode::color(note.get_color()),
                    ),
                    Transform::from_translation(child_translation).with_scale(child_scale_vec),
                ))
                .insert(NoteCircle { note: *note });
            });
        }
    }

    entity_builder.insert(crate::Orb { cluster });

    entity_builder.insert(crate::Draggable {});
}
