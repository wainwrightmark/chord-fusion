use bevy::prelude::*;
use itertools::Itertools;

use crate::*;

pub struct CombinePlugin;
impl Plugin for CombinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombineEvent>()
            .add_system(combine_orbs.label("combine_orbs"));
    }
}

fn combine_orbs(
    mut commands: Commands,
    mut er_combine: EventReader<CombineEvent>,
    orbs: Query<(Entity, &Transform, &Orb, &Children)>,
    note_circles: Query<(Entity, &NoteCircle, &GlobalTransform)>,
) {
    for ev in er_combine.iter() {
        if let Ok((e0, t0, o0, children0)) = orbs.get(ev.0) {
            if let Ok((e1, t1, o1, children1)) = orbs.get(ev.1) {
                let mut note_circles = children0
                    .iter()
                    .chain(children1.iter())
                    .filter_map(|&e| note_circles.get(e).ok())
                    .collect_vec();

                let new_clusters = o0.cluster.combine(&o1.cluster);

                let mut first = true;

                for cluster in new_clusters {
                    if first {
                        create_orb(
                            &mut commands,
                            SHAPE_SIZE,
                            t0.translation.truncate(),
                            t0.rotation.angle_between(Default::default()),
                            cluster,
                            &mut note_circles,
                        );

                        first = false;
                    } else {
                        let midx = (t0.translation.x + t1.translation.x) / 2.;
                        let midy = (t0.translation.y + t1.translation.y) / 2.;

                        let rangex = (midx - SHAPE_SIZE).max(-WINDOW_WIDTH / 2.)
                            ..(midx + SHAPE_SIZE).min(WINDOW_WIDTH / 2.);
                        let rangey = (midy - SHAPE_SIZE).max(-WINDOW_HEIGHT / 2.)
                            ..(midy + SHAPE_SIZE).min(WINDOW_HEIGHT / 2.);
                        create_orb_near(
                            &mut commands,
                            SHAPE_SIZE,
                            cluster,
                            rangex.clone(),
                            rangey.clone(),
                            &mut note_circles,
                        )
                    }
                }

                commands.entity(e0).despawn();
                commands.entity(e1).despawn();
            }
        }
    }
}
