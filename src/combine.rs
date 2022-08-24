use bevy::prelude::*;
use itertools::Itertools;

use crate::cluster::*;
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
        let groups = ev.0.iter().filter_map(|&e| orbs.get(e).ok()).collect_vec();

        if groups.len() > 1 {
            let mut note_circles = groups
                .iter()
                .flat_map(|x| x.3)
                .filter_map(|&e| note_circles.get(e).ok())
                .collect_vec();

            let new_clusters =
                Cluster::combine(&groups.iter().map(|x| x.2.cluster.clone()).collect_vec());

            let first_entity = groups[0];

            let mut first = true;

            for cluster in new_clusters {
                if first {
                    create_orb(
                        &mut commands,
                        SHAPE_SIZE,
                        first_entity.1.translation.truncate(),
                        first_entity.1.rotation.angle_between(Default::default()),
                        cluster,
                        &mut note_circles,
                    );

                    first = false;
                } else {
                    let minx = groups
                        .iter()
                        .map(|x| x.1.translation.x)
                        .reduce(f32::min)
                        .unwrap();
                    let miny = groups
                        .iter()
                        .map(|x| x.1.translation.y)
                        .reduce(f32::min)
                        .unwrap();
                    let maxx = groups
                        .iter()
                        .map(|x| x.1.translation.x)
                        .reduce(f32::max)
                        .unwrap();
                    let maxy = groups
                        .iter()
                        .map(|x| x.1.translation.y)
                        .reduce(f32::max)
                        .unwrap();

                    let rangex = (minx - SHAPE_SIZE).max(-WINDOW_WIDTH / 2.)
                        ..(maxx + SHAPE_SIZE).min(WINDOW_WIDTH / 2.);
                    let rangey = (miny - SHAPE_SIZE).max(-WINDOW_HEIGHT / 2.)
                        ..(maxy + SHAPE_SIZE).min(WINDOW_HEIGHT / 2.);
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

            for (e, _, _, _) in groups {
                commands.entity(e).despawn();
            }
        }
    }
}
