use std::fs::File;
use std::io::Write;
use std::path::Path;
use midly::{
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent,
    TrackEventKind, num::{u4, u7, u24, u28},
};
use crate::arranger::{ArrangerSettings, ScheduledNote, TICKS_PER_BAR, TICKS_PER_BEAT};
use crate::theory::Progression;

pub fn export_smf_file<P: AsRef<Path>>(
    path: P,
    prog: &Progression,
    settings: &ArrangerSettings,
    chorus_count: usize,
) -> Result<(), String> {
    let bytes = build_smf_bytes(prog, settings, chorus_count)?;
    let mut file = File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&bytes).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

pub fn build_smf_bytes(
    prog: &Progression,
    settings: &ArrangerSettings,
    chorus_count: usize,
) -> Result<Vec<u8>, String> {
    let total_bars = prog.bars.len();
    let chorus_ticks = (total_bars as u32) * TICKS_PER_BAR;
    let single_chorus_notes = crate::arranger::generate_arrangement_notes(prog, settings);

    // Duplicate notes across requested choruses
    let mut all_notes: Vec<ScheduledNote> = Vec::new();
    for chorus in 0..chorus_count {
        let offset = (chorus as u32) * chorus_ticks;
        for n in &single_chorus_notes {
            let mut note = *n;
            note.tick += offset;
            all_notes.push(note);
        }
    }

    // Standard MIDI Header: Format 1, 480 PPQ
    let header = Header::new(Format::Parallel, Timing::Metrical(u15_from_u32(TICKS_PER_BEAT)));
    let mut smf = Smf::new(header);

    // --- Track 0: Conductor Track (Tempo, Time Sig, Chord markers) ---
    let mut track0 = Track::new();
    // Tempo in microseconds per quarter note (60_000_000 / BPM)
    let tempo_us = (60_000_000.0 / settings.bpm.max(20.0)).round() as u32;
    track0.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::TrackName(b"Conductor")),
    });
    track0.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::Tempo(u24::from(tempo_us))),
    });
    track0.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::TimeSignature(4, 2, 24, 8)),
    });

    // Chord markers on track 0
    let mut marker_strings: Vec<String> = Vec::new();
    let mut marker_deltas: Vec<u32> = Vec::new();

    let mut last_marker_tick: u32 = 0;
    for chorus in 0..chorus_count {
        let chorus_offset = (chorus as u32) * chorus_ticks;
        for (b_idx, bar) in prog.bars.iter().enumerate() {
            let bar_tick = chorus_offset + (b_idx as u32) * TICKS_PER_BAR;
            let slot_count = bar.slots.len();
            for (s_idx, chord) in bar.slots.iter().enumerate() {
                let slot_tick = bar_tick + (s_idx as u32) * (TICKS_PER_BAR / slot_count as u32);
                let delta = slot_tick - last_marker_tick;
                last_marker_tick = slot_tick;
                marker_strings.push(chord.name(false));
                marker_deltas.push(delta);
            }
        }
    }

    for (str_val, delta) in marker_strings.iter().zip(marker_deltas.iter()) {
        track0.push(TrackEvent {
            delta: u28::from(*delta),
            kind: TrackEventKind::Meta(MetaMessage::Marker(str_val.as_bytes())),
        });
    }
    // End of track 0
    track0.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });
    smf.tracks.push(track0);

    // --- Track 1: Chords (Channel 1, 0-indexed 0) ---
    let chord_notes: Vec<&ScheduledNote> = all_notes.iter().filter(|n| n.channel == 0).collect();
    let track1 = build_channel_track(b"Chords", 0, settings.chord_instrument.program_number(), &chord_notes);
    smf.tracks.push(track1);

    // --- Track 2: Bass (Channel 2, 0-indexed 1) ---
    if settings.bass_enabled {
        let bass_notes: Vec<&ScheduledNote> = all_notes.iter().filter(|n| n.channel == 1).collect();
        let track2 = build_channel_track(b"Bass", 1, settings.bass_instrument.program_number(), &bass_notes);
        smf.tracks.push(track2);
    }

    // --- Track 3: Drums (Channel 10, 0-indexed 9) ---
    if settings.drum_enabled {
        let drum_notes: Vec<&ScheduledNote> = all_notes.iter().filter(|n| n.channel == 9).collect();
        let track3 = build_channel_track(b"Drums", 9, settings.drum_kit.program_number(), &drum_notes);
        smf.tracks.push(track3);
    }

    let mut out_bytes = Vec::new();
    smf.write(&mut out_bytes).map_err(|e| format!("Failed to encode SMF: {}", e))?;
    Ok(out_bytes)
}

fn u15_from_u32(val: u32) -> midly::num::u15 {
    midly::num::u15::from((val.min(32767)) as u16)
}

#[derive(Clone, Copy)]
enum RawMidiAction {
    NoteOn { note: u8, vel: u8 },
    NoteOff { note: u8 },
}

struct RawTimedEvent {
    tick: u32,
    channel: u8,
    action: RawMidiAction,
}

fn build_channel_track(
    name: &'static [u8],
    channel: u8,
    program: u8,
    notes: &[&ScheduledNote],
) -> Track<'static> {
    let mut track = Track::new();
    track.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::TrackName(name)),
    });

    // Program change
    track.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Midi {
            channel: u4::from(channel),
            message: MidiMessage::ProgramChange {
                program: u7::from(program.min(127)),
            },
        },
    });

    let mut raw_events: Vec<RawTimedEvent> = Vec::new();
    for n in notes {
        raw_events.push(RawTimedEvent {
            tick: n.tick,
            channel: n.channel,
            action: RawMidiAction::NoteOn { note: n.note, vel: n.velocity },
        });
        raw_events.push(RawTimedEvent {
            tick: n.tick + n.duration_ticks,
            channel: n.channel,
            action: RawMidiAction::NoteOff { note: n.note },
        });
    }

    // Stable sort by tick. NoteOff before NoteOn if ticks match to avoid hanging notes.
    raw_events.sort_by(|a, b| {
        if a.tick != b.tick {
            a.tick.cmp(&b.tick)
        } else {
            match (a.action, b.action) {
                (RawMidiAction::NoteOff { .. }, RawMidiAction::NoteOn { .. }) => std::cmp::Ordering::Less,
                (RawMidiAction::NoteOn { .. }, RawMidiAction::NoteOff { .. }) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        }
    });

    let mut last_tick: u32 = 0;
    for ev in raw_events {
        let delta = ev.tick.saturating_sub(last_tick);
        last_tick = ev.tick;

        let midi_msg = match ev.action {
            RawMidiAction::NoteOn { note, vel } => MidiMessage::NoteOn {
                key: u7::from(note),
                vel: u7::from(vel),
            },
            RawMidiAction::NoteOff { note } => MidiMessage::NoteOff {
                key: u7::from(note),
                vel: u7::from(0u8),
            },
        };

        track.push(TrackEvent {
            delta: u28::from(delta),
            kind: TrackEventKind::Midi {
                channel: u4::from(ev.channel),
                message: midi_msg,
            },
        });
    }

    track.push(TrackEvent {
        delta: u28::from(0u32),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    track
}
