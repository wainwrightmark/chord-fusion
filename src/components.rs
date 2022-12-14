use bevy::prelude::*;

use crate::{
    cluster::{Cluster, Note},
    events::*,
};

#[derive(Component)]
pub struct RestartButton {}

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Orb {
    pub cluster: Cluster,
}

#[derive(Component)]
pub struct Interactable {
    pub interacting: bool,
}

#[derive(Component)]
pub struct NoteCircle {
    pub note: Note,
}

#[derive(Component)]
pub struct Deconstructor {}

#[derive(Component)]
pub struct Draggable {}

#[derive(Component)]
pub struct Dragged {
    pub origin: Vec3,
    pub offset: Vec3,
    pub drag_source: DragSource,
}

#[derive(Component)]
pub struct WinTimer {
    pub win_time: f64,
}

// #[derive(Component)]
// pub struct Foundation {}

#[derive(Component)]
pub struct Wall {}

#[derive(Component)]
pub enum Shelf {
    Left,
    Middle,
    Right,
}
