use std::fmt::Debug;

use bevy::prelude::Color;
use smallvec::SmallVec;


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Cluster{

    Single(Note),
    Many(SmallVec<[Note;4]>)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note(u8);

const fn create_note(i: usize) -> Note {
    Note(i as u8)
}

impl Note{

    pub const ALL_NOTES : [Note; 12] = array_const_fn_init::array_const_fn_init![create_note; 12];

    pub fn get_frequency(self)-> f32{
        const FREQUENCY_C : f32 = 130.81;
        match self.0 % 12{
            0 => FREQUENCY_C * 1.0,
            1 => FREQUENCY_C * 16. / 15.,
            2 => FREQUENCY_C * 9. / 8.,
            3 => FREQUENCY_C * 6. /5.,
            4 => FREQUENCY_C * 5. /4.,
            5 => FREQUENCY_C * 4. /3.,
            6 => FREQUENCY_C * 64. /45.,
            7 => FREQUENCY_C * 3. / 2.,
            8 => FREQUENCY_C * 8. /5.,
            9 => FREQUENCY_C * 5./3.,
            10 => FREQUENCY_C * 16./9.,
            11 => FREQUENCY_C * 15./8.,
            _=> unimplemented!()
        }
    }
    pub fn get_name(self)-> &'static str{
        match self.0 % 12{
            0 => "A",
            1 => "A#",
            2 => "B",
            3 => "C",
            4 => "C#",
            5 => "D",
            6 => "D#",
            7 => "E",
            8 => "F",
            9 => "F#",
            10 => "G",
            11 => "G#",
            _=> unimplemented!()
        }
    }

    pub fn get_color(self)-> Color{
        let hue = 30.0 * (self.0 as f32 );
        Color::Hsla { hue, saturation: 0.7, lightness: 0.8, alpha: 1.0 }
    }
}

impl Debug for Note{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}