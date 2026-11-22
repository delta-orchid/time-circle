use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PitchClass {
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

impl PitchClass {
    pub const ALL: [PitchClass; 12] = [
        PitchClass::C,
        PitchClass::Db,
        PitchClass::D,
        PitchClass::Eb,
        PitchClass::E,
        PitchClass::F,
        PitchClass::Gb,
        PitchClass::G,
        PitchClass::Ab,
        PitchClass::A,
        PitchClass::Bb,
        PitchClass::B,
    ];

    pub fn from_semitones(val: i32) -> Self {
        let m = val.rem_euclid(12) as u8;
        match m {
            0 => PitchClass::C,
            1 => PitchClass::Db,
            2 => PitchClass::D,
            3 => PitchClass::Eb,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::Gb,
            7 => PitchClass::G,
            8 => PitchClass::Ab,
            9 => PitchClass::A,
            10 => PitchClass::Bb,
            _ => PitchClass::B,
        }
    }

    pub fn transpose(self, semitones: i32) -> Self {
        Self::from_semitones(self as i32 + semitones)
    }

    pub fn semitones(self) -> u8 {
        self as u8
    }

    pub fn name(self, prefer_sharp: bool) -> &'static str {
        match self {
            PitchClass::C => "C",
            PitchClass::Db => if prefer_sharp { "C#" } else { "Db" },
            PitchClass::D => "D",
            PitchClass::Eb => if prefer_sharp { "D#" } else { "Eb" },
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::Gb => if prefer_sharp { "F#" } else { "Gb" },
            PitchClass::G => "G",
            PitchClass::Ab => if prefer_sharp { "G#" } else { "Ab" },
            PitchClass::A => "A",
            PitchClass::Bb => if prefer_sharp { "A#" } else { "Bb" },
            PitchClass::B => "B",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChordQuality {
    Dom7,     // 1 3 5 b7
    Dom9,     // 1 3 5 b7 9
    Dom13,    // 1 3 b7 13
    Dom7Alt,  // 1 3 b7 #9 / b13
    Maj7,     // 1 3 5 7
    Maj6,     // 1 3 5 6
    Min7,     // 1 b3 5 b7
    Min6,     // 1 b3 5 6
    Min9,     // 1 b3 5 b7 9
    HalfDim7, // 1 b3 b5 b7 (m7b5)
    Dim7,     // 1 b3 b5 bb7
}

impl ChordQuality {
    pub const ALL: [ChordQuality; 11] = [
        ChordQuality::Dom7,
        ChordQuality::Dom9,
        ChordQuality::Dom13,
        ChordQuality::Dom7Alt,
        ChordQuality::Maj7,
        ChordQuality::Maj6,
        ChordQuality::Min7,
        ChordQuality::Min6,
        ChordQuality::Min9,
        ChordQuality::HalfDim7,
        ChordQuality::Dim7,
    ];

    pub fn symbol(&self) -> &'static str {
        match self {
            ChordQuality::Dom7 => "7",
            ChordQuality::Dom9 => "9",
            ChordQuality::Dom13 => "13",
            ChordQuality::Dom7Alt => "7alt",
            ChordQuality::Maj7 => "maj7",
            ChordQuality::Maj6 => "6",
            ChordQuality::Min7 => "m7",
            ChordQuality::Min6 => "m6",
            ChordQuality::Min9 => "m9",
            ChordQuality::HalfDim7 => "m7b5",
            ChordQuality::Dim7 => "dim7",
        }
    }

    /// Intervals in semitones relative to root for voicing (shell/extensions)
    pub fn intervals(&self) -> &'static [u8] {
        match self {
            ChordQuality::Dom7 => &[0, 4, 7, 10],
            ChordQuality::Dom9 => &[0, 4, 10, 14], // 1 3 b7 9
            ChordQuality::Dom13 => &[0, 4, 10, 21], // 1 3 b7 13 (A = 21 st above root)
            ChordQuality::Dom7Alt => &[0, 4, 10, 15], // 1 3 b7 #9
            ChordQuality::Maj7 => &[0, 4, 7, 11],
            ChordQuality::Maj6 => &[0, 4, 7, 9],
            ChordQuality::Min7 => &[0, 3, 7, 10],
            ChordQuality::Min6 => &[0, 3, 7, 9],
            ChordQuality::Min9 => &[0, 3, 10, 14],
            ChordQuality::HalfDim7 => &[0, 3, 6, 10],
            ChordQuality::Dim7 => &[0, 3, 6, 9],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chord {
    pub root: PitchClass,
    pub quality: ChordQuality,
}

impl Chord {
    pub fn new(root: PitchClass, quality: ChordQuality) -> Self {
        Self { root, quality }
    }

    pub fn name(&self, prefer_sharp: bool) -> String {
        format!("{}{}", self.root.name(prefer_sharp), self.quality.symbol())
    }

    /// Returns MIDI note numbers centered around target pitch (e.g., C4 = 60)
    pub fn voicing(&self, center_midi: u8) -> Vec<u8> {
        let base_semitone = self.root.semitones() as i32;
        let mut notes = Vec::new();
        for &interval in self.quality.intervals() {
            let semitone = base_semitone + interval as i32;
            // bring octave near center_midi
            let mut note = 60 + (semitone % 12);
            while (note as i32 - center_midi as i32) > 6 {
                note -= 12;
            }
            while (center_midi as i32 - note as i32) > 6 {
                note += 12;
            }
            if interval == 14 || interval == 15 || interval == 21 {
                // Extensions sound great placed in higher register
                note += 12;
            }
            notes.push(note as u8);
        }
        notes.sort_unstable();
        notes.dedup();
        notes
    }
}

/// Represents the chords in one bar. Supports 1, 2, or 4 chords (subdividing 4 beats).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BarChords {
    pub slots: Vec<Chord>, // length 1, 2, or 4
}

impl BarChords {
    pub fn single(chord: Chord) -> Self {
        Self { slots: vec![chord] }
    }

    pub fn two(first: Chord, second: Chord) -> Self {
        Self { slots: vec![first, second] }
    }

    pub fn four(c1: Chord, c2: Chord, c3: Chord, c4: Chord) -> Self {
        Self { slots: vec![c1, c2, c3, c4] }
    }

    pub fn get_chord_at_beat(&self, beat_index: usize) -> Chord {
        match self.slots.len() {
            1 => self.slots[0],
            2 => if beat_index < 2 { self.slots[0] } else { self.slots[1] },
            4 => self.slots[beat_index.min(3)],
            _ => self.slots[0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BluesFormPreset {
    Standard12,
    QuickChange12,
    ChicagoSlow12,
    JazzBird12,
    MinorBlues12,
    EightBarBlues,
    SixteenBarBlues,
}

impl BluesFormPreset {
    pub const ALL: [BluesFormPreset; 7] = [
        BluesFormPreset::Standard12,
        BluesFormPreset::QuickChange12,
        BluesFormPreset::ChicagoSlow12,
        BluesFormPreset::JazzBird12,
        BluesFormPreset::MinorBlues12,
        BluesFormPreset::EightBarBlues,
        BluesFormPreset::SixteenBarBlues,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BluesFormPreset::Standard12 => "Standard 12-Bar Blues",
            BluesFormPreset::QuickChange12 => "Quick-Change 12-Bar",
            BluesFormPreset::ChicagoSlow12 => "Chicago Slow 12-Bar (#IVdim7)",
            BluesFormPreset::JazzBird12 => "Jazz / Bird Blues 12-Bar",
            BluesFormPreset::MinorBlues12 => "Minor Blues 12-Bar",
            BluesFormPreset::EightBarBlues => "8-Bar Blues (Key to Highway)",
            BluesFormPreset::SixteenBarBlues => "16-Bar Extended Blues",
        }
    }

    pub fn bar_count(&self) -> usize {
        match self {
            BluesFormPreset::EightBarBlues => 8,
            BluesFormPreset::SixteenBarBlues => 16,
            _ => 12,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub key: PitchClass,
    pub bars: Vec<BarChords>,
    pub preset: BluesFormPreset,
}

impl Progression {
    pub fn new_preset(key: PitchClass, preset: BluesFormPreset) -> Self {
        let bars = generate_preset_bars(key, preset);
        Self { key, bars, preset }
    }

    pub fn transpose(&mut self, new_key: PitchClass) {
        let delta = new_key.semitones() as i32 - self.key.semitones() as i32;
        self.key = new_key;
        for bar in &mut self.bars {
            for chord in &mut bar.slots {
                chord.root = chord.root.transpose(delta);
            }
        }
    }

    pub fn bar_count(&self) -> usize {
        self.bars.len()
    }
}

pub fn generate_preset_bars(key: PitchClass, preset: BluesFormPreset) -> Vec<BarChords> {
    let i = key;
    let iv = key.transpose(5);
    let v = key.transpose(7);
    let sharp_iv = key.transpose(6);
    let ii = key.transpose(2);
    let iii = key.transpose(4);
    let vi = key.transpose(9);
    let _flat_vii = key.transpose(10);
    let flat_vi = key.transpose(8);
    let _flat_iii = key.transpose(3);
    let _flat_ii = key.transpose(1);

    match preset {
        BluesFormPreset::Standard12 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
        ],
        BluesFormPreset::QuickChange12 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom9)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom9)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom9)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom9)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7)),
        ],
        BluesFormPreset::ChicagoSlow12 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom9)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom9)),
            BarChords::single(Chord::new(sharp_iv, ChordQuality::Dim7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(vi, ChordQuality::Dom7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom9)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(vi, ChordQuality::Dom7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7Alt)),
        ],
        BluesFormPreset::JazzBird12 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Maj7)),
            BarChords::two(Chord::new(vii_half_dim(key), ChordQuality::HalfDim7), Chord::new(iii, ChordQuality::Dom7Alt)),
            BarChords::two(Chord::new(vi, ChordQuality::Min7), Chord::new(ii.transpose(5), ChordQuality::Dom7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(sharp_iv, ChordQuality::Dim7)),
            BarChords::two(Chord::new(i, ChordQuality::Maj7), Chord::new(vi, ChordQuality::Dom7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7)),
            BarChords::two(Chord::new(iii, ChordQuality::Min7), Chord::new(vi, ChordQuality::Dom7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Maj7), Chord::new(vi, ChordQuality::Dom7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7Alt)),
        ],
        BluesFormPreset::MinorBlues12 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(iv, ChordQuality::Min7)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(iv, ChordQuality::Min7)),
            BarChords::single(Chord::new(iv, ChordQuality::Min7)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(flat_vi, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7Alt)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::two(Chord::new(flat_vi, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7Alt)),
        ],
        BluesFormPreset::EightBarBlues => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(iv, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7)),
        ],
        BluesFormPreset::SixteenBarBlues => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7)),
        ],
    }
}

