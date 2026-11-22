use serde::{Deserialize, Serialize};
use crate::theory::{PitchClass, Progression};

pub const TICKS_PER_BEAT: u32 = 480;
pub const BEATS_PER_BAR: u32 = 4;
pub const TICKS_PER_BAR: u32 = TICKS_PER_BEAT * BEATS_PER_BAR; // 1920

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduledNote {
    pub tick: u32,
    pub duration_ticks: u32,
    pub channel: u8,
    pub note: u8,
    pub velocity: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChordInstrument {
    GrandPiano = 0,
    ElectricPiano = 4,
    DrawbarOrgan = 16,
    ElectricGuitar = 27,
}

impl ChordInstrument {
    pub const ALL: [ChordInstrument; 4] = [
        ChordInstrument::GrandPiano,
        ChordInstrument::ElectricPiano,
        ChordInstrument::DrawbarOrgan,
        ChordInstrument::ElectricGuitar,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ChordInstrument::GrandPiano => "Acoustic Grand Piano",
            ChordInstrument::ElectricPiano => "Rhodes Electric Piano",
            ChordInstrument::DrawbarOrgan => "Hammond B3 Organ",
            ChordInstrument::ElectricGuitar => "Electric Guitar",
        }
    }

    pub fn program_number(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChordCompingStyle {
    SustainedPads,
    ChicagoShuffleChops,
    Slow12_8Triplets,
    FreddieGreenFour,
    CharlestonSyncopated,
    FunkyStabs,
    PopEighthPulse,
    NeoSoulSyncopated,
}

impl ChordCompingStyle {
    pub const ALL: [ChordCompingStyle; 8] = [
        ChordCompingStyle::SustainedPads,
        ChordCompingStyle::ChicagoShuffleChops,
        ChordCompingStyle::Slow12_8Triplets,
        ChordCompingStyle::FreddieGreenFour,
        ChordCompingStyle::CharlestonSyncopated,
        ChordCompingStyle::FunkyStabs,
        ChordCompingStyle::PopEighthPulse,
        ChordCompingStyle::NeoSoulSyncopated,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ChordCompingStyle::SustainedPads => "Sustained Whole-Bar Pads",
            ChordCompingStyle::ChicagoShuffleChops => "Chicago Shuffle Chops (2 & 4)",
            ChordCompingStyle::Slow12_8Triplets => "Slow 12/8 Triplet Arpeggio",
            ChordCompingStyle::FreddieGreenFour => "Freddie Green (Four-on-Floor)",
            ChordCompingStyle::CharlestonSyncopated => "Charleston / Jazz Syncopated",
            ChordCompingStyle::FunkyStabs => "New Orleans Funky Stabs",
            ChordCompingStyle::PopEighthPulse => "Pop / Rock 8th-Note Pulse",
            ChordCompingStyle::NeoSoulSyncopated => "Soul / R&B Neo-Soul Pocket",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BassInstrument {
    UprightBass = 32,
    ElectricBass = 33,
}

impl BassInstrument {
    pub const ALL: [BassInstrument; 2] = [
        BassInstrument::UprightBass,
        BassInstrument::ElectricBass,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BassInstrument::UprightBass => "Upright Acoustic Bass",
            BassInstrument::ElectricBass => "Fingered Electric Bass",
        }
    }

    pub fn program_number(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BassStyle {
    WalkingBass,
    ShuffleBoogie,
    Slow12_8Blues,
    RootFifthTwoFeel,
    EighthRootDrive,
    MotownSoulGroove,
}

impl BassStyle {
    pub const ALL: [BassStyle; 6] = [
        BassStyle::WalkingBass,
        BassStyle::ShuffleBoogie,
        BassStyle::Slow12_8Blues,
        BassStyle::RootFifthTwoFeel,
        BassStyle::EighthRootDrive,
        BassStyle::MotownSoulGroove,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            BassStyle::WalkingBass => "Walking Bass (Quarter Notes)",
            BassStyle::ShuffleBoogie => "Shuffle Boogie (8th Swing)",
            BassStyle::Slow12_8Blues => "Slow 12/8 Blues Bass",
            BassStyle::RootFifthTwoFeel => "Two-Feel (Root - Fifth)",
            BassStyle::EighthRootDrive => "Rock / Pop Driving 8ths",
            BassStyle::MotownSoulGroove => "Motown / Soul Syncopated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DrumBeatPreset {
    ChicagoShuffle,
    Slow12_8Blues,
    TexasSRVShuffle,
    NewOrleansFunk,
    JazzBluesSwing,
    StraightRockBlues,
    PopRockBackbeat,
    MotownSoulBeat,
    NeoSoulPocket,
}

impl DrumBeatPreset {
    pub const ALL: [DrumBeatPreset; 9] = [
        DrumBeatPreset::ChicagoShuffle,
        DrumBeatPreset::Slow12_8Blues,
        DrumBeatPreset::TexasSRVShuffle,
        DrumBeatPreset::NewOrleansFunk,
        DrumBeatPreset::JazzBluesSwing,
        DrumBeatPreset::StraightRockBlues,
        DrumBeatPreset::PopRockBackbeat,
        DrumBeatPreset::MotownSoulBeat,
        DrumBeatPreset::NeoSoulPocket,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            DrumBeatPreset::ChicagoShuffle => "Chicago Shuffle",
            DrumBeatPreset::Slow12_8Blues => "Slow 12/8 Blues",
            DrumBeatPreset::TexasSRVShuffle => "Texas SRV Double-Shuffle",
            DrumBeatPreset::NewOrleansFunk => "New Orleans Second Line Funk",
            DrumBeatPreset::JazzBluesSwing => "Jazz Blues Swing",
            DrumBeatPreset::StraightRockBlues => "Straight Rock Blues",
            DrumBeatPreset::PopRockBackbeat => "Pop / Classic Rock 8th Backbeat",
            DrumBeatPreset::MotownSoulBeat => "Motown Four-on-the-Floor Soul",
            DrumBeatPreset::NeoSoulPocket => "Neo-Soul / R&B Laid-Back Pocket",
        }
    }
}

// GM Drum Notes
pub const GM_DRUM_KICK: u8 = 36;
pub const GM_DRUM_RIMSHOT: u8 = 37;
pub const GM_DRUM_SNARE: u8 = 38;
pub const GM_DRUM_CL_HAT: u8 = 42;
pub const GM_DRUM_PEDAL_HAT: u8 = 44;
pub const _GM_DRUM_OP_HAT: u8 = 46;
pub const GM_DRUM_LOW_TOM: u8 = 45;
pub const GM_DRUM_MID_TOM: u8 = 47;
pub const GM_DRUM_HIGH_TOM: u8 = 50;
pub const _GM_DRUM_CRASH: u8 = 49;
pub const GM_DRUM_RIDE: u8 = 51;
pub const GM_DRUM_RIDE_BELL: u8 = 53;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrangerSettings {
    pub chord_instrument: ChordInstrument,
    pub chord_style: ChordCompingStyle,
    pub chord_volume: f32, // 0.0 to 1.0
    pub chord_enabled: bool,

    pub bass_instrument: BassInstrument,
    pub bass_style: BassStyle,
    pub bass_volume: f32,
    pub bass_enabled: bool,

    pub drum_preset: DrumBeatPreset,
    pub drum_volume: f32,
    pub drum_enabled: bool,
    pub turnaround_drum_fill: bool,

    pub swing_ratio: f32, // 0.50 (straight) to 0.67 (triplet) to 0.75 (heavy shuffle)
    pub bpm: f32,
}

impl Default for ArrangerSettings {
    fn default() -> Self {
        Self {
            chord_instrument: ChordInstrument::GrandPiano,
            chord_style: ChordCompingStyle::ChicagoShuffleChops,
            chord_volume: 0.85,
            chord_enabled: true,

            bass_instrument: BassInstrument::UprightBass,
            bass_style: BassStyle::WalkingBass,
            bass_volume: 0.90,
            bass_enabled: true,

            drum_preset: DrumBeatPreset::ChicagoShuffle,
            drum_volume: 0.80,
            drum_enabled: true,
            turnaround_drum_fill: true,

            swing_ratio: 0.66,
            bpm: 110.0,
        }
    }
}

/// Computes the exact tick for an 8th-note offbeat given swing_ratio
pub fn swung_offbeat_tick(beat_start: u32, swing_ratio: f32) -> u32 {
    let offset = (TICKS_PER_BEAT as f32 * swing_ratio).round() as u32;
    beat_start + offset
}

pub fn generate_arrangement_notes(
    prog: &Progression,
    settings: &ArrangerSettings,
) -> Vec<ScheduledNote> {
    let mut notes = Vec::new();
    let total_bars = prog.bars.len();

    for (bar_idx, bar) in prog.bars.iter().enumerate() {
        let bar_start_tick = (bar_idx as u32) * TICKS_PER_BAR;
        let is_last_bar = bar_idx == total_bars - 1;

        // 1. Chords Track (Channel 1)
        if settings.chord_enabled && settings.chord_volume > 0.0 {
            generate_bar_chords(
                bar_start_tick,
                bar,
                settings,
                &mut notes,
            );
        }

        // 2. Bass Track (Channel 2)
        if settings.bass_enabled && settings.bass_volume > 0.0 {
            let next_bar = if bar_idx + 1 < total_bars {
                &prog.bars[bar_idx + 1]
            } else {
                &prog.bars[0]
            };
            generate_bar_bass(
                bar_start_tick,
                bar,
                next_bar,
                settings,
                &mut notes,
            );
        }

        // 3. Drums Track (Channel 10, index 9)
        if settings.drum_enabled && settings.drum_volume > 0.0 {
            generate_bar_drums(
                bar_start_tick,
                settings,
                is_last_bar,
                &mut notes,
            );
        }
    }

    notes.sort_by_key(|n| n.tick);
    notes
}

fn generate_bar_chords(
    bar_start: u32,
    bar: &crate::theory::BarChords,
    settings: &ArrangerSettings,
    notes: &mut Vec<ScheduledNote>,
) {
    let vel = (100.0 * settings.chord_volume).clamp(20.0, 127.0) as u8;
    let swing = settings.swing_ratio;

    // Depending on slot count, each chord covers 4 beats, 2 beats, or 1 beat
    let slots_count = bar.slots.len();

    match settings.chord_style {
        ChordCompingStyle::SustainedPads => {
            for (idx, chord) in bar.slots.iter().enumerate() {
                let chord_notes = chord.voicing(62);
                let duration = (TICKS_PER_BAR / slots_count as u32) - 40;
                let tick = bar_start + (idx as u32) * (TICKS_PER_BAR / slots_count as u32);
                for note in chord_notes {
                    notes.push(ScheduledNote {
                        tick,
                        duration_ticks: duration,
                        channel: 0,
                        note,
                        velocity: vel,
                    });
                }
            }
        }
        ChordCompingStyle::ChicagoShuffleChops => {
            // Accented chops on beats 2 & 4, with optional swung anticipation on "and of 4"
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let chord = bar.get_chord_at_beat(beat as usize);
                let chord_notes = chord.voicing(63);

                if beat == 1 || beat == 3 {
                    // Beats 2 and 4 chop
                    for note in &chord_notes {
                        notes.push(ScheduledNote {
                            tick: beat_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * 0.5) as u32,
                            channel: 0,
                            note: *note,
                            velocity: (vel as f32 * 1.05).min(127.0) as u8,
                        });
                    }
                } else if beat == 0 {
                    // Short downbeat stab
                    for note in &chord_notes {
                        notes.push(ScheduledNote {
                            tick: beat_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * 0.35) as u32,
                            channel: 0,
                            note: *note,
                            velocity: (vel as f32 * 0.8) as u8,
                        });
                    }
                }

                // Swung offbeat pickup on "and" of beat 4
                if beat == 3 {
                    let offbeat = swung_offbeat_tick(beat_tick, swing);
                    for note in &chord_notes {
                        notes.push(ScheduledNote {
                            tick: offbeat,
                            duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.8) as u32,
                            channel: 0,
                            note: *note,
                            velocity: (vel as f32 * 0.75) as u8,
                        });
                    }
                }
            }
        }
        ChordCompingStyle::Slow12_8Triplets => {
            // 3 triplet pulses per beat = 12 pulses per bar
            let triplet_tick = TICKS_PER_BEAT / 3; // 160
            for beat in 0..4 {
                let chord = bar.get_chord_at_beat(beat as usize);
                let chord_notes = chord.voicing(62);
                for sub in 0..3 {
                    let tick = bar_start + beat * TICKS_PER_BEAT + sub * triplet_tick;
                    let is_downbeat = sub == 0;
                    let sub_vel = if is_downbeat { vel } else { (vel as f32 * 0.75) as u8 };
                    for (i, note) in chord_notes.iter().enumerate() {
                        // arpeggiated ripple
                        let n_tick = tick + (i as u32 * 8);
                        notes.push(ScheduledNote {
                            tick: n_tick,
                            duration_ticks: triplet_tick - 20,
                            channel: 0,
                            note: *note,
                            velocity: sub_vel,
                        });
                    }
                }
            }
        }
        ChordCompingStyle::FreddieGreenFour => {
            // Crisp quarter-note pulse on all 4 beats
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let chord = bar.get_chord_at_beat(beat as usize);
                let chord_notes = chord.voicing(60);
                let b_vel = if beat == 1 || beat == 3 { (vel as f32 * 1.05).min(127.0) as u8 } else { (vel as f32 * 0.9) as u8 };
                for note in chord_notes {
                    notes.push(ScheduledNote {
                        tick: beat_tick,
                        duration_ticks: (TICKS_PER_BEAT as f32 * 0.6) as u32,
                        channel: 0,
                        note,
                        velocity: b_vel,
                    });
                }
            }
        }
        ChordCompingStyle::CharlestonSyncopated => {
            // Charleston rhythm: Beat 1 and the "and" of Beat 2
            let c1 = bar.get_chord_at_beat(0);
            for n in c1.voicing(62) {
                notes.push(ScheduledNote {
                    tick: bar_start,
                    duration_ticks: (TICKS_PER_BEAT as f32 * 1.2) as u32,
                    channel: 0,
                    note: n,
                    velocity: vel,
                });
            }
            let beat1_start = bar_start + TICKS_PER_BEAT;
            let and_beat2 = swung_offbeat_tick(beat1_start, swing);
            let c2 = bar.get_chord_at_beat(1);
            for n in c2.voicing(62) {
                notes.push(ScheduledNote {
                    tick: and_beat2,
                    duration_ticks: (TICKS_PER_BEAT as f32 * 0.8) as u32,
                    channel: 0,
                    note: n,
                    velocity: (vel as f32 * 1.1).min(127.0) as u8,
                });
            }
        }
        ChordCompingStyle::FunkyStabs => {
            // 16th syncopation: Beat 1, and-of-2, beat 4
            let beats = [0, 1, 3];
            for &b in &beats {
                let tick = if b == 1 {
                    swung_offbeat_tick(bar_start + TICKS_PER_BEAT, swing)
                } else {
                    bar_start + b * TICKS_PER_BEAT
                };
                let chord = bar.get_chord_at_beat(b as usize);
                for n in chord.voicing(63) {
                    notes.push(ScheduledNote {
                        tick,
                        duration_ticks: (TICKS_PER_BEAT as f32 * 0.35) as u32,
                        channel: 0,
                        note: n,
                        velocity: vel,
                    });
                }
            }
        }
        ChordCompingStyle::PopEighthPulse => {
            // Steady driving 8th-note pulse
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(beat_tick, swing);
                let chord = bar.get_chord_at_beat(beat as usize);
                let chord_notes = chord.voicing(62);

                let on_vel = match beat {
                    0 => (vel as f32 * 1.05).min(127.0) as u8,
                    2 => vel,
                    _ => (vel as f32 * 0.9) as u8,
                };
                let off_vel = (vel as f32 * 0.78) as u8;

                for &note in &chord_notes {
                    notes.push(ScheduledNote {
                        tick: beat_tick,
                        duration_ticks: (TICKS_PER_BEAT as f32 * swing * 0.75) as u32,
                        channel: 0,
                        note,
                        velocity: on_vel,
                    });
                    notes.push(ScheduledNote {
                        tick: off_tick,
                        duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.75) as u32,
                        channel: 0,
                        note,
                        velocity: off_vel,
                    });
                }
            }
        }
        ChordCompingStyle::NeoSoulSyncopated => {
            // Lush beat 1 + anticipated syncopation on and-of-2 and pickup on and-of-4
            let c1 = bar.get_chord_at_beat(0);
            for n in c1.voicing(62) {
                notes.push(ScheduledNote {
                    tick: bar_start,
                    duration_ticks: (TICKS_PER_BEAT as f32 * 1.3) as u32,
                    channel: 0,
                    note: n,
                    velocity: (vel as f32 * 0.95) as u8,
                });
            }
            let and_beat2 = swung_offbeat_tick(bar_start + TICKS_PER_BEAT, swing);
            let c2 = bar.get_chord_at_beat(1);
            for n in c2.voicing(63) {
                notes.push(ScheduledNote {
                    tick: and_beat2,
                    duration_ticks: (TICKS_PER_BEAT as f32 * 1.1) as u32,
                    channel: 0,
                    note: n,
                    velocity: (vel as f32 * 1.05).min(127.0) as u8,
                });
            }
            let and_beat4 = swung_offbeat_tick(bar_start + 3 * TICKS_PER_BEAT, swing);
            let c4 = bar.get_chord_at_beat(3);
            for n in c4.voicing(62) {
                notes.push(ScheduledNote {
                    tick: and_beat4,
                    duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.9) as u32,
                    channel: 0,
                    note: n,
                    velocity: (vel as f32 * 0.75) as u8,
                });
            }
        }
    }
}

