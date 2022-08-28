use bevy_prototype_lyon::shapes::Rectangle;

use crate::*;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_walls.after("main_setup").label("walls"));
    }
}

fn spawn_walls(mut commands: Commands) {
    let color = FIXED_OBJECT_STROKE;
    const OFFSET: f32 = crate::WALL_WIDTH / 2.0;
    const EXTRA_WIDTH: f32 = crate::WALL_WIDTH * 2.0;

    let bottom_wall_pos: Vec2 = Vec2::new(0.0, -crate::WINDOW_HEIGHT / 2.0 - OFFSET);
    let top_wall_pos: Vec2 = Vec2::new(0.0, crate::WINDOW_HEIGHT / 2.0 + OFFSET);
    let left_wall_pos: Vec2 = Vec2::new(-crate::WINDOW_WIDTH / 2.0 - OFFSET, 0.0);
    let right_wall_pos: Vec2 = Vec2::new(crate::WINDOW_WIDTH / 2.0 + OFFSET, 0.0);

    spawn_wall(
        &mut commands,
        bottom_wall_pos,
        crate::WINDOW_WIDTH + EXTRA_WIDTH,
        crate::WALL_WIDTH,
        color,
        "Bottom-Wall".to_string(),
    );
    spawn_wall(
        &mut commands,
        top_wall_pos,
        crate::WINDOW_WIDTH + EXTRA_WIDTH,
        crate::WALL_WIDTH,
        color,
        "Top-Wall".to_string(),
    );

    spawn_wall(
        &mut commands,
        left_wall_pos,
        crate::WALL_WIDTH,
        crate::WINDOW_HEIGHT,
        color,
        "Left-Wall".to_string(),
    );
    spawn_wall(
        &mut commands,
        right_wall_pos,
        crate::WALL_WIDTH,
        crate::WINDOW_HEIGHT,
        color,
        "Right-Wall".to_string(),
    );
}

fn spawn_wall(
    commands: &mut Commands,
    point: Vec2,
    width: f32,
    height: f32,
    color: Color,
    name: String,
) {
    let shape = Rectangle {
        extents: Vec2::new(width, height),
        origin: RectangleOrigin::Center,
    };
    let collider_shape = Collider::cuboid(shape.extents.x / 2.0, shape.extents.y / 2.0);

    commands
        .spawn()
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: bevy_prototype_lyon::prelude::FillMode::color(color),
                outline_mode: StrokeMode::color(color),
            },
            Transform::default(),
        ))
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(point.extend(0.0)))
        .insert(collider_shape)
        .insert(Name::new(name))
        .insert(Wall {});
}
