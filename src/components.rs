use bevy::prelude::*;

use crate::{cluster::Cluster, events::*};

use bevy_oddio::{
    builtins::sine::{self, Sine},
    output::{AudioHandle, AudioSink},
    Audio,
};
use oddio::Sample;

#[derive(Component)]
pub struct RestartButton {}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Draggable {
    pub cluster: Cluster,
}

#[derive(Component)]
pub struct Dragged {
    pub origin: Vec3,
    pub offset: Vec3,
    pub drag_source: DragSource,
    pub handles: (Handle<AudioHandle<Sine>>, Handle<AudioSink<Sine>>),
}

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

// #[derive(Component)]
// pub struct Foundation {}

#[derive(Component)]
pub struct Wall {}