use std::io::Cursor;
use std::sync::{
    atomic::{AtomicBool, AtomicU32, Ordering},
    Arc, Mutex,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};

use crate::arranger::{ArrangerSettings, ScheduledNote, TICKS_PER_BAR};
use crate::soundfont_gen::generate_gm_soundfont;
use crate::theory::Progression;

pub struct AudioEngine {
    pub is_playing: Arc<AtomicBool>,
    pub current_tick: Arc<AtomicU32>,
    pub total_ticks: Arc<AtomicU32>,
    pub shared_data: Arc<Mutex<AudioSharedData>>,
    _stream: Option<cpal::Stream>,
}

pub struct AudioSharedData {
    pub notes: Vec<ScheduledNote>,
    pub bpm: f32,
    pub total_ticks: u32,
    pub chord_instrument: u8,
    pub bass_instrument: u8,
    pub drum_kit: u8,
    pub preview_notes: Vec<u8>,
    pub preview_trigger: bool,
}

impl AudioEngine {
    pub fn new() -> Result<Self, String> {
        let is_playing = Arc::new(AtomicBool::new(false));
        let current_tick = Arc::new(AtomicU32::new(0));
        let total_ticks = Arc::new(AtomicU32::new(12 * TICKS_PER_BAR));

        let shared_data = Arc::new(Mutex::new(AudioSharedData {
            notes: Vec::new(),
            bpm: 110.0,
            total_ticks: 12 * TICKS_PER_BAR,
            chord_instrument: 0,
            bass_instrument: 32,
            drum_kit: 0,
            preview_notes: Vec::new(),
            preview_trigger: false,
        }));

        // Build in-memory SF2
        let sf2_bytes = generate_gm_soundfont();
        let mut cursor = Cursor::new(sf2_bytes);
        let sf = SoundFont::new(&mut cursor).map_err(|e| format!("Failed to parse SoundFont: {:?}", e))?;
        let sf_arc = Arc::new(sf);

        // Setup CPAL Audio Stream
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| "No audio output device found".to_string())?;

        let config = device
            .default_output_config()
            .map_err(|e| format!("Failed to get default output config: {}", e))?;

        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let syn_settings = SynthesizerSettings::new(sample_rate as i32);
        let mut synth = Synthesizer::new(&sf_arc, &syn_settings)
            .map_err(|e| format!("Failed to create synthesizer: {:?}", e))?;

        // Initialize program changes
        synth.process_midi_message(0, 0xC0, 0, 0); // Ch 1 Piano
        synth.process_midi_message(1, 0xC0, 32, 0); // Ch 2 Upright Bass
        synth.process_midi_message(9, 0xC0, 0, 0); // Ch 10 Drums

        let is_playing_clone = Arc::clone(&is_playing);
        let current_tick_clone = Arc::clone(&current_tick);
        let shared_data_clone = Arc::clone(&shared_data);

        // Circular buffer or sample generator state
        let mut left_buf = vec![0.0f32; 512];
        let mut right_buf = vec![0.0f32; 512];
        let mut last_prog_chord = 255u8;
        let mut last_prog_bass = 255u8;
        let mut last_prog_drum = 255u8;
        let mut play_head_sample = 0.0f64;

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    render_audio_block(
                        data,
                        channels,
                        sample_rate,
                        &mut synth,
                        &mut left_buf,
                        &mut right_buf,
                        &is_playing_clone,
                        &current_tick_clone,
                        &shared_data_clone,
                        &mut last_prog_chord,
                        &mut last_prog_bass,
                        &mut last_prog_drum,
                        &mut play_head_sample,
                    );
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            ),
            _ => return Err("Unsupported sample format".to_string()),
        }
        .map_err(|e| format!("Failed to build audio stream: {}", e))?;

        stream.play().map_err(|e| format!("Failed to start audio stream: {}", e))?;

        Ok(Self {
            is_playing,
            current_tick,
            total_ticks,
            shared_data,
            _stream: Some(stream),
        })
    }

    pub fn set_playing(&self, playing: bool) {
        self.is_playing.store(playing, Ordering::Relaxed);
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn set_tick(&self, tick: u32) {
        self.current_tick.store(tick, Ordering::Relaxed);
    }

    pub fn current_tick(&self) -> u32 {
        self.current_tick.load(Ordering::Relaxed)
    }

    pub fn update_arrangement(
        &self,
        prog: &Progression,
        settings: &ArrangerSettings,
    ) {
        let notes = crate::arranger::generate_arrangement_notes(prog, settings);
        let total = (prog.bars.len() as u32) * TICKS_PER_BAR;
        self.total_ticks.store(total, Ordering::Relaxed);

        if let Ok(mut lock) = self.shared_data.lock() {
            lock.notes = notes;
            lock.bpm = settings.bpm;
            lock.total_ticks = total;
            lock.chord_instrument = settings.chord_instrument.program_number();
            lock.bass_instrument = settings.bass_instrument.program_number();
            lock.drum_kit = settings.drum_kit.program_number();
        }
    }

    pub fn preview_chord(&self, chord: &crate::theory::Chord) {
        let notes = chord.voicing(60);
        if let Ok(mut lock) = self.shared_data.lock() {
            lock.preview_notes = notes;
            lock.preview_trigger = true;
        }
    }
}