fn bass_root_midi(pitch: PitchClass) -> u8 {
    let semitone = pitch.semitones();
    // Keep bass in punchy octave C2..B2 (MIDI 36..47)
    36 + semitone
}

fn generate_bar_bass(
    bar_start: u32,
    bar: &crate::theory::BarChords,
    next_bar: &crate::theory::BarChords,
    settings: &ArrangerSettings,
    notes: &mut Vec<ScheduledNote>,
) {
    let vel = (105.0 * settings.bass_volume).clamp(20.0, 127.0) as u8;
    let swing = settings.swing_ratio;

    let target_next_root = bass_root_midi(next_bar.get_chord_at_beat(0).root);

    match settings.bass_style {
        BassStyle::WalkingBass => {
            // 4 quarter notes walking bass
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let chord = bar.get_chord_at_beat(beat as usize);
                let root = bass_root_midi(chord.root);

                let note_pitch = match beat {
                    0 => root,
                    1 => {
                        // Major 3rd or Minor 3rd depending on quality
                        let is_min = matches!(chord.quality, crate::theory::ChordQuality::Min | crate::theory::ChordQuality::Min7 | crate::theory::ChordQuality::Min6 | crate::theory::ChordQuality::Min9 | crate::theory::ChordQuality::HalfDim7 | crate::theory::ChordQuality::Dim7);
                        root + if is_min { 3 } else { 4 }
                    }
                    2 => root + 7, // Perfect 5th
                    3 => {
                        // Chromatic approach to next chord root
                        if target_next_root > root {
                            target_next_root - 1
                        } else {
                            target_next_root + 1
                        }
                    }
                    _ => root,
                };

                notes.push(ScheduledNote {
                    tick: beat_tick,
                    duration_ticks: (TICKS_PER_BEAT as f32 * 0.85) as u32,
                    channel: 1,
                    note: note_pitch.clamp(28, 55),
                    velocity: vel,
                });
            }
        }
        BassStyle::ShuffleBoogie => {
            // Texas/Chicago boogie: root-5th-6th-5th or root-root-5th-5th-6th-6th-b7th-6th
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let offbeat = swung_offbeat_tick(beat_tick, swing);
                let chord = bar.get_chord_at_beat(beat as usize);
                let root = bass_root_midi(chord.root);

                let (p1, p2) = match beat {
                    0 => (root, root),
                    1 => (root + 7, root + 7), // 5th
                    2 => (root + 9, root + 9), // 6th
                    3 => (root + 10, root + 9), // b7th, 6th
                    _ => (root, root),
                };

                notes.push(ScheduledNote {
                    tick: beat_tick,
                    duration_ticks: (TICKS_PER_BEAT as f32 * swing * 0.8) as u32,
                    channel: 1,
                    note: p1.clamp(28, 55),
                    velocity: vel,
                });
                notes.push(ScheduledNote {
                    tick: offbeat,
                    duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.8) as u32,
                    channel: 1,
                    note: p2.clamp(28, 55),
                    velocity: (vel as f32 * 0.85) as u8,
                });
            }
        }
        BassStyle::Slow12_8Blues => {
            // Root on beat 1, sustained with pickup on beat 4
            let root = bass_root_midi(bar.get_chord_at_beat(0).root);
            notes.push(ScheduledNote {
                tick: bar_start,
                duration_ticks: TICKS_PER_BEAT * 3,
                channel: 1,
                note: root,
                velocity: vel,
            });
            // Pickup to next bar
            let pickup_tick = bar_start + 3 * TICKS_PER_BEAT + (TICKS_PER_BEAT / 3) * 2;
            notes.push(ScheduledNote {
                tick: pickup_tick,
                duration_ticks: TICKS_PER_BEAT / 3,
                channel: 1,
                note: (target_next_root as i32 - 1).clamp(28, 55) as u8,
                velocity: (vel as f32 * 0.9) as u8,
            });
        }
        BassStyle::RootFifthTwoFeel => {
            let chord1 = bar.get_chord_at_beat(0);
            let chord2 = bar.get_chord_at_beat(2);
            let r1 = bass_root_midi(chord1.root);
            let r2 = bass_root_midi(chord2.root) + 7;

            notes.push(ScheduledNote {
                tick: bar_start,
                duration_ticks: (TICKS_PER_BEAT * 2) - 80,
                channel: 1,
                note: r1,
                velocity: vel,
            });
            notes.push(ScheduledNote {
                tick: bar_start + 2 * TICKS_PER_BEAT,
                duration_ticks: (TICKS_PER_BEAT * 2) - 80,
                channel: 1,
                note: (r2 - 12).clamp(28, 55),
                velocity: vel,
            });
        }
        BassStyle::EighthRootDrive => {
            // Driving 8th-note rock/pop bass line
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(beat_tick, swing);
                let chord = bar.get_chord_at_beat(beat as usize);
                let root = bass_root_midi(chord.root);

                let (p1, p2) = match beat {
                    3 => (root, if target_next_root > root { target_next_root - 1 } else { target_next_root + 1 }),
                    _ => (root, root),
                };

                notes.push(ScheduledNote {
                    tick: beat_tick,
                    duration_ticks: (TICKS_PER_BEAT as f32 * swing * 0.8) as u32,
                    channel: 1,
                    note: p1.clamp(28, 55),
                    velocity: vel,
                });
                notes.push(ScheduledNote {
                    tick: off_tick,
                    duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.8) as u32,
                    channel: 1,
                    note: p2.clamp(28, 55),
                    velocity: (vel as f32 * 0.88) as u8,
                });
            }
        }
        BassStyle::MotownSoulGroove => {
            // James Jamerson style syncopated soul line:
            // Beat 1: Root (dotted quarter)
            // Beat 2-and: 5th
            // Beat 3: 6th
            // Beat 4 & 4-and: Chromatic approach to next root
            for beat in 0..4 {
                let beat_tick = bar_start + beat * TICKS_PER_BEAT;
                let chord = bar.get_chord_at_beat(beat as usize);
                let root = bass_root_midi(chord.root);

                match beat {
                    0 => {
                        notes.push(ScheduledNote {
                            tick: beat_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * 1.3) as u32,
                            channel: 1,
                            note: root.clamp(28, 55),
                            velocity: (vel as f32 * 1.05).min(127.0) as u8,
                        });
                    }
                    1 => {
                        let off_tick = swung_offbeat_tick(beat_tick, swing);
                        notes.push(ScheduledNote {
                            tick: off_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.9) as u32,
                            channel: 1,
                            note: (root + 7).clamp(28, 55),
                            velocity: (vel as f32 * 0.95) as u8,
                        });
                    }
                    2 => {
                        notes.push(ScheduledNote {
                            tick: beat_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * 0.8) as u32,
                            channel: 1,
                            note: (root + 9).clamp(28, 55),
                            velocity: vel,
                        });
                    }
                    3 => {
                        let off_tick = swung_offbeat_tick(beat_tick, swing);
                        let approach = if target_next_root > root { target_next_root - 1 } else { target_next_root + 1 };
                        notes.push(ScheduledNote {
                            tick: beat_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * swing * 0.8) as u32,
                            channel: 1,
                            note: (root + 7).clamp(28, 55),
                            velocity: (vel as f32 * 0.9) as u8,
                        });
                        notes.push(ScheduledNote {
                            tick: off_tick,
                            duration_ticks: (TICKS_PER_BEAT as f32 * (1.0 - swing) * 0.8) as u32,
                            channel: 1,
                            note: approach.clamp(28, 55),
                            velocity: vel,
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn generate_bar_drums(
    bar_start: u32,
    settings: &ArrangerSettings,
    is_last_bar: bool,
    notes: &mut Vec<ScheduledNote>,
) {
    let vel = (100.0 * settings.drum_volume).clamp(20.0, 127.0) as u8;
    let swing = settings.swing_ratio;

    let add_drum = |notes: &mut Vec<ScheduledNote>, tick: u32, note: u8, v: u8| {
        notes.push(ScheduledNote {
            tick,
            duration_ticks: 120,
            channel: 9, // GM Drum Channel 10 is 0-indexed 9
            note,
            velocity: v,
        });
    };

    // If turnaround drum fill is active on the last bar, generate a crescendo triplet snare/tom fill in beats 3 & 4
    if is_last_bar && settings.turnaround_drum_fill {
        // Normal groove on beats 1 & 2
        add_drum(notes, bar_start, GM_DRUM_KICK, vel);
        add_drum(notes, bar_start, GM_DRUM_RIDE, (vel as f32 * 0.9) as u8);
        let off1 = swung_offbeat_tick(bar_start, swing);
        add_drum(notes, off1, GM_DRUM_RIDE, (vel as f32 * 0.7) as u8);

        let beat2_tick = bar_start + TICKS_PER_BEAT;
        add_drum(notes, beat2_tick, GM_DRUM_SNARE, (vel as f32 * 1.1).min(127.0) as u8);
        add_drum(notes, beat2_tick, GM_DRUM_RIDE, (vel as f32 * 0.9) as u8);

        // Beats 3 & 4: Snare + High Tom + Mid Tom + Low Tom triplet fill
        let triplet_tick = TICKS_PER_BEAT / 3;
        let fill_start = bar_start + 2 * TICKS_PER_BEAT;
        let fill_drums = [
            GM_DRUM_SNARE, GM_DRUM_SNARE, GM_DRUM_HIGH_TOM,
            GM_DRUM_HIGH_TOM, GM_DRUM_MID_TOM, GM_DRUM_LOW_TOM,
        ];
        for (i, &drum) in fill_drums.iter().enumerate() {
            let f_tick = fill_start + (i as u32) * triplet_tick;
            let f_vel = ((vel as f32 * 0.7) + (i as f32 * 9.0)).min(127.0) as u8;
            add_drum(notes, f_tick, drum, f_vel);
        }
        return;
    }

    match settings.drum_preset {
        DrumBeatPreset::ChicagoShuffle => {
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                // Ride shuffle pattern
                add_drum(notes, b_tick, GM_DRUM_RIDE, (vel as f32 * 0.95) as u8);
                add_drum(notes, off_tick, GM_DRUM_RIDE, (vel as f32 * 0.75) as u8);

                // Kick on 1 and 3
                if beat == 0 || beat == 2 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                }

                // Snare on 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.1).min(127.0) as u8);
                }

                // Pedal hi-hat chick on 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_PEDAL_HAT, (vel as f32 * 0.7) as u8);
                }
            }
        }
        DrumBeatPreset::Slow12_8Blues => {
            let triplet = TICKS_PER_BEAT / 3;
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                // Full 12/8 bell/ride pattern: 3 hits per beat
                for sub in 0..3 {
                    let s_tick = b_tick + sub * triplet;
                    let v = if sub == 0 { (vel as f32 * 0.95) as u8 } else { (vel as f32 * 0.65) as u8 };
                    add_drum(notes, s_tick, GM_DRUM_RIDE_BELL, v);
                }

                // Kick on beat 1 and pickup
                if beat == 0 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                } else if beat == 2 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, (vel as f32 * 0.85) as u8);
                }

                // Snare backbeat on beats 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.15).min(127.0) as u8);
                }
            }
        }
        DrumBeatPreset::TexasSRVShuffle => {
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                // Driving double-shuffle hat
                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.9) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.8) as u8);

                // Solid kick on every beat
                add_drum(notes, b_tick, GM_DRUM_KICK, (vel as f32 * 0.95) as u8);

                // Snare backbeat on 2 and 4 with ghost notes on offbeats
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.15).min(127.0) as u8);
                } else {
                    add_drum(notes, off_tick, GM_DRUM_SNARE, (vel as f32 * 0.45) as u8);
                }
            }
        }
        DrumBeatPreset::NewOrleansFunk => {
            // Second line syncopated groove
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.85) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.7) as u8);

                match beat {
                    0 => {
                        add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                    }
                    1 => {
                        add_drum(notes, b_tick, GM_DRUM_RIMSHOT, (vel as f32 * 0.9) as u8);
                        add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.85) as u8);
                    }
                    2 => {
                        add_drum(notes, off_tick, GM_DRUM_SNARE, (vel as f32 * 0.6) as u8);
                    }
                    3 => {
                        add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.1).min(127.0) as u8);
                        add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.75) as u8);
                    }
                    _ => {}
                }
            }
        }
        DrumBeatPreset::JazzBluesSwing => {
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                // "Spang-a-lang" jazz ride pattern:
                // Beat 1: Ding
                // Beat 2: Ding - Da (on offbeat)
                // Beat 3: Ding
                // Beat 4: Ding - Da (on offbeat)
                add_drum(notes, b_tick, GM_DRUM_RIDE, (vel as f32 * 0.85) as u8);
                if beat == 1 || beat == 3 {
                    add_drum(notes, off_tick, GM_DRUM_RIDE, (vel as f32 * 0.7) as u8);
                }

                // Feathered kick on all 4 beats
                add_drum(notes, b_tick, GM_DRUM_KICK, (vel as f32 * 0.5) as u8);

                // Hi-Hat chick on beats 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_PEDAL_HAT, (vel as f32 * 0.8) as u8);
                }
            }
        }
        DrumBeatPreset::StraightRockBlues => {
            let half_beat = TICKS_PER_BEAT / 2;
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = b_tick + half_beat;

                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.9) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.75) as u8);

                if beat == 0 || beat == 2 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                }
                if beat == 2 {
                    add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.85) as u8);
                }
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.15).min(127.0) as u8);
                }
            }
        }
        DrumBeatPreset::PopRockBackbeat => {
            let half_beat = TICKS_PER_BEAT / 2;
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = b_tick + half_beat;

                // Steady 8th hats
                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.92) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.75) as u8);

                // Kick on 1, 3, and offbeat of 3
                if beat == 0 || beat == 2 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                }
                if beat == 2 {
                    add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.88) as u8);
                }

                // Snare on 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.15).min(127.0) as u8);
                }
            }
        }
        DrumBeatPreset::MotownSoulBeat => {
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                // Four-on-the-floor kick
                add_drum(notes, b_tick, GM_DRUM_KICK, (vel as f32 * 0.95) as u8);

                // Driving hat 8ths
                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.85) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.7) as u8);

                // Solid snare on 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_SNARE, (vel as f32 * 1.2).min(127.0) as u8);
                }
            }
        }
        DrumBeatPreset::NeoSoulPocket => {
            for beat in 0..4 {
                let b_tick = bar_start + beat * TICKS_PER_BEAT;
                let off_tick = swung_offbeat_tick(b_tick, swing);

                // Closed hat with light dynamics
                add_drum(notes, b_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.82) as u8);
                add_drum(notes, off_tick, GM_DRUM_CL_HAT, (vel as f32 * 0.65) as u8);

                // Laid-back kick pattern (1, and-of-2, and 3-and)
                if beat == 0 {
                    add_drum(notes, b_tick, GM_DRUM_KICK, vel);
                } else if beat == 1 {
                    add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.8) as u8);
                } else if beat == 2 {
                    add_drum(notes, off_tick, GM_DRUM_KICK, (vel as f32 * 0.75) as u8);
                }

                // Rimshot / snare on 2 and 4
                if beat == 1 || beat == 3 {
                    add_drum(notes, b_tick, GM_DRUM_RIMSHOT, (vel as f32 * 1.1).min(127.0) as u8);
                }
            }
        }
    }
}
