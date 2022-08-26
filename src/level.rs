use bevy::prelude::*;

use crate::cluster::*;
use crate::objective::*;
use crate::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentLevel>()
        .add_system(check_for_completion.after("update_met_objectives"))
            .add_startup_system_to_stage(StartupStage::PostStartup, start_next_level);
    }
}

fn check_for_completion(
    mut commands: Commands,
    added_completions: Query<Added<CompletingObjective>>,
    objectives: Query<(Entity, &Objective)>,
    orbs: Query<(Entity, &Orb)>,
    current_level: ResMut<CurrentLevel>,
) {

        if !added_completions.is_empty(){

            if objectives.iter().all(|o|o.1.is_complete){
                for (e, _) in objectives.iter(){
                    commands.entity(e).despawn_recursive();
                }
                for (e, _) in orbs.iter(){
                    commands.entity(e).despawn_recursive();
                }

                start_next_level(commands, current_level)
            }
        }


}

fn start_next_level(mut commands: Commands, mut current_level: ResMut<CurrentLevel>) {
    
    current_level.0 += 1;
    let level = GameLevel::get_level(current_level.0);

    for i in 0..level.objectives {
        create_objective(&mut commands, i, level.objectives);
    }

    for n in level.notes {
        let rangex = -100f32..100f32;
        let rangey = -100f32..100f32;

        create_orb_near(
            &mut commands,
            SHAPE_SIZE,
            Cluster {
                notes: smallvec::smallvec![n],
            },
            rangex,
            rangey,
            &mut Default::default(),
        )
    }
    
}

#[derive(Default)]
pub struct CurrentLevel(pub usize);

pub struct GameLevel {
    pub name: &'static str,
    pub objectives: usize, //change this
    pub notes: Vec<Note>,
}

impl GameLevel {
    pub fn get_level(i: usize) -> GameLevel {
        match i % 8{
            0 => GameLevel { name: "Octave", objectives: 3, notes: Note::ALL_NOTES.into() },
            1 => GameLevel { name: "First Inversion", objectives: 1, notes: vec![Note(0), Note(4), Note(7)] },
            2 => GameLevel { name: "Major Second", objectives: 2, notes: vec![Note(0), Note(4), Note(7), Note(1), Note(5), Note(8)] },
            3 => GameLevel { name: "Piano down a mine shaft", objectives: 1, notes: vec![Note(8), Note(3), Note(11)] },
            4 => GameLevel { name: "It goes like this", objectives: 2, notes: vec![Note(0), Note(2), Note(4), Note(5), Note(7), Note(11)] },
            5 => GameLevel { name: "Diminished Fifth", objectives: 2, notes: vec![Note(0), Note(3), Note(6), Note(1), Note(4), Note(7)] },
            6 => GameLevel { name: "Augmented Sense of Self Worth", objectives: 2, notes: vec![Note(0), Note(4), Note(8), Note(1), Note(5), Note(9)] },
            7 => GameLevel { name: "Seventh Heaven", objectives: 2, notes: vec![Note(0), Note(4), Note(7), Note(10), Note(1), Note(4), Note(7), Note(11)] },


            _=> unimplemented!()
        }
    }
}