fn render_audio_block(
    output: &mut [f32],
    channels: usize,
    sample_rate: u32,
    synth: &mut Synthesizer,
    left_buf: &mut Vec<f32>,
    right_buf: &mut Vec<f32>,
    is_playing: &Arc<AtomicBool>,
    current_tick: &Arc<AtomicU32>,
    shared_data: &Arc<Mutex<AudioSharedData>>,
    last_prog_chord: &mut u8,
    last_prog_bass: &mut u8,
    last_prog_drum: &mut u8,
    _play_head_sample: &mut f64,
) {
    let num_frames = output.len() / channels;

    if left_buf.len() < num_frames {
        left_buf.resize(num_frames, 0.0);
        right_buf.resize(num_frames, 0.0);
    }

    // Process triggers & events under quick lock
    if let Ok(mut data) = shared_data.try_lock() {
        // Check program changes
        if data.chord_instrument != *last_prog_chord {
            *last_prog_chord = data.chord_instrument;
            synth.process_midi_message(0, 0xC0, *last_prog_chord as i32, 0);
        }
        if data.bass_instrument != *last_prog_bass {
            *last_prog_bass = data.bass_instrument;
            synth.process_midi_message(1, 0xC0, *last_prog_bass as i32, 0);
        }
        if data.drum_kit != *last_prog_drum {
            *last_prog_drum = data.drum_kit;
            synth.process_midi_message(9, 0xC0, *last_prog_drum as i32, 0);
        }

        // Preview trigger
        if data.preview_trigger {
            data.preview_trigger = false;
            // Stop previous notes on ch 0
            for note in 0..128 {
                synth.note_off(0, note);
            }
            for &n in &data.preview_notes {
                synth.note_on(0, n as i32, 100);
            }
        }

        // If playing, schedule note on/offs
        let playing = is_playing.load(Ordering::Relaxed);
        if playing && data.total_ticks > 0 {
            let samples_per_beat = (sample_rate as f64 * 60.0) / data.bpm.max(20.0) as f64;
            let samples_per_tick = samples_per_beat / (crate::arranger::TICKS_PER_BEAT as f64);

            let cur_tick = current_tick.load(Ordering::Relaxed);
            let frame_duration_ticks = (num_frames as f64 / samples_per_tick) as u32;
            let next_tick = (cur_tick + frame_duration_ticks) % data.total_ticks;

            // Dispatch notes that start within this frame
            let mut triggered_ons: Vec<(u8, u8, u8)> = Vec::new();
            let mut triggered_offs: Vec<(u8, u8)> = Vec::new();

            for note in &data.notes {
                let note_start = note.tick;
                let note_end = note.tick + note.duration_ticks;

                let triggered_start = if next_tick >= cur_tick {
                    note_start >= cur_tick && note_start < next_tick
                } else {
                    // wrapped around loop
                    note_start >= cur_tick || note_start < next_tick
                };

                let triggered_end = if next_tick >= cur_tick {
                    note_end >= cur_tick && note_end < next_tick
                } else {
                    note_end >= cur_tick || note_end < next_tick
                };

                if triggered_start {
                    triggered_ons.push((note.channel, note.note, note.velocity));
                }
                if triggered_end {
                    triggered_offs.push((note.channel, note.note));
                }
            }

            for (ch, note, vel) in triggered_ons {
                synth.note_on(ch as i32, note as i32, vel as i32);
            }

            for (ch, note) in triggered_offs {
                synth.note_off(ch as i32, note as i32);
            }

            current_tick.store(next_tick, Ordering::Relaxed);
        }
    }

    // Synthesize samples
    let l_slice = &mut left_buf[..num_frames];
    let r_slice = &mut right_buf[..num_frames];
    synth.render(l_slice, r_slice);

    // Write to CPAL interleaved buffer
    for i in 0..num_frames {
        if channels == 1 {
            output[i] = (l_slice[i] + r_slice[i]) * 0.5;
        } else {
            output[i * channels] = l_slice[i];
            output[i * channels + 1] = r_slice[i];
            for c in 2..channels {
                output[i * channels + c] = 0.0;
            }
        }
    }
}
