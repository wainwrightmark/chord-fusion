#![allow(dead_code)]
use bevy::DefaultPlugins;
use bevy_oddio::AudioPlugin;

use bevy::log::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

pub const WINDOW_WIDTH: f32 = 360f32;
pub const WINDOW_HEIGHT: f32 = 640f32;
pub const WALL_WIDTH: f32 = 360f32;

mod draggable;
use draggable::*;
mod events;
use events::*;
mod components;
use components::*;
mod walls;
use walls::*;
mod cluster;

mod sound;
use sound::*;

mod input;
use input::*;

mod orb;
use orb::*;

mod hover;
use hover::*;

mod combine;
use combine::*;

mod deconstructor;
use deconstructor::*;

fn main() {
    App::new()
        .insert_resource(LogSettings {
            level: Level::INFO,
            ..Default::default()
        })
        .insert_resource(WindowDescriptor {
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#game".to_string()),
            title: "Chord Fusion".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.95)))
        .add_plugins(DefaultPlugins)
        .add_plugin(WallsPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(InputPlugin)
        .add_plugin(EventsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            WINDOW_HEIGHT / 10.0,
        ))
        .add_startup_system(setup.label("main_setup"))
        .add_plugin(AudioPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(DragPlugin)
        .add_plugin(HoverPlugin)
        .add_plugin(CombinePlugin)
        .add_plugin(DeconstructPlugin)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_initial_orbs)
        .run();
}

fn setup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0.0, -1000.0);

    commands
        .spawn()
        .insert_bundle(Camera2dBundle::default())
        .insert(MainCamera);
}
