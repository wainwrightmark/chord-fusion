use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use bevy::render::once_cell::sync::OnceCell;

use itertools::Itertools;
use strum::{EnumCount, EnumIter, FromRepr, IntoEnumIterator};

use crate::cluster::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter, EnumCount, FromRepr)]
pub enum Chord {
    Major,
    Minor,
    Diminished,
    Augmented,
    Suspended2,
    Suspended4,

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

impl  std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.nice_name())
    }
}

impl Chord {
    pub fn get_notes(self, root: Note) -> Vec<Note> {
        self.intervals()
            .iter()
            .map(|i| Note((root.0 + i) % 12))
            .collect_vec()
    }
}

impl Chord {
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::Major => "M",
            Self::Minor => "m",
            Self::Diminished => "o",
            Self::Augmented => "+",
            Self::Suspended2 => "s2",
            Self::Suspended4 => "s4",

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
    pub fn nice_name(&self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Diminished => "dim",
            Self::Augmented => "aug",
            Self::Suspended2 => "sus2",
            Self::Suspended4 => "sus4",

            Self::Dominant7 => "7",
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
    pub fn intervals(&self) -> Vec<u8> {
        match self {
            Self::Major => vec![0, 4, 7],
            Self::Minor => vec![0, 3, 7],
            Self::Diminished => vec![0, 3, 6],
            Self::Augmented => vec![0, 4, 8],
            Self::Suspended2 => vec![0, 2, 7],
            Self::Suspended4 => vec![0, 5, 7],

            Self::Dominant7 => vec![0, 4, 7, 10],
            Self::Major7 => vec![0, 4, 7, 11],
            Self::Minor7 => vec![0, 3, 7, 10],
            Self::MinorMajor7 => vec![0, 3, 7, 11],
            Self::HalfDiminished => vec![0, 3, 6, 10],
            Self::Diminished7 => vec![0, 3, 6, 9],
            Self::Augmented7 => vec![0, 4, 8, 10],
            Self::AugmentedMaj7 => vec![0, 4, 8, 11],
            Self::Dominant11 => vec![0, 5, 7, 10],
        }
    }
    pub fn all() -> &'static BTreeMap<Vec<u8>, Self> {
        CHORDS.get_or_init(|| BTreeMap::from_iter(Chord::iter().map(|c| (c.intervals(), c))))
    }
}

static CHORDS: OnceCell<BTreeMap<Vec<u8>, Chord>> = OnceCell::new();
