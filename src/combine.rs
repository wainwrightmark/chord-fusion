use bevy::prelude::*;

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
    orbs: Query<(Entity, &Transform, &Orb)>,
) {
    for ev in er_combine.iter() {
        if let Ok((e0, t0, o0)) = orbs.get(ev.0) {
            if let Ok((e1, t1, o1)) = orbs.get(ev.1) {
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
            }
        }
    }
}
