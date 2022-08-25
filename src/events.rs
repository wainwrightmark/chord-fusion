use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DragStartEvent>()
            .add_event::<DragMoveEvent>()
            .add_event::<DragEndEvent>()
            .add_event::<NewGameEvent>()
            .add_event::<CombineEvent>();
    }
}

#[derive(Debug)]
pub struct DragStartEvent {
    pub drag_source: DragSource,
    pub position: Vec2,
}

#[derive(Debug)]
pub struct DragMoveEvent {
    pub drag_source: DragSource,
    pub new_position: Vec2,
}

#[derive(Debug)]
pub struct DragEndEvent {
    pub drag_source: DragSource,
    pub position: Option<Vec2>,
}

#[derive(Debug)]
pub struct CombineEvent(pub Vec<Entity>);

#[derive(Debug)]
pub struct DragEndWithIntersection {
    pub dragged: Entity,
    pub target: Entity,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum DragSource {
    Mouse,
    Touch { id: u64 },
}

#[derive(Debug)]
pub struct NewGameEvent {
    pub box_count_change: i32,
}
