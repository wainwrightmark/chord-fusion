use std::fmt::Debug;

use bevy::prelude::Color;
use bevy_prototype_lyon::prelude::*;
use itertools::Itertools;
use smallvec::*;

use crate::*;
use crate::chord::*;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cluster {
    pub notes: SmallVec<[Note; 4]>,
}

impl From<Note> for Cluster {
    fn from(val: Note) -> Self {
        Cluster {
            notes: smallvec![val],
        }
    }
}

impl Cluster {

    pub fn get_draw_mode(&self, playing: bool) -> DrawMode{
        if self.notes.len()== 1{

            if playing {
                DrawMode::Fill(FillMode::color(self.notes[0].get_dark_color()))
            }
            else{
                DrawMode::Fill(FillMode::color(Color::NONE))
            }

            
        }
        else{
            if playing {
                DrawMode::Fill(FillMode::color(CHORD_COLOR))
            }
            else{                
                DrawMode::Stroke(StrokeMode::color(CHORD_COLOR))
            }
        }
    }

    pub fn get_chord_name(&self) -> Option<String> {
        if self.notes.len() == 3 {
            let sorted_notes = self.notes.iter().map(|&x| x.0).sorted();
            let arr: [u8; 3] = sorted_notes.collect_vec().try_into().unwrap();

            for i in 0..3 {
                let a1 = Self::permute(arr, i);
                let chord_option = Chord3::all().get(&a1);

                if let Some(chord) = chord_option {
                    return Some(format!("{}{}", self.notes[i].get_name(), chord.nice_name()));
                }
            }
        } else if self.notes.len() == 4 {
            let sorted_notes = self.notes.iter().map(|&x| x.0).sorted();
            let arr: [u8; 4] = sorted_notes.collect_vec().try_into().unwrap();

            for i in 0..4 {
                let a1 = Self::permute(arr, i);
                let chord_option = Chord4::all().get(&a1);

                if let Some(chord) = chord_option {
                    return Some(format!("{}{}", self.notes[i].get_name(), chord.nice_name()));
                }
            }
        }

        return None;
    }

    fn permute<const L: usize>(notes: [u8; L], index: usize) -> [u8; L] {
        let mut new_notes = notes.clone();
        new_notes.rotate_left(index);
        for i in 0..L {
            new_notes[i] = ((new_notes[i] + 12) - notes[index]) % 12;
        }

        new_notes
    }
}

impl Cluster {
    pub fn get_notes_text(&self) -> String {
        self.notes.iter().map(|x| x.get_name()).join(" ")
    }

    ///Combine many clusters
    pub fn combine(clusters: &Vec<Self>) -> Vec<Cluster> {
        let all_notes = clusters.iter().flat_map(|x| x.notes.clone()).counts();

        let mut all_clusters = Vec::<Cluster>::new();

        let mut main_cluster = Vec::<Note>::new();

        for (&note, &count) in all_notes.iter() {
            if count == 1 {
                main_cluster.push(note);
            } else {
                for _ in 0..count {
                    all_clusters.push(note.into());
                }
            }
        }

        if main_cluster.len() >= 12 {
            for n in main_cluster {
                all_clusters.push(n.into());
            }
        } else {
            all_clusters.push(Cluster {
                notes: main_cluster.to_smallvec(),
            })
        }

        all_clusters
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Note(pub u8);

const fn create_note(i: usize) -> Note {
    Note(i as u8)
}

impl Note {
    pub const ALL_NOTES: [Note; 12] = array_const_fn_init::array_const_fn_init![create_note; 12];

    pub fn get_frequency(self) -> f32 {
        const FREQUENCY_C: f32 = 261.62;
        match self.0 % 12 {
            0 => FREQUENCY_C * 1.0,
            1 => FREQUENCY_C * 16. / 15.,
            2 => FREQUENCY_C * 9. / 8.,
            3 => FREQUENCY_C * 6. / 5.,
            4 => FREQUENCY_C * 5. / 4.,
            5 => FREQUENCY_C * 4. / 3.,
            6 => FREQUENCY_C * 64. / 45.,
            7 => FREQUENCY_C * 3. / 2.,
            8 => FREQUENCY_C * 8. / 5.,
            9 => FREQUENCY_C * 5. / 3.,
            10 => FREQUENCY_C * 16. / 9.,
            11 => FREQUENCY_C * 15. / 8.,
            _ => unimplemented!(),
        }
    }
    pub fn get_name(self) -> &'static str {
        match self.0 % 12 {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => unimplemented!(),
        }
    }

    pub fn get_color(self) -> Color {
        let hue = (210. * (self.0 as f32)) % 360.;
        Color::Hsla {
            hue,
            saturation: 0.7,
            lightness: 0.8,
            alpha: 0.8,
        }
    }
    
    pub fn get_dark_color(self) -> Color {
        let hue = 30.0 * (self.0 as f32);
        Color::Hsla {
            hue,
            saturation: 0.7,
            lightness: 0.4,
            alpha: 0.8,
        }
    }
}

impl Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
