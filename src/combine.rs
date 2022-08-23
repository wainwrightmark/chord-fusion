use bevy::prelude::*;

use crate::*;

use bevy_oddio::{
    builtins::sine::Sine,
    output::{AudioHandle, AudioSink},
};

pub struct CombinePlugin;
impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombineEvent>()
            .add_system(combine_orbs.label("combine_orbs"));
    }
}

pub fn deconstruct(
    commands: &mut Commands,
    orb: (Entity, &Transform, &Orb, Option<&PlayingSound>),
    deconstructor: (Entity, &Transform, &Deconstructor),
    audio_handles: &mut ResMut<Assets<AudioHandle<Sine>>>,
    audio_sinks: &mut ResMut<Assets<AudioSink<Sine>>>,
) {
    let midx = (orb.1.translation.x + deconstructor.1.translation.x) / 2.;
    let midy = (orb.1.translation.y + deconstructor.1.translation.y) / 2.;

    let rangex =
        (midx - SHAPE_SIZE).max(-WINDOW_WIDTH / 2.)..(midx + SHAPE_SIZE).min(WINDOW_WIDTH / 2.);
    let rangey =
        (midy - SHAPE_SIZE).max(-WINDOW_HEIGHT / 2.)..(midy + SHAPE_SIZE).min(WINDOW_HEIGHT / 2.);

    if let Some(playing1) = orb.3 {
        stop_sine(playing1.handles.clone(), audio_handles, audio_sinks);
    }

    commands.entity(orb.0).despawn_recursive();

    for &note in orb.2.cluster.notes.iter() {
        create_orb_near(
            commands,
            SHAPE_SIZE,
            note.into(),
            rangex.clone(),
            rangey.clone(),
        )
    }
}

pub fn combine_orbs(
    mut commands: Commands,
    mut er_combine: EventReader<CombineEvent>,
    orbs: Query<(
        Entity,
        &Transform,
        Option<(&Orb, Option<&PlayingSound>)>,
        Option<&Deconstructor>,
    )>,

    mut audio_handles: ResMut<Assets<AudioHandle<Sine>>>,
    mut audio_sinks: ResMut<Assets<AudioSink<Sine>>>,
) {
    for ev in er_combine.iter() {
        info!("Combine Event Received");
        if let Ok((e0, t0, orb0, deconstructor0)) = orbs.get(ev.0) {
            if let Ok((e1, t1, orb1, deconstructor1)) = orbs.get(ev.1) {
                if orb0.is_some() && orb1.is_some() {
                    let (o0, p0) = orb0.unwrap();
                    let (o1, p1) = orb1.unwrap();
                    if let Some(playing0) = p0 {
                        stop_sine(
                            playing0.handles.clone(),
                            &mut audio_handles,
                            &mut audio_sinks,
                        );
                    }

                    if let Some(playing1) = p1 {
                        stop_sine(
                            playing1.handles.clone(),
                            &mut audio_handles,
                            &mut audio_sinks,
                        );
                    }

                    commands.entity(e0).despawn_recursive();
                    commands.entity(e1).despawn_recursive();

                    let midx = (t0.translation.x + t1.translation.x) / 2.;
                    let midy = (t0.translation.y + t1.translation.y) / 2.;

                    let rangex = (midx - SHAPE_SIZE).max(-WINDOW_WIDTH / 2.)
                        ..(midx + SHAPE_SIZE).min(WINDOW_WIDTH / 2.);
                    let rangey = (midy - SHAPE_SIZE).max(-WINDOW_HEIGHT / 2.)
                        ..(midy + SHAPE_SIZE).min(WINDOW_HEIGHT / 2.);

                    let new_clusters = o0.cluster.combine(&o1.cluster);

                    for cluster in new_clusters {
                        create_orb_near(
                            &mut commands,
                            SHAPE_SIZE,
                            cluster,
                            rangex.clone(),
                            rangey.clone(),
                        )
                    }
                } else if deconstructor0.is_some() && orb1.is_some() {
                    deconstruct(
                        &mut commands,
                        (e1, t1, orb1.unwrap().0, orb1.unwrap().1),
                        (e0, t0, deconstructor0.unwrap()),
                        &mut audio_handles,
                        &mut audio_sinks,
                    )
                } else if deconstructor1.is_some() && orb0.is_some() {
                    deconstruct(
                        &mut commands,
                        (e0, t0, orb0.unwrap().0, orb0.unwrap().1),
                        (e1, t1, deconstructor1.unwrap()),
                        &mut audio_handles,
                        &mut audio_sinks,
                    )
                }
            }
        }
    }
}
