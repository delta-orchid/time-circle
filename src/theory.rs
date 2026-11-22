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

    pub fn from_name(name: &str) -> Option<Self> {
        let clean = name.trim();
        match clean.to_ascii_uppercase().as_str() {
            "C" => Some(PitchClass::C),
            "C#" | "DB" => Some(PitchClass::Db),
            "D" => Some(PitchClass::D),
            "D#" | "EB" => Some(PitchClass::Eb),
            "E" => Some(PitchClass::E),
            "F" => Some(PitchClass::F),
            "F#" | "GB" => Some(PitchClass::Gb),
            "G" => Some(PitchClass::G),
            "G#" | "AB" => Some(PitchClass::Ab),
            "A" => Some(PitchClass::A),
            "A#" | "BB" => Some(PitchClass::Bb),
            "B" => Some(PitchClass::B),
            _ => None,
        }
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
    Maj,       // 1 3 5
    Min,       // 1 b3 5
    Sus4,      // 1 4 5
    Add9,      // 1 3 5 9
    Dom7,      // 1 3 5 b7
    Dom7Sus4,  // 1 4 5 b7
    Dom9,      // 1 3 5 b7 9
    Dom13,     // 1 3 b7 13
    Dom7Alt,   // 1 3 b7 #9 / b13
    Maj7,      // 1 3 5 7
    Maj6,      // 1 3 5 6
    Min7,      // 1 b3 5 b7
    Min6,      // 1 b3 5 6
    Min9,      // 1 b3 5 b7 9
    HalfDim7,  // 1 b3 b5 b7 (m7b5)
    Dim7,      // 1 b3 b5 bb7
}

