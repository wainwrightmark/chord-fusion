use std::f32::consts::TAU;

use bevy_prototype_lyon::shapes::Rectangle;

use crate::*;

pub struct ShelfPlugin;

impl Plugin for ShelfPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_shelves.after("main_setup").label("spawn_shelves"));
    }
}

fn spawn_shelves(mut commands: Commands) {
    let color = FIXED_OBJECT_STROKE;
    // const OFFSET: f32 = crate::WALL_WIDTH / 2.0;
    const CENTRE_WIDTH: f32 = crate::WINDOW_WIDTH * 0.25;
    const HEIGHT: f32 = 10.;

    // let bottom_wall_pos: Vec2 = Vec2::new(0.0, -crate::WINDOW_HEIGHT / 2.0 - OFFSET);
    // let top_wall_pos: Vec2 = Vec2::new(0.0, crate::WINDOW_HEIGHT / 2.0 + OFFSET);
    // let left_wall_pos: Vec2 = Vec2::new(-crate::WINDOW_WIDTH / 2.0 - OFFSET, 0.0);
    // let right_wall_pos: Vec2 = Vec2::new(crate::WINDOW_WIDTH / 2.0 + OFFSET, 0.0);

    spawn_shelf(
        &mut commands,
        RectangleOrigin::Center,
        Vec2 { x: 0., y: -100. },
        CENTRE_WIDTH,
        HEIGHT,
        color,
        0.,
        Shelf::Middle
    );
    
    spawn_shelf(
        &mut commands,
        RectangleOrigin::TopRight,
        Vec2 { x: -CENTRE_WIDTH * 0.5, y: -100. },
        CENTRE_WIDTH,
        HEIGHT,
        color,
        -TAU / 12.,
        Shelf::Left
    );
    
    spawn_shelf(
        &mut commands,
        RectangleOrigin::TopLeft,
        Vec2 { x: CENTRE_WIDTH * 0.5, y: -100. },
        CENTRE_WIDTH,
        HEIGHT,
        color,
        TAU / 12.,
        Shelf::Right
    );

}

fn spawn_shelf(
    commands: &mut Commands,
    origin: RectangleOrigin,
    point: Vec2,
    width: f32,
    height: f32,
    color: Color,
    angle: f32,
    shelf: Shelf
) {
    let shape = Rectangle {
        extents: Vec2::new(width, height),
        origin,
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
        .insert(Transform::from_translation(point.extend(0.0)).with_rotation(Quat::from_rotation_z(angle)))
        .insert(collider_shape.clone())
        .insert(shelf);
}