fn vii_half_dim(key: PitchClass) -> PitchClass {
    key.transpose(11)
}

/// Applies musical substitutions and variations based on spice factor (0.0 - 1.0)
pub fn mutate_progression(prog: &mut Progression, spice: f32) {
    let key = prog.key;
    let _i = key;
    let iv = key.transpose(5);
    let v = key.transpose(7);
    let sharp_iv = key.transpose(6);
    let ii = key.transpose(2);
    let vi = key.transpose(9);

    if prog.bars.len() == 12 {
        // Quick change bar 2
        if spice > 0.2 {
            prog.bars[1] = BarChords::single(Chord::new(iv, if spice > 0.6 { ChordQuality::Dom9 } else { ChordQuality::Dom7 }));
        }

        // #IVdim7 passing chord in bar 6
        if spice > 0.4 {
            prog.bars[5] = BarChords::single(Chord::new(sharp_iv, ChordQuality::Dim7));
        }

        // Bar 8 ii-V preparation (VI7 or VI7alt)
        if spice > 0.5 {
            prog.bars[7] = BarChords::single(Chord::new(vi, if spice > 0.75 { ChordQuality::Dom7Alt } else { ChordQuality::Dom7 }));
            prog.bars[8] = BarChords::single(Chord::new(ii, ChordQuality::Min7));
            prog.bars[9] = BarChords::single(Chord::new(v, ChordQuality::Dom9));
        }

        // Color extensions (9ths, 13ths)
        if spice > 0.3 {
            for bar in &mut prog.bars {
                for chord in &mut bar.slots {
                    if chord.quality == ChordQuality::Dom7 && rand::random::<f32>() < spice * 0.7 {
                        chord.quality = if rand::random::<bool>() { ChordQuality::Dom9 } else { ChordQuality::Dom13 };
                    }
                }
            }
        }
    }
}