impl ChordQuality {
    pub const ALL: [ChordQuality; 16] = [
        ChordQuality::Maj,
        ChordQuality::Min,
        ChordQuality::Sus4,
        ChordQuality::Add9,
        ChordQuality::Dom7,
        ChordQuality::Dom7Sus4,
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
            ChordQuality::Maj => "",
            ChordQuality::Min => "m",
            ChordQuality::Sus4 => "sus4",
            ChordQuality::Add9 => "add9",
            ChordQuality::Dom7 => "7",
            ChordQuality::Dom7Sus4 => "7sus4",
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
            ChordQuality::Maj => &[0, 4, 7, 12],
            ChordQuality::Min => &[0, 3, 7, 12],
            ChordQuality::Sus4 => &[0, 5, 7, 12],
            ChordQuality::Add9 => &[0, 4, 7, 14],
            ChordQuality::Dom7 => &[0, 4, 7, 10],
            ChordQuality::Dom7Sus4 => &[0, 5, 7, 10],
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

    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        let (root, rem) = if s.len() >= 2 {
            if let Some(r) = PitchClass::from_name(&s[0..2]) {
                (r, &s[2..])
            } else if let Some(r) = PitchClass::from_name(&s[0..1]) {
                (r, &s[1..])
            } else {
                return None;
            }
        } else if let Some(r) = PitchClass::from_name(&s[0..1]) {
            (r, &s[1..])
        } else {
            return None;
        };

        let rem_lower = rem.trim().to_ascii_lowercase();
        let quality = match rem_lower.as_str() {
            "" | "maj" | "major" => ChordQuality::Maj,
            "m" | "min" | "minor" => ChordQuality::Min,
            "7" | "dom7" => ChordQuality::Dom7,
            "9" | "dom9" => ChordQuality::Dom9,
            "13" | "dom13" => ChordQuality::Dom13,
            "maj7" | "m7+" | "ma7" => ChordQuality::Maj7,
            "maj6" | "6" => ChordQuality::Maj6,
            "m7" | "min7" => ChordQuality::Min7,
            "m6" | "min6" => ChordQuality::Min6,
            "m9" | "min9" => ChordQuality::Min9,
            "m7b5" | "halfdim" | "ø" => ChordQuality::HalfDim7,
            "dim" | "dim7" | "o" => ChordQuality::Dim7,
            "sus4" | "sus" => ChordQuality::Sus4,
            "7sus4" | "7sus" => ChordQuality::Dom7Sus4,
            "add9" | "2" => ChordQuality::Add9,
            "7alt" | "alt" => ChordQuality::Dom7Alt,
            _ => ChordQuality::Maj,
        };

        Some(Chord::new(root, quality))
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormCategory {
    Blues,
    PopRock,
    SoulRnB,
}

impl FormCategory {
    pub const ALL: [FormCategory; 3] = [
        FormCategory::Blues,
        FormCategory::PopRock,
        FormCategory::SoulRnB,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            FormCategory::Blues => "Blues",
            FormCategory::PopRock => "Pop & Rock",
            FormCategory::SoulRnB => "Soul & R&B",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormPreset {
    // Blues
    Standard12,
    QuickChange12,
    ChicagoSlow12,
    JazzBird12,
    MinorBlues12,
    EightBarBlues,
    SixteenBarBlues,

    // Pop & Rock
    PopAxis4,
    DooWop4,
    RockMixolydian4,
    Andalusian4,
    PopVerseChorus16,

    // Soul & R&B
    MinorSoul4,
    RoyalRoad4,
    NeoSoulClimb8,
    Motown8,
    SoulAaba16,
}

#[allow(dead_code)]
pub type BluesFormPreset = FormPreset;

impl FormPreset {
    pub const ALL: [FormPreset; 17] = [
        // Blues
        FormPreset::Standard12,
        FormPreset::QuickChange12,
        FormPreset::ChicagoSlow12,
        FormPreset::JazzBird12,
        FormPreset::MinorBlues12,
        FormPreset::EightBarBlues,
        FormPreset::SixteenBarBlues,

        // Pop & Rock
        FormPreset::PopAxis4,
        FormPreset::DooWop4,
        FormPreset::RockMixolydian4,
        FormPreset::Andalusian4,
        FormPreset::PopVerseChorus16,

        // Soul & R&B
        FormPreset::MinorSoul4,
        FormPreset::RoyalRoad4,
        FormPreset::NeoSoulClimb8,
        FormPreset::Motown8,
        FormPreset::SoulAaba16,
    ];

    pub fn category(&self) -> FormCategory {
        match self {
            FormPreset::Standard12
            | FormPreset::QuickChange12
            | FormPreset::ChicagoSlow12
            | FormPreset::JazzBird12
            | FormPreset::MinorBlues12
            | FormPreset::EightBarBlues
            | FormPreset::SixteenBarBlues => FormCategory::Blues,

            FormPreset::PopAxis4
            | FormPreset::DooWop4
            | FormPreset::RockMixolydian4
            | FormPreset::Andalusian4
            | FormPreset::PopVerseChorus16 => FormCategory::PopRock,

            FormPreset::MinorSoul4
            | FormPreset::RoyalRoad4
            | FormPreset::NeoSoulClimb8
            | FormPreset::Motown8
            | FormPreset::SoulAaba16 => FormCategory::SoulRnB,
        }
    }

    pub fn presets_for_category(category: FormCategory) -> Vec<FormPreset> {
        Self::ALL
            .iter()
            .copied()
            .filter(|p| p.category() == category)
            .collect()
    }

    pub fn from_identifier_or_name(input: &str) -> Option<Self> {
        let clean = input.trim();
        for p in Self::ALL {
            if format!("{:?}", p).eq_ignore_ascii_case(clean) {
                return Some(p);
            }
            if p.name().eq_ignore_ascii_case(clean) {
                return Some(p);
            }
        }
        // Substring / simplified match
        let s = clean.to_ascii_lowercase().replace([' ', '-', '_', '/'], "");
        for p in Self::ALL {
            let ident = format!("{:?}", p).to_ascii_lowercase();
            let name_simplified = p.name().to_ascii_lowercase().replace([' ', '-', '_', '/'], "");
            if ident.contains(&s) || name_simplified.contains(&s) || s.contains(&ident) {
                return Some(p);
            }
        }
        None
    }

    pub fn name(&self) -> &'static str {
        match self {
            // Blues
            FormPreset::Standard12 => "Standard 12-Bar Blues",
            FormPreset::QuickChange12 => "Quick-Change 12-Bar",
            FormPreset::ChicagoSlow12 => "Chicago Slow 12-Bar (#IVdim7)",
            FormPreset::JazzBird12 => "Jazz / Bird Blues 12-Bar",
            FormPreset::MinorBlues12 => "Minor Blues 12-Bar",
            FormPreset::EightBarBlues => "8-Bar Blues (Key to Highway)",
            FormPreset::SixteenBarBlues => "16-Bar Extended Blues",

            // Pop & Rock
            FormPreset::PopAxis4 => "Pop Axis (I – V – vi – IV)",
            FormPreset::DooWop4 => "50s Doo-Wop / Ballad (I – vi – IV – V)",
            FormPreset::RockMixolydian4 => "Classic Rock Mixolydian (I – bVII – IV – I)",
            FormPreset::Andalusian4 => "Andalusian Cadence (i – bVII – bVI – V7)",
            FormPreset::PopVerseChorus16 => "Pop Verse + Chorus 16-Bar",

            // Soul & R&B
            FormPreset::MinorSoul4 => "Minor Soul / Bill Withers (i7 – V7 – i7 – IV9)",
            FormPreset::RoyalRoad4 => "Royal Road Soul (IVmaj7 – V7 – iii7 – vi7)",
            FormPreset::NeoSoulClimb8 => "Neo-Soul / Gospel Climb (8-Bar)",
            FormPreset::Motown8 => "Motown 8-Bar Groove",
            FormPreset::SoulAaba16 => "16-Bar Soul AABA",
        }
    }

    #[allow(dead_code)]
    pub fn bar_count(&self) -> usize {
        match self {
            FormPreset::PopAxis4
            | FormPreset::DooWop4
            | FormPreset::RockMixolydian4
            | FormPreset::Andalusian4
            | FormPreset::MinorSoul4
            | FormPreset::RoyalRoad4 => 4,

            FormPreset::EightBarBlues
            | FormPreset::NeoSoulClimb8
            | FormPreset::Motown8 => 8,

            FormPreset::Standard12
            | FormPreset::QuickChange12
            | FormPreset::ChicagoSlow12
            | FormPreset::JazzBird12
            | FormPreset::MinorBlues12 => 12,

            FormPreset::SixteenBarBlues
            | FormPreset::PopVerseChorus16
            | FormPreset::SoulAaba16 => 16,
        }
    }

    /// Optional section badge displayed on the outer rim of the circle for multi-section forms
    pub fn section_label(&self, bar_idx: usize) -> Option<&'static str> {
        match self {
            FormPreset::PopVerseChorus16 => match bar_idx {
                0 => Some("A: VERSE"),
                8 => Some("B: CHORUS"),
                _ => None,
            },
            FormPreset::SoulAaba16 => match bar_idx {
                0 => Some("A1"),
                4 => Some("A2"),
                8 => Some("B: BRIDGE"),
                12 => Some("A3"),
                _ => None,
            },
            FormPreset::SixteenBarBlues => match bar_idx {
                0 => Some("A: VERSE"),
                8 => Some("B: REFRAIN"),
                _ => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub key: PitchClass,
    pub bars: Vec<BarChords>,
    pub preset: FormPreset,
}

impl Progression {
    pub fn new_preset(key: PitchClass, preset: FormPreset) -> Self {
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

    #[allow(dead_code)]
    pub fn bar_count(&self) -> usize {
        self.bars.len()
    }
}

pub fn generate_preset_bars(key: PitchClass, preset: FormPreset) -> Vec<BarChords> {
    let i = key;
    let _flat_ii = key.transpose(1);
    let ii = key.transpose(2);
    let _flat_iii = key.transpose(3);
    let iii = key.transpose(4);
    let iv = key.transpose(5);
    let sharp_iv = key.transpose(6);
    let v = key.transpose(7);
    let flat_vi = key.transpose(8);
    let vi = key.transpose(9);
    let flat_vii = key.transpose(10);

    match preset {
        // --- BLUES ---
        FormPreset::Standard12 => vec![
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
        FormPreset::QuickChange12 => vec![
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
        FormPreset::ChicagoSlow12 => vec![
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
        FormPreset::JazzBird12 => vec![
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
        FormPreset::MinorBlues12 => vec![
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
        FormPreset::EightBarBlues => vec![
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(iv, ChordQuality::Dom7)),
            BarChords::two(Chord::new(i, ChordQuality::Dom7), Chord::new(v, ChordQuality::Dom7)),
        ],
        FormPreset::SixteenBarBlues => vec![
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

        // --- POP & ROCK ---
        FormPreset::PopAxis4 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
        ],
        FormPreset::DooWop4 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
        ],
        FormPreset::RockMixolydian4 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(flat_vii, ChordQuality::Maj)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
        ],
        FormPreset::Andalusian4 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Min)),
            BarChords::single(Chord::new(flat_vii, ChordQuality::Maj)),
            BarChords::single(Chord::new(flat_vi, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
        ],
        FormPreset::PopVerseChorus16 => vec![
            // Verse (bars 1-8): vi - IV - I - V (x2)
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
            // Chorus (bars 9-16): I - V - vi - IV (x2)
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(v, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
        ],

        // --- SOUL & R&B ---
        FormPreset::MinorSoul4 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(i, ChordQuality::Min7)),
            BarChords::single(Chord::new(iv, ChordQuality::Dom9)),
        ],
        FormPreset::RoyalRoad4 => vec![
            BarChords::single(Chord::new(iv, ChordQuality::Maj7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iii, ChordQuality::Min7)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
        ],
        FormPreset::NeoSoulClimb8 => vec![
            BarChords::single(Chord::new(iv, ChordQuality::Maj7)),
            BarChords::single(Chord::new(iii, ChordQuality::Dom7Alt)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::two(Chord::new(v, ChordQuality::Min7), Chord::new(i, ChordQuality::Dom7)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj7)),
            BarChords::single(Chord::new(iii, ChordQuality::Dom7Alt)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::two(Chord::new(ii, ChordQuality::Min7), Chord::new(v, ChordQuality::Dom7Alt)),
        ],
        FormPreset::Motown8 => vec![
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(iii, ChordQuality::Min7)),
            BarChords::single(Chord::new(iv, ChordQuality::Maj)),
            BarChords::single(Chord::new(i, ChordQuality::Maj)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
        ],
        FormPreset::SoulAaba16 => vec![
            // A1
            BarChords::single(Chord::new(i, ChordQuality::Maj7)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            // A2
            BarChords::single(Chord::new(i, ChordQuality::Maj7)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7)),
            // B Bridge
            BarChords::single(Chord::new(iv, ChordQuality::Maj7)),
            BarChords::single(Chord::new(iii, ChordQuality::Min7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(v, ChordQuality::Dom7Alt)),
            // A3
            BarChords::single(Chord::new(i, ChordQuality::Maj7)),
            BarChords::single(Chord::new(vi, ChordQuality::Min7)),
            BarChords::single(Chord::new(ii, ChordQuality::Min7)),
            BarChords::single(Chord::new(i, ChordQuality::Maj7)),
        ],
    }
}

fn vii_half_dim(key: PitchClass) -> PitchClass {
    key.transpose(11)
}

/// Applies musical substitutions and variations based on spice factor (0.0 - 1.0)
pub fn mutate_progression(prog: &mut Progression, spice: f32) {
    if prog.preset.category() == FormCategory::Blues {
        mutate_blues_progression(prog, spice);
    } else {
        mutate_extension_progression(prog, spice);
    }
}

fn mutate_blues_progression(prog: &mut Progression, spice: f32) {
    let key = prog.key;
    let iv = key.transpose(5);
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
            prog.bars[9] = BarChords::single(Chord::new(v_from_key(key), ChordQuality::Dom9));
        }
    }

    // Color extensions (9ths, 13ths) across blues forms
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

fn v_from_key(key: PitchClass) -> PitchClass {
    key.transpose(7)
}

/// Uniformly upgrades chord extensions for Pop, Rock, and Soul forms
fn mutate_extension_progression(prog: &mut Progression, spice: f32) {
    if spice <= 0.15 {
        return;
    }

    for bar in &mut prog.bars {
        for chord in &mut bar.slots {
            if rand::random::<f32>() > (spice * 0.85) {
                continue;
            }
            match chord.quality {
                ChordQuality::Maj => {
                    chord.quality = if spice > 0.6 && rand::random::<bool>() {
                        ChordQuality::Maj7
                    } else if rand::random::<bool>() {
                        ChordQuality::Add9
                    } else {
                        ChordQuality::Maj6
                    };
                }
                ChordQuality::Min => {
                    chord.quality = if spice > 0.7 && rand::random::<bool>() {
                        ChordQuality::Min9
                    } else {
                        ChordQuality::Min7
                    };
                }
                ChordQuality::Dom7 => {
                    chord.quality = if spice > 0.7 && rand::random::<bool>() {
                        ChordQuality::Dom13
                    } else if rand::random::<bool>() {
                        ChordQuality::Dom9
                    } else {
                        ChordQuality::Dom7Sus4
                    };
                }
                ChordQuality::Min7 => {
                    if spice > 0.5 {
                        chord.quality = ChordQuality::Min9;
                    }
                }
                ChordQuality::Maj7 => {
                    if spice > 0.5 && rand::random::<bool>() {
                        chord.quality = ChordQuality::Maj6;
                    }
                }
                _ => {}
            }
        }
    }
}

/// Randomizes the classic 2-bar turnaround in the final 2 bars (for Blues)
pub fn mutate_turnaround(prog: &mut Progression) {
    if prog.preset.category() != FormCategory::Blues {
        return;
    }
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

/// Provides one-tap harmonic degree presets tailored to the active form category
pub fn degree_presets_for_category(category: FormCategory, key: PitchClass) -> Vec<(&'static str, PitchClass, ChordQuality)> {
    match category {
        FormCategory::Blues => vec![
            ("I7", key, ChordQuality::Dom7),
            ("I9", key, ChordQuality::Dom9),
            ("IV7", key.transpose(5), ChordQuality::Dom7),
            ("IV9", key.transpose(5), ChordQuality::Dom9),
            ("#IVdim7", key.transpose(6), ChordQuality::Dim7),
            ("V7", key.transpose(7), ChordQuality::Dom7),
            ("V7alt", key.transpose(7), ChordQuality::Dom7Alt),
            ("ii7", key.transpose(2), ChordQuality::Min7),
            ("VI7", key.transpose(9), ChordQuality::Dom7),
            ("bII7", key.transpose(1), ChordQuality::Dom7),
        ],
        FormCategory::PopRock => vec![
            ("I", key, ChordQuality::Maj),
            ("Iadd9", key, ChordQuality::Add9),
            ("ii", key.transpose(2), ChordQuality::Min),
            ("ii7", key.transpose(2), ChordQuality::Min7),
            ("iii", key.transpose(4), ChordQuality::Min),
            ("IV", key.transpose(5), ChordQuality::Maj),
            ("IVmaj7", key.transpose(5), ChordQuality::Maj7),
            ("V", key.transpose(7), ChordQuality::Maj),
            ("Vsus4", key.transpose(7), ChordQuality::Sus4),
            ("vi", key.transpose(9), ChordQuality::Min),
            ("vi7", key.transpose(9), ChordQuality::Min7),
            ("bVI", key.transpose(8), ChordQuality::Maj),
            ("bVII", key.transpose(10), ChordQuality::Maj),
        ],
        FormCategory::SoulRnB => vec![
            ("Imaj7", key, ChordQuality::Maj7),
            ("i7", key, ChordQuality::Min7),
            ("ii7", key.transpose(2), ChordQuality::Min7),
            ("iii7", key.transpose(4), ChordQuality::Min7),
            ("III7", key.transpose(4), ChordQuality::Dom7),
            ("IVmaj7", key.transpose(5), ChordQuality::Maj7),
            ("IV9", key.transpose(5), ChordQuality::Dom9),
            ("V7", key.transpose(7), ChordQuality::Dom7),
            ("V7sus4", key.transpose(7), ChordQuality::Dom7Sus4),
            ("vi7", key.transpose(9), ChordQuality::Min7),
            ("VI7", key.transpose(9), ChordQuality::Dom7),
            ("bVII7", key.transpose(10), ChordQuality::Dom7),
        ],
    }
}
