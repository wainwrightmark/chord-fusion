use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use bevy::render::once_cell::sync::OnceCell;

use strum::{EnumIter, IntoEnumIterator};

pub trait Chord<const L: usize>: Clone + Copy + PartialEq + Eq + Hash + Debug {
    fn short_name(&self) -> &'static str;

    fn nice_name(&self) -> &'static str;

    fn intervals(&self) -> [u8; L];

    fn all() -> &'static BTreeMap<[u8; L], Self>;
}

static CHORDS3: OnceCell<BTreeMap<[u8; 3], Chord3>> = OnceCell::new();
static CHORDS4: OnceCell<BTreeMap<[u8; 4], Chord4>> = OnceCell::new();

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum Chord3 {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended2,
    Suspended4,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum Chord4 {
    Dominant7,
    Major7,
    Minor7,
    MinorMajor7,
    HalfDiminished,
    Diminished7,
    Augmented7,
    AugmentedMaj7,
    Dominant11,
}

impl Chord<3> for Chord3 {
    fn short_name(&self) -> &'static str {
        match self {
            Self::Major => "M",
            Self::Minor => "m",
            Self::Diminished => "o",
            Self::Augmented => "+",
            Self::Suspended2 => "s2",
            Self::Suspended4 => "s4",
        }
    }


    fn nice_name(&self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Diminished => "dim",
            Self::Augmented => "aug",
            Self::Suspended2 => "sus2",
            Self::Suspended4 => "sus4",
        }
    }

    fn intervals(&self) -> [u8; 3] {
        match self {
            Chord3::Major => [0, 4,7],
            Chord3::Minor => [0,3,7],
            Chord3::Diminished => [0,3,6],
            Chord3::Augmented => [0,4,8],
            Chord3::Suspended2 => [0,2,7],
            Chord3::Suspended4 => [0,5,7],
        }
    }

    fn all() -> &'static BTreeMap<[u8; 3], Self> {
        CHORDS3.get_or_init(|| BTreeMap::from_iter(Chord3::iter().map(|c| (c.intervals(), c))))
    }
}

impl Chord<4> for Chord4 {
    fn short_name(&self) -> &'static str {
        match self {
            Self::Dominant7 => "7",
            Self::Major7 => "M7",
            Self::Minor7 => "m7",
            Self::MinorMajor7 => "mM7",
            Self::HalfDiminished => "ø7",
            Self::Diminished7 => "o7",
            Self::Augmented7 => "+7",
            Self::AugmentedMaj7 => "+M7",
            Self::Dominant11 => "11",
        }
    }

    fn nice_name(&self) -> &'static str {
        match self {
            Self::Dominant7 => "dom7",
            Self::Major7 => "major7",
            Self::Minor7 => "minor7",
            Self::MinorMajor7 => "minor major7",
            Self::HalfDiminished => "half dim7",
            Self::Diminished7 => "dim7",
            Self::Augmented7 => "aug7",
            Self::AugmentedMaj7 => "aug major7",
            Self::Dominant11 => "11",
        }
    }

    fn intervals(&self) -> [u8; 4] {
        match self{
            Chord4::Dominant7 => [0,4,7,10],
            Chord4::Major7 => [0,4,7,11],
            Chord4::Minor7 => [0,3,7,10],
            Chord4::MinorMajor7 => [0,3,7,11],
            Chord4::HalfDiminished => [0,3,6,10],
            Chord4::Diminished7 => [0,3,6,9],
            Chord4::Augmented7 => [0,4,8,10],
            Chord4::AugmentedMaj7 => [0,4,8,11],
            Chord4::Dominant11 => [0,5,7,10],
        }
    }

    fn all() -> &'static BTreeMap<[u8; 4], Self> {
        CHORDS4.get_or_init(|| BTreeMap::from_iter(Chord4::iter().map(|c| (c.intervals(), c))))
    }
}