/// Randomizes the classic 2-bar turnaround in the final 2 bars
pub fn mutate_turnaround(prog: &mut Progression) {
    let len = prog.bars.len();
    if len < 2 {
        return;
    }
    let key = prog.key;
    let i = key;
    let iv = key.transpose(5);
    let v = key.transpose(7);
    let ii = key.transpose(2);
    let vi = key.transpose(9);
    let flat_iii = key.transpose(3);
    let flat_ii = key.transpose(1);
    let sharp_iv = key.transpose(6);

    let pattern: u8 = rand::random::<u8>() % 5;
    match pattern {
        0 => {
            // Classic I7 - VI7 | ii7 - V7
            prog.bars[len - 2] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(vi, ChordQuality::Dom7));
            prog.bars[len - 1] = BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7Alt));
        }
        1 => {
            // Tritone walkdown: I7 - bIII7 | bII7 - I7
            prog.bars[len - 2] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(flat_iii, ChordQuality::Dom7));
            prog.bars[len - 1] = BarChords::two(Chord::new(flat_ii, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7));
        }
        2 => {
            // #IVdim7 turnaround: I7 - #IVdim7 | I/V - V7
            prog.bars[len - 2] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(sharp_iv, ChordQuality::Dim7));
            prog.bars[len - 1] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom9));
        }
        3 => {
            // 4-beat walk: | I7 VI7 ii7 V7 |
            prog.bars[len - 2] = BarChords::single(Chord::new(i, ChordQuality::Dom7));
            prog.bars[len - 1] = BarChords::four(
                Chord::new(i, ChordQuality::Dom7),
                Chord::new(vi, ChordQuality::Dom7),
                Chord::new(ii, ChordQuality::Min7),
                Chord::new(v, ChordQuality::Dom7Alt),
            );
        }
        _ => {
            // Classic Chicago: I7 - IV7 | I7 - V7
            prog.bars[len - 2] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(iv, ChordQuality::Dom7));
            prog.bars[len - 1] = BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7));
        }
    }
}
