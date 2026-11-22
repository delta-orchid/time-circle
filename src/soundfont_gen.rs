use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct SampleMeta {
    pub name: &'static str,
    pub start: u32,
    pub end: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub root_key: u8,
}

/// Generates a valid SoundFont 2.04 (.sf2) in memory containing GM instruments:
/// Melodic presets (Bank 0):
/// - 0: Acoustic Grand Piano
/// - 4: Electric Piano (Rhodes style)
/// - 7: Clavinet D6
/// - 11: Vibraphone / Jazz Vibes
/// - 16: Drawbar Organ
/// - 24: Acoustic Nylon/Steel Guitar
/// - 27: Clean Electric Guitar
/// - 32: Acoustic Upright Bass
/// - 33: Electric Fingered Bass
/// - 34: Picked Electric Bass
/// - 35: Fretless Bass
/// - 36: Slap & Pop Bass
/// - 38: Moog Synth Bass
/// - 39: 808 Sub Bass
/// - 48: String Ensemble
/// - 58: Acoustic Tuba / Low Brass
/// - 61: Brass Section
/// - 89: Warm Synth Pad
///
/// Drum Kits (Bank 128):
/// - 0: Standard Acoustic Drum Kit
/// - 16: Rock Power Arena Kit
/// - 24: Vintage Funk Breakbeat Kit
/// - 25: TR-808 Electronic Synth Kit
/// - 32: Lo-Fi Hip Hop Kit
/// - 40: Jazz Brush Kit
pub fn generate_gm_soundfont() -> Vec<u8> {
    let mut sf2 = Vec::new();

    // 1. Synthesize PCM samples (16-bit mono @ 44100 Hz)
    let sample_rate = 44100u32;
    let mut pcm_samples: Vec<i16> = Vec::new();
    let mut metas: Vec<SampleMeta> = Vec::new();

    // Helper to append a generated sample with 46 padding zero samples (SF2 spec requires 46 extra samples)
    let mut add_sample = |name: &'static str, root_key: u8, loop_start_offset: Option<u32>, raw_data: Vec<i16>| {
        let start = pcm_samples.len() as u32;
        let sample_len = raw_data.len() as u32;
        pcm_samples.extend(raw_data);
        let end = (pcm_samples.len() - 1) as u32;

        let (loop_start, loop_end) = match loop_start_offset {
            Some(offset) => (start + offset, end),
            None => (start + sample_len / 2, end),
        };

        // SF2 spec: at least 46 zero-value sample points after the data point
        for _ in 0..46 {
            pcm_samples.push(0);
        }

        metas.push(SampleMeta {
            name,
            start,
            end,
            loop_start,
            loop_end,
            root_key,
        });
    };

    let pi = std::f32::consts::PI;

    // --- Melodic Samples (0..17) ---

    // Sample 0: Piano (Multi-harmonic decaying tone, Root C4 = 60, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63; // C4
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.2).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.6
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.3
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.15
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.08;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Piano_C4", 60, None, data);
    }

    // Sample 1: Electric Piano (Warm bell + sine decay, Root C4 = 60, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.5).exp();
            let bell_env = (-t * 12.0).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.7
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.25
                + (2.0 * pi * freq * 7.0 * t).sin() * 0.3 * bell_env;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("EPiano_C4", 60, None, data);
    }

    // Sample 2: Clavinet (Sharp percussive bite with prominent odd harmonics, Root C4 = 60, ~0.9s)
    {
        let len = (sample_rate as f32 * 0.9) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 4.5).exp();
            let click = if t < 0.004 { (1.0 - t / 0.004) * 0.4 } else { 0.0 };
            // Strong odd harmonics: 1, 3, 5, 7, with fast decaying upper harmonics
            let h3_env = (-t * 8.0).exp();
            let h5_env = (-t * 12.0).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.55
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.35 * (0.4 + 0.6 * h3_env)
                + (2.0 * pi * freq * 5.0 * t).sin() * 0.25 * h5_env
                + (2.0 * pi * freq * 7.0 * t).sin() * 0.15 * h5_env
                + click;
            let sample = (wave * env * 23000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Clavinet_C4", 60, None, data);
    }

    // Sample 3: Vibraphone (Pure resonant bell tone with 5.5 Hz tremolo, Root C4 = 60, ~1.4s)
    {
        let len = (sample_rate as f32 * 1.4) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.0).exp();
            let tremolo = 1.0 + 0.18 * (2.0 * pi * 5.5 * t).sin();
            let transient = (-t * 25.0).exp() * 0.3 * (2.0 * pi * freq * 4.0 * t).sin();
            let wave = ((2.0 * pi * freq * t).sin() * 0.85
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.15
                + transient) * tremolo;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Vibes_C4", 60, None, data);
    }

    // Sample 4: Organ (Looped tone with drawbar harmonics: 8', 4', 2 2/3', 2', Root C4 = 60, ~0.6s)
    {
        let len = (sample_rate as f32 * 0.6) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let wave = (2.0 * pi * freq * t).sin() * 0.5
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.3
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.2
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.15;
            let sample = (wave * 20000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        let cycle_samples = (sample_rate as f32 / freq).round() as u32;
        let loop_start = len as u32 - (cycle_samples * 10).min(len as u32 / 2);
        add_sample("Organ_C4", 60, Some(loop_start), data);
    }

    // Sample 5: Acoustic Guitar (Warm nylon/steel pluck, Root C3 = 48, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 130.81; // C3
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.0).exp();
            let body = (2.0 * pi * 210.0 * t).sin() * (-t * 15.0).exp() * 0.25;
            let wave = (2.0 * pi * freq * t).sin() * 0.55
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.32
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.22
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.14
                + (2.0 * pi * freq * 5.0 * t).sin() * 0.08
                + body;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("AcGuitar_C3", 48, None, data);
    }

    // Sample 6: Clean Electric Guitar (Plucked string, Root C3 = 48, ~1.0s)
    {
        let len = (sample_rate as f32 * 1.0) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 130.81; // C3
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.5).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.6
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.35
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.25
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.15;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("ElGuitar_C3", 48, None, data);
    }

    // Sample 7: String Ensemble (Lush detuned sawtooth strings with warm body, Root C4 = 60, ~1.5s, looped)
    {
        let len = (sample_rate as f32 * 1.5) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        let f_detune1 = freq * 1.004; // +7 cents
        let f_detune2 = freq * 0.996; // -7 cents
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            // Smooth attack curve (0.08s)
            let att = (t / 0.08).min(1.0);
            let w1 = (2.0 * pi * freq * t).sin() * 0.5 + (2.0 * pi * freq * 2.0 * t).sin() * 0.25 + (2.0 * pi * freq * 3.0 * t).sin() * 0.12;
            let w2 = (2.0 * pi * f_detune1 * t).sin() * 0.45 + (2.0 * pi * f_detune1 * 2.0 * t).sin() * 0.2;
            let w3 = (2.0 * pi * f_detune2 * t).sin() * 0.45 + (2.0 * pi * f_detune2 * 2.0 * t).sin() * 0.2;
            let wave = (w1 + w2 + w3) * 0.45 * att;
            let sample = (wave * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        let cycle_samples = (sample_rate as f32 / freq).round() as u32;
        let loop_start = len as u32 - (cycle_samples * 20).min(len as u32 / 2);
        add_sample("Strings_C4", 60, Some(loop_start), data);
    }

    // Sample 8: Brass Section (Punchy brass swell with rich harmonic buzz, Root C4 = 60, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.2).exp();
            // Brass bite filter swell over 35ms
            let swell = (t / 0.035).min(1.0);
            let buzz = (2.0 * pi * freq * t).sin() * 0.55
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.35
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.28
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.22
                + (2.0 * pi * freq * 5.0 * t).sin() * 0.15
                + (2.0 * pi * freq * 6.0 * t).sin() * 0.10;
            let wave = buzz * swell;
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Brass_C4", 60, None, data);
    }

    // Sample 9: Warm Synth Pad (Dual detuned analog pad with slow attack, Root C4 = 60, ~1.5s, looped)
    {
        let len = (sample_rate as f32 * 1.5) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        let f_detune = freq * 1.003;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let att = (t / 0.12).min(1.0);
            let lfo = 1.0 + 0.12 * (2.0 * pi * 0.5 * t).sin();
            let w1 = (2.0 * pi * freq * t).sin() * 0.6 + (2.0 * pi * freq * 2.0 * t).sin() * 0.25;
            let w2 = (2.0 * pi * f_detune * t).sin() * 0.55 + (2.0 * pi * f_detune * 2.0 * t).sin() * 0.2;
            let wave = (w1 + w2) * 0.5 * att * lfo;
            let sample = (wave * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        let cycle_samples = (sample_rate as f32 / freq).round() as u32;
        let loop_start = len as u32 - (cycle_samples * 20).min(len as u32 / 2);
        add_sample("SynthPad_C4", 60, Some(loop_start), data);
    }

    // Sample 10: Upright Acoustic Bass (Deep woody resonance, Root C2 = 36, ~1.3s)
    {
        let len = (sample_rate as f32 * 1.3) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41; // C2
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.6).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.75
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.35
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.12;
            let sample = (wave * env * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("UpBass_C2", 36, None, data);
    }

    // Sample 11: Electric Fingered Bass (Warm round electric bass, Root C2 = 36, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.8).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.70
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.40
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.18;
            let sample = (wave * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("ElBass_C2", 36, None, data);
    }

    // Sample 12: Picked Electric Bass (Sharp attack bite with aggressive midrange, Root C2 = 36, ~1.1s)
    {
        let len = (sample_rate as f32 * 1.1) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.2).exp();
            let pick_click = if t < 0.006 { (1.0 - t / 0.006) * 0.45 * (2.0 * pi * 1200.0 * t).sin() } else { 0.0 };
            let wave = (2.0 * pi * freq * t).sin() * 0.65
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.45
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.30
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.18
                + pick_click;
            let sample = (wave * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("PickBass_C2", 36, None, data);
    }

    // Sample 13: Fretless Bass (Singing "mwah" mid-frequency harmonic bloom, Root C2 = 36, ~1.4s)
    {
        let len = (sample_rate as f32 * 1.4) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.3).exp();
            // Harmonic 2 and 3 swell slightly after 50ms (the characteristic fretless bloom)
            let mwah = (t / 0.06).min(1.0) * (-t * 1.8).exp();
            let wave = (2.0 * pi * freq * t).sin() * 0.70
                + (2.0 * pi * freq * 2.0 * t).sin() * (0.25 + 0.35 * mwah)
                + (2.0 * pi * freq * 3.0 * t).sin() * (0.15 + 0.20 * mwah);
            let sample = (wave * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Fretless_C2", 36, None, data);
    }

    // Sample 14: Slap & Pop Bass (Crisp thumb slap impact transient and metallic ring, Root C2 = 36, ~0.9s)
    {
        let len = (sample_rate as f32 * 0.9) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 4.0).exp();
            let slap_snap = if t < 0.008 { (1.0 - t / 0.008) * 0.6 * (2.0 * pi * 2400.0 * t).sin() } else { 0.0 };
            let wave = (2.0 * pi * freq * t).sin() * 0.60
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.40
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.35 * (-t * 12.0).exp()
                + slap_snap;
            let sample = (wave * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("SlapBass_C2", 36, None, data);
    }

    // Sample 15: Synth Bass (Moog-style resonant 24dB low-pass sweep saw/square, Root C2 = 36, ~1.1s)
    {
        let len = (sample_rate as f32 * 1.1) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.0).exp();
            let filter_env = (-t * 9.0).exp();
            // Rich saw + sub square with decaying high harmonics
            let wave = (2.0 * pi * freq * t).sin() * 0.65
                + (2.0 * pi * freq * 2.0 * t).sin() * (0.2 + 0.4 * filter_env)
                + (2.0 * pi * freq * 3.0 * t).sin() * (0.1 + 0.3 * filter_env)
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.25 * filter_env;
            let sample = (wave * env * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("SynthBass_C2", 36, None, data);
    }

    // Sample 16: 808 Sub Bass (Massive clean sine sub drop, Root C1 = 24, ~1.6s)
    {
        let len = (sample_rate as f32 * 1.6) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 32.70; // C1
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 1.6).exp();
            // Subtle pitch sweep at start: 55 Hz -> 32.7 Hz
            let cur_freq = freq + 22.0 * (-t * 25.0).exp();
            phase += 2.0 * pi * cur_freq / sample_rate as f32;
            // Warm sub with subtle 2nd harmonic saturation
            let wave = phase.sin() * 0.90 + (2.0 * phase).sin() * 0.15;
            let sample = (wave * env * 28000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("SubBass_C1", 24, None, data);
    }

    // Sample 17: Acoustic Tuba / Low Brass (Warm brassy low-end chug with lip buzz, Root C2 = 36, ~1.1s)
    {
        let len = (sample_rate as f32 * 1.1) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.8).exp();
            let buzz = (2.0 * pi * freq * t).sin() * 0.65
                + (2.0 * pi * freq * 2.0 * t).sin() * 0.30
                + (2.0 * pi * freq * 3.0 * t).sin() * 0.25
                + (2.0 * pi * freq * 4.0 * t).sin() * 0.12;
            let sample = (buzz * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Tuba_C2", 36, None, data);
    }

    // --- Drum Samples (18..36) ---

    // Sample 18: Standard Kick Drum (Pitch sweep 150 -> 45 Hz, ~0.4s)
    {
        let len = (sample_rate as f32 * 0.4) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 9.0).exp();
            let freq = 45.0 + 105.0 * (-t * 30.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let click = if t < 0.005 { (1.0 - t / 0.005) * 0.5 } else { 0.0 };
            let wave = phase.sin() * 0.9 + click;
            let sample = (wave * env * 28000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Kick", 36, None, data);
    }

    // Sample 19: Standard Snare Drum (Body tone 185 Hz + noise snare wires, ~0.35s)
    {
        let len = (sample_rate as f32 * 0.35) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 12345;
        let freq = 185.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * (-t * 14.0).exp();
            let tone = (2.0 * pi * freq * t).sin() * (-t * 22.0).exp();
            let wave = tone * 0.5 + noise * 0.7;
            let sample = (wave * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Snare", 38, None, data);
    }

    // Sample 20: Standard Hi-Hat (High-passed metallic noise, ~0.08s)
    {
        let len = (sample_rate as f32 * 0.08) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 98765;
        let mut last_val: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let raw_noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            let hp = raw_noise - last_val;
            last_val = raw_noise * 0.5;
            let env = (-t * 60.0).exp();
            let sample = (hp * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_HiHat", 42, None, data);
    }

    // Sample 21: Standard Ride Cymbal (Metallic shimmering ringing decay, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 45678;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * 0.4;
            let f1 = (2.0 * pi * 587.0 * t).sin();
            let f2 = (2.0 * pi * 845.0 * t).sin();
            let f3 = (2.0 * pi * 1320.0 * t).sin();
            let wave = (f1 + f2 + f3 + noise) * 0.25;
            let env = (-t * 3.0).exp();
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Ride", 51, None, data);
    }

    // Sample 22: Standard Tom (Pitched decay 110 -> 85 Hz, ~0.45s)
    {
        let len = (sample_rate as f32 * 0.45) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 7.0).exp();
            let freq = 85.0 + 35.0 * (-t * 20.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let sample = (phase.sin() * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Tom", 45, None, data);
    }

    // Sample 23: Standard Rimshot (Short woody click, ~0.06s)
    {
        let len = (sample_rate as f32 * 0.06) as usize;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 90.0).exp();
            let wave = (2.0 * pi * 800.0 * t).sin();
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Rim", 37, None, data);
    }

    // Sample 24: TR-808 Kick (Boomy resonant 808 sub kick, ~0.75s)
    {
        let len = (sample_rate as f32 * 0.75) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 4.2).exp();
            let freq = 42.0 + 80.0 * (-t * 35.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let click = if t < 0.003 { (1.0 - t / 0.003) * 0.6 } else { 0.0 };
            let wave = phase.sin() * 0.95 + click;
            let sample = (wave * env * 28000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_808_Kick", 36, None, data);
    }

    // Sample 25: TR-808 Snare (Dual-tone 180Hz/330Hz + snappy noise burst, ~0.25s)
    {
        let len = (sample_rate as f32 * 0.25) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 24680;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * (-t * 22.0).exp();
            let t1 = (2.0 * pi * 180.0 * t).sin() * (-t * 28.0).exp();
            let t2 = (2.0 * pi * 330.0 * t).sin() * (-t * 32.0).exp();
            let wave = (t1 + t2) * 0.4 + noise * 0.65;
            let sample = (wave * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_808_Snare", 38, None, data);
    }

    // Sample 26: TR-808 Hi-Hat (Crisp synthetic square-mix metallic tick, ~0.05s)
    {
        let len = (sample_rate as f32 * 0.05) as usize;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 95.0).exp();
            let f1 = (2.0 * pi * 3500.0 * t).sin();
            let f2 = (2.0 * pi * 5200.0 * t).sin();
            let f3 = (2.0 * pi * 7800.0 * t).sin();
            let wave = (f1 + f2 + f3) * 0.33;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_808_Hat", 42, None, data);
    }

    // Sample 27: TR-808 Tom (Resonant sine pitch drop, ~0.35s)
    {
        let len = (sample_rate as f32 * 0.35) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 8.5).exp();
            let freq = 75.0 + 55.0 * (-t * 30.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let sample = (phase.sin() * env * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_808_Tom", 45, None, data);
    }

    // Sample 28: Jazz Brush Kick (Soft felt beater low thud, ~0.28s)
    {
        let len = (sample_rate as f32 * 0.28) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 11.0).exp();
            let freq = 50.0 + 35.0 * (-t * 20.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let sample = (phase.sin() * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Brush_Kick", 36, None, data);
    }

    // Sample 29: Jazz Brush Snare (Soft brush sweep stir on coated head, ~0.38s)
    {
        let len = (sample_rate as f32 * 0.38) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 54321;
        let mut filter_mem: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let raw_noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            // Soft lowpass filter for gentle brush texture
            filter_mem = filter_mem * 0.65 + raw_noise * 0.35;
            let env = (-t * 8.0).exp() * (1.0 + 0.3 * (2.0 * pi * 15.0 * t).sin());
            let tap = if t < 0.008 { (1.0 - t / 0.008) * 0.35 } else { 0.0 };
            let wave = filter_mem * 0.75 + tap;
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Brush_Snare", 38, None, data);
    }

    // Sample 30: Jazz Brush Ride (Airy delicate sizzle cymbal, ~1.1s)
    {
        let len = (sample_rate as f32 * 1.1) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 13579;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * 0.35;
            let f1 = (2.0 * pi * 480.0 * t).sin() * 0.3;
            let f2 = (2.0 * pi * 720.0 * t).sin() * 0.3;
            let wave = (f1 + f2 + noise) * 0.35;
            let env = (-t * 2.5).exp();
            let sample = (wave * env * 20000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Brush_Ride", 51, None, data);
    }

    // Sample 31: Vintage Funk Kick (Tight, deadened, punchy 70s kick, ~0.22s)
    {
        let len = (sample_rate as f32 * 0.22) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 16.0).exp();
            let freq = 55.0 + 85.0 * (-t * 40.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let click = if t < 0.004 { (1.0 - t / 0.004) * 0.6 } else { 0.0 };
            let wave = phase.sin() * 0.9 + click;
            let sample = (wave * env * 28000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Funk_Kick", 36, None, data);
    }

    // Sample 32: Vintage Funk Snare (Damped with tape, dry crack with immediate choke, ~0.18s)
    {
        let len = (sample_rate as f32 * 0.18) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 33333;
        let freq = 210.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * (-t * 28.0).exp();
            let tone = (2.0 * pi * freq * t).sin() * (-t * 30.0).exp();
            let wave = tone * 0.55 + noise * 0.65;
            let sample = (wave * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Funk_Snare", 38, None, data);
    }

    // Sample 33: Rock Arena Kick (Gated heavy kick with sub punch, ~0.45s)
    {
        let len = (sample_rate as f32 * 0.45) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 7.5).exp();
            let freq = 42.0 + 130.0 * (-t * 32.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let click = if t < 0.005 { (1.0 - t / 0.005) * 0.7 } else { 0.0 };
            let wave = phase.sin() * 0.95 + click;
            let sample = (wave * env * 29000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Rock_Kick", 36, None, data);
    }

    // Sample 34: Rock Arena Snare (High-energy crack with explosive reverberant body, ~0.45s)
    {
        let len = (sample_rate as f32 * 0.45) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 77777;
        let freq = 195.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * (-t * 10.0).exp();
            let tone = (2.0 * pi * freq * t).sin() * (-t * 18.0).exp();
            let rim_crack = (2.0 * pi * 850.0 * t).sin() * (-t * 40.0).exp() * 0.4;
            let wave = tone * 0.5 + noise * 0.75 + rim_crack;
            let sample = (wave * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Rock_Snare", 38, None, data);
    }

    // Sample 35: Lo-Fi Hip Hop Kick (Warm, filtered, pitch-dropped vintage kick, ~0.35s)
    {
        let len = (sample_rate as f32 * 0.35) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 9.5).exp();
            let freq = 40.0 + 65.0 * (-t * 22.0).exp();
            phase += 2.0 * pi * freq / sample_rate as f32;
            let wave = phase.sin() * 0.95;
            let sample = (wave * env * 27000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_LoFi_Kick", 36, None, data);
    }

    // Sample 36: Lo-Fi Hip Hop Snare (Filtered vinyl dusty snare pop, ~0.24s)
    {
        let len = (sample_rate as f32 * 0.24) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 88888;
        let freq = 170.0;
        let mut filter_mem: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let raw_noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            // Warm vintage lowpass
            filter_mem = filter_mem * 0.55 + raw_noise * 0.45;
            let noise = filter_mem * (-t * 18.0).exp();
            let tone = (2.0 * pi * freq * t).sin() * (-t * 24.0).exp();
            let wave = tone * 0.6 + noise * 0.6;
            let sample = (wave * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_LoFi_Snare", 38, None, data);
    }

    // --- Dedicated Percussion Samples (37..40) ---

    // Sample 37: Tambourine (Metallic jingle cluster + woody frame tap, GM 54, ~0.25s)
    {
        let len = (sample_rate as f32 * 0.25) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 99999;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * 0.3;
            // High metallic jingle cluster
            let j1 = (2.0 * pi * 4400.0 * t).sin();
            let j2 = (2.0 * pi * 6800.0 * t).sin();
            let j3 = (2.0 * pi * 9500.0 * t).sin();
            let tap = if t < 0.005 { (1.0 - t / 0.005) * 0.4 * (2.0 * pi * 600.0 * t).sin() } else { 0.0 };
            let env = (-t * 18.0).exp();
            let wave = (j1 + j2 + j3 + noise) * 0.25 * env + tap;
            let sample = (wave * 23000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Perc_Tambourine", 54, None, data);
    }

    // Sample 38: Cowbell (Dual-mode metallic tone 587Hz + 845Hz, GM 56, ~0.22s)
    {
        let len = (sample_rate as f32 * 0.22) as usize;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 20.0).exp();
            let f1 = (2.0 * pi * 587.0 * t).sin();
            let f2 = (2.0 * pi * 845.0 * t).sin();
            // Subtle square distortion characteristic of cowbell struck metal
            let mix = (f1 * 0.55 + f2 * 0.45).clamp(-0.8, 0.8) * 1.25;
            let sample = (mix * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Perc_Cowbell", 56, None, data);
    }

    // Sample 39: Hand Clap (Three micro-transient claps + cohesive room tail, GM 39, ~0.22s)
    {
        let len = (sample_rate as f32 * 0.22) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 44444;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            // 3 micro-bursts at t=0, t=0.010, t=0.022s
            let b1 = if t < 0.010 { (-t * 300.0).exp() } else { 0.0 };
            let b2 = if (0.010..0.022).contains(&t) { (-(t - 0.010) * 300.0).exp() } else { 0.0 };
            let b3 = if t >= 0.022 { (-(t - 0.022) * 22.0).exp() } else { 0.0 };
            let clap_env = b1 * 0.7 + b2 * 0.85 + b3 * 1.0;
            let sample = (noise * clap_env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Perc_HandClap", 39, None, data);
    }

    // Sample 40: Shaker (Textured friction noise envelope, GM 70, ~0.12s)
    {
        let len = (sample_rate as f32 * 0.12) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 66666;
        let mut hp_last = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let raw_noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            // High-pass filter for grainy shaker rattle
            let hp = raw_noise - hp_last * 0.4;
            hp_last = raw_noise;
            // Smooth attack into sharp decay
            let attack = (t / 0.015).min(1.0);
            let decay = (-t * 40.0).exp();
            let wave = hp * attack * decay;
            let sample = (wave * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Perc_Shaker", 70, None, data);
    }

    // Build RIFF SFBK structures
    build_riff_soundfont(&metas, &pcm_samples, sample_rate, &mut sf2);
    sf2
}

fn write_chunk<F>(out: &mut Vec<u8>, fourcc: &[u8; 4], write_content: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    out.extend_from_slice(fourcc);
    let size_pos = out.len();
    out.write_u32::<LittleEndian>(0).unwrap();
    let start_pos = out.len();
    write_content(out);
    let end_pos = out.len();
    let size = (end_pos - start_pos) as u32;
    (&mut out[size_pos..size_pos + 4]).write_u32::<LittleEndian>(size).unwrap();
    if size % 2 != 0 {
        out.push(0);
    }
}

fn write_list<F>(out: &mut Vec<u8>, list_type: &[u8; 4], write_subchunks: F)
where
    F: FnOnce(&mut Vec<u8>),
{
    write_chunk(out, b"LIST", |w| {
        w.extend_from_slice(list_type);
        write_subchunks(w);
    });
}

fn write_fixed_str(out: &mut Vec<u8>, s: &str, len: usize) {
    let bytes = s.as_bytes();
    for i in 0..len {
        if i < bytes.len() {
            out.push(bytes[i]);
        } else {
            out.push(0);
        }
    }
}

fn build_riff_soundfont(metas: &[SampleMeta], pcm_samples: &[i16], sample_rate: u32, out: &mut Vec<u8>) {
    write_chunk(out, b"RIFF", |riff| {
        riff.extend_from_slice(b"sfbk");

        // 1. INFO list
        write_list(riff, b"INFO", |info| {
            write_chunk(info, b"ifil", |w| {
                w.write_u16::<LittleEndian>(2).unwrap(); // major 2
                w.write_u16::<LittleEndian>(4).unwrap(); // minor 4
            });
            write_chunk(info, b"isng", |w| {
                w.write_all(b"EMU8000\0").unwrap();
            });
            write_chunk(info, b"INAM", |w| {
                w.write_all(b"Time Circle SoundFont\0").unwrap();
            });
        });

        // 2. sdta list (sample data)
        write_list(riff, b"sdta", |sdta| {
            write_chunk(sdta, b"smpl", |w| {
                for &sample in pcm_samples {
                    w.write_i16::<LittleEndian>(sample).unwrap();
                }
            });
        });

        // 3. pdta list (preset / instrument / sample headers)
        write_list(riff, b"pdta", |pdta| {
            build_pdta_chunk(pdta, metas, sample_rate);
        });
    });
}

// SF2 Generator enumerations
const GEN_KEY_RANGE: u16 = 43;
const GEN_OVERRIDING_ROOT_KEY: u16 = 58;
const GEN_INSTRUMENT: u16 = 41;
const GEN_SAMPLE_ID: u16 = 53;

fn build_pdta_chunk(pdta: &mut Vec<u8>, metas: &[SampleMeta], sample_rate: u32) {
    struct InstZoneDef {
        low_key: u8,
        high_key: u8,
        sample_idx: u16,
        root_key: Option<u8>,
    }

    let mut inst_defs: Vec<Vec<InstZoneDef>> = Vec::new();

    // Helper for 1-zone melodic instrument covering all 128 keys
    let mut add_melodic_inst = |sample_idx: u16| {
        inst_defs.push(vec![InstZoneDef {
            low_key: 0,
            high_key: 127,
            sample_idx,
            root_key: None,
        }]);
    };

    // Melodic Instruments (0..17)
    // 0: Acoustic Grand Piano -> Sample 0
    add_melodic_inst(0);
    // 1: Electric Piano -> Sample 1
    add_melodic_inst(1);
    // 2: Clavinet -> Sample 2
    add_melodic_inst(2);
    // 3: Vibraphone -> Sample 3
    add_melodic_inst(3);
    // 4: Drawbar Organ -> Sample 4
    add_melodic_inst(4);
    // 5: Acoustic Guitar -> Sample 5
    add_melodic_inst(5);
    // 6: Electric Guitar -> Sample 6
    add_melodic_inst(6);
    // 7: Upright Bass -> Sample 10
    add_melodic_inst(10);
    // 8: Electric Bass -> Sample 11
    add_melodic_inst(11);
    // 9: Picked Bass -> Sample 12
    add_melodic_inst(12);
    // 10: Fretless Bass -> Sample 13
    add_melodic_inst(13);
    // 11: Slap Bass -> Sample 14
    add_melodic_inst(14);
    // 12: Synth Bass -> Sample 15
    add_melodic_inst(15);
    // 13: 808 Sub Bass -> Sample 16
    add_melodic_inst(16);
    // 14: String Ensemble -> Sample 7
    add_melodic_inst(7);
    // 15: Acoustic Tuba -> Sample 17
    add_melodic_inst(17);
    // 16: Brass Section -> Sample 8
    add_melodic_inst(8);
    // 17: Warm Synth Pad -> Sample 9
    add_melodic_inst(9);

    // Shared percussion zones for all drum kits:
    // Sample 37: Tambourine (key 54)
    // Sample 38: Cowbell (key 56)
    // Sample 39: HandClap (key 39)
    // Sample 40: Shaker (keys 69..70, 82)
    let make_shared_perc_zones = || {
        vec![
            InstZoneDef { low_key: 39, high_key: 39, sample_idx: 39, root_key: Some(39) }, // HandClap
            InstZoneDef { low_key: 54, high_key: 54, sample_idx: 37, root_key: Some(54) }, // Tambourine
            InstZoneDef { low_key: 56, high_key: 56, sample_idx: 38, root_key: Some(56) }, // Cowbell
            InstZoneDef { low_key: 69, high_key: 70, sample_idx: 40, root_key: Some(70) }, // Shaker
            InstZoneDef { low_key: 82, high_key: 82, sample_idx: 40, root_key: Some(70) }, // Shaker alt
        ]
    };

    // Helper to build a drum kit instrument
    let mut add_drum_kit = |kick_idx: u16, snare_idx: u16, hat_idx: u16, tom_idx: u16, ride_idx: u16, rim_idx: u16| {
        let mut zones = vec![
            InstZoneDef { low_key: 35, high_key: 36, sample_idx: kick_idx, root_key: Some(36) },
            InstZoneDef { low_key: 37, high_key: 37, sample_idx: rim_idx, root_key: Some(37) },
            InstZoneDef { low_key: 38, high_key: 38, sample_idx: snare_idx, root_key: Some(38) },
            InstZoneDef { low_key: 40, high_key: 40, sample_idx: snare_idx, root_key: Some(38) },
            InstZoneDef { low_key: 42, high_key: 42, sample_idx: hat_idx, root_key: Some(42) },
            InstZoneDef { low_key: 44, high_key: 44, sample_idx: hat_idx, root_key: Some(42) },
            InstZoneDef { low_key: 46, high_key: 46, sample_idx: hat_idx, root_key: Some(42) },
            InstZoneDef { low_key: 41, high_key: 41, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 43, high_key: 43, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 45, high_key: 45, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 47, high_key: 47, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 48, high_key: 48, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 50, high_key: 50, sample_idx: tom_idx, root_key: Some(45) },
            InstZoneDef { low_key: 51, high_key: 53, sample_idx: ride_idx, root_key: Some(51) },
            InstZoneDef { low_key: 55, high_key: 55, sample_idx: ride_idx, root_key: Some(51) }, // Splash
            InstZoneDef { low_key: 57, high_key: 68, sample_idx: ride_idx, root_key: Some(51) }, // Cymbals
            InstZoneDef { low_key: 71, high_key: 81, sample_idx: ride_idx, root_key: Some(51) }, // High perc/cymbals
            InstZoneDef { low_key: 83, high_key: 127, sample_idx: ride_idx, root_key: Some(51) },
        ];
        zones.extend(make_shared_perc_zones());
        inst_defs.push(zones);
    };

    // Drum Kits:
    // Inst 18: Standard Drum Kit
    add_drum_kit(18, 19, 20, 22, 21, 23);
    // Inst 19: Rock Power Kit
    add_drum_kit(33, 34, 20, 22, 21, 23);
    // Inst 20: Vintage Funk Kit
    add_drum_kit(31, 32, 20, 22, 21, 23);
    // Inst 21: TR-808 Kit
    add_drum_kit(24, 25, 26, 27, 21, 23);
    // Inst 22: Lo-Fi Hip Hop Kit
    add_drum_kit(35, 36, 26, 27, 21, 23);
    // Inst 23: Jazz Brush Kit
    add_drum_kit(28, 29, 20, 22, 30, 23);

    // Build igen (Instrument Generators)
    let mut igen_bytes = Vec::new();
    let mut ibag_bytes = Vec::new();
    let mut current_igen_idx: u16 = 0;
    let mut current_ibag_idx: u16 = 0;
    let mut inst_bag_indices = Vec::new();

    for zones in &inst_defs {
        inst_bag_indices.push(current_ibag_idx);
        for z in zones {
            ibag_bytes.write_u16::<LittleEndian>(current_igen_idx).unwrap(); // wGenNdx
            ibag_bytes.write_u16::<LittleEndian>(0).unwrap(); // wModNdx
            current_ibag_idx += 1;

            // Generator: keyRange (low, high)
            igen_bytes.write_u16::<LittleEndian>(GEN_KEY_RANGE).unwrap();
            igen_bytes.write_u8(z.low_key).unwrap();
            igen_bytes.write_u8(z.high_key).unwrap();
            current_igen_idx += 1;

            if let Some(rk) = z.root_key {
                igen_bytes.write_u16::<LittleEndian>(GEN_OVERRIDING_ROOT_KEY).unwrap();
                igen_bytes.write_u16::<LittleEndian>(rk as u16).unwrap();
                current_igen_idx += 1;
            }

            // Generator: sampleID
            igen_bytes.write_u16::<LittleEndian>(GEN_SAMPLE_ID).unwrap();
            igen_bytes.write_u16::<LittleEndian>(z.sample_idx).unwrap();
            current_igen_idx += 1;
        }
    }
    // Terminal ibag & igen entries
    ibag_bytes.write_u16::<LittleEndian>(current_igen_idx).unwrap();
    ibag_bytes.write_u16::<LittleEndian>(0).unwrap();
    igen_bytes.write_u16::<LittleEndian>(0).unwrap();
    igen_bytes.write_u16::<LittleEndian>(0).unwrap();

    // Build inst chunk
    let mut inst_bytes = Vec::new();
    let inst_names = [
        "Acoustic Grand Piano",
        "Electric Piano",
        "Clavinet D6",
        "Vibraphone",
        "Drawbar Organ",
        "Acoustic Guitar",
        "Electric Guitar",
        "Acoustic Upright Bass",
        "Electric Fingered Bass",
        "Picked Electric Bass",
        "Fretless Bass",
        "Slap Bass",
        "Synth Bass",
        "808 Sub Bass",
        "String Ensemble",
        "Acoustic Tuba",
        "Brass Section",
        "Warm Synth Pad",
        "Standard Drum Kit",
        "Rock Power Kit",
        "Vintage Funk Kit",
        "TR-808 Kit",
        "Lo-Fi Hip Hop Kit",
        "Jazz Brush Kit",
    ];
    for (i, name) in inst_names.iter().enumerate() {
        write_fixed_str(&mut inst_bytes, name, 20);
        inst_bytes.write_u16::<LittleEndian>(inst_bag_indices[i]).unwrap();
    }
    // Terminal inst
    write_fixed_str(&mut inst_bytes, "EOI", 20);
    inst_bytes.write_u16::<LittleEndian>(current_ibag_idx).unwrap();

    // --- Presets & Zones ---
    struct PresetDef {
        name: &'static str,
        patch: u16,
        bank: u16,
        inst_idx: u16,
    }

    let presets = [
        // Bank 0 Melodic Presets
        PresetDef { name: "Acoustic Grand", patch: 0, bank: 0, inst_idx: 0 },
        PresetDef { name: "Electric Piano", patch: 4, bank: 0, inst_idx: 1 },
        PresetDef { name: "Clavinet", patch: 7, bank: 0, inst_idx: 2 },
        PresetDef { name: "Vibraphone", patch: 11, bank: 0, inst_idx: 3 },
        PresetDef { name: "Drawbar Organ", patch: 16, bank: 0, inst_idx: 4 },
        PresetDef { name: "Acoustic Guitar", patch: 24, bank: 0, inst_idx: 5 },
        PresetDef { name: "Electric Guitar", patch: 27, bank: 0, inst_idx: 6 },
        PresetDef { name: "Upright Bass", patch: 32, bank: 0, inst_idx: 7 },
        PresetDef { name: "Electric Bass", patch: 33, bank: 0, inst_idx: 8 },
        PresetDef { name: "Picked Bass", patch: 34, bank: 0, inst_idx: 9 },
        PresetDef { name: "Fretless Bass", patch: 35, bank: 0, inst_idx: 10 },
        PresetDef { name: "Slap Bass", patch: 36, bank: 0, inst_idx: 11 },
        PresetDef { name: "Synth Bass", patch: 38, bank: 0, inst_idx: 12 },
        PresetDef { name: "808 Sub Bass", patch: 39, bank: 0, inst_idx: 13 },
        PresetDef { name: "String Ensemble", patch: 48, bank: 0, inst_idx: 14 },
        PresetDef { name: "Acoustic Tuba", patch: 58, bank: 0, inst_idx: 15 },
        PresetDef { name: "Brass Section", patch: 61, bank: 0, inst_idx: 16 },
        PresetDef { name: "Warm Synth Pad", patch: 89, bank: 0, inst_idx: 17 },

        // Bank 128 Drum Kits
        PresetDef { name: "Standard Drum Kit", patch: 0, bank: 128, inst_idx: 18 },
        PresetDef { name: "Rock Power Kit", patch: 16, bank: 128, inst_idx: 19 },
        PresetDef { name: "Vintage Funk Kit", patch: 24, bank: 128, inst_idx: 20 },
        PresetDef { name: "TR-808 Kit", patch: 25, bank: 128, inst_idx: 21 },
        PresetDef { name: "Lo-Fi Hip Hop Kit", patch: 32, bank: 128, inst_idx: 22 },
        PresetDef { name: "Jazz Brush Kit", patch: 40, bank: 128, inst_idx: 23 },
    ];

    let mut pgen_bytes = Vec::new();
    let mut pbag_bytes = Vec::new();
    let mut phdr_bytes = Vec::new();

    let mut current_pgen_idx: u16 = 0;
    let mut current_pbag_idx: u16 = 0;

    for p in &presets {
        // Preset header
        write_fixed_str(&mut phdr_bytes, p.name, 20);
        phdr_bytes.write_u16::<LittleEndian>(p.patch).unwrap();
        phdr_bytes.write_u16::<LittleEndian>(p.bank).unwrap();
        phdr_bytes.write_u16::<LittleEndian>(current_pbag_idx).unwrap();
        phdr_bytes.write_u32::<LittleEndian>(0).unwrap(); // library
        phdr_bytes.write_u32::<LittleEndian>(0).unwrap(); // genre
        phdr_bytes.write_u32::<LittleEndian>(0).unwrap(); // morphology

        // 1 zone per preset
        pbag_bytes.write_u16::<LittleEndian>(current_pgen_idx).unwrap();
        pbag_bytes.write_u16::<LittleEndian>(0).unwrap();
        current_pbag_idx += 1;

        // Generator: instrument
        pgen_bytes.write_u16::<LittleEndian>(GEN_INSTRUMENT).unwrap();
        pgen_bytes.write_u16::<LittleEndian>(p.inst_idx).unwrap();
        current_pgen_idx += 1;
    }

    // Terminal Preset Header
    write_fixed_str(&mut phdr_bytes, "EOP", 20);
    phdr_bytes.write_u16::<LittleEndian>(255).unwrap();
    phdr_bytes.write_u16::<LittleEndian>(255).unwrap();
    phdr_bytes.write_u16::<LittleEndian>(current_pbag_idx).unwrap();
    phdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    phdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    phdr_bytes.write_u32::<LittleEndian>(0).unwrap();

    // Terminal pbag
    pbag_bytes.write_u16::<LittleEndian>(current_pgen_idx).unwrap();
    pbag_bytes.write_u16::<LittleEndian>(0).unwrap();

    // Terminal pgen
    pgen_bytes.write_u16::<LittleEndian>(0).unwrap();
    pgen_bytes.write_u16::<LittleEndian>(0).unwrap();

    // --- shdr (Sample Headers) ---
    let mut shdr_bytes = Vec::new();
    for m in metas {
        write_fixed_str(&mut shdr_bytes, m.name, 20);
        shdr_bytes.write_u32::<LittleEndian>(m.start).unwrap();
        shdr_bytes.write_u32::<LittleEndian>(m.end).unwrap();
        shdr_bytes.write_u32::<LittleEndian>(m.loop_start).unwrap();
        shdr_bytes.write_u32::<LittleEndian>(m.loop_end).unwrap();
        shdr_bytes.write_u32::<LittleEndian>(sample_rate).unwrap();
        shdr_bytes.write_u8(m.root_key).unwrap();
        shdr_bytes.write_i8(0).unwrap(); // pitch correction
        shdr_bytes.write_u16::<LittleEndian>(0).unwrap(); // sample link
        shdr_bytes.write_u16::<LittleEndian>(1).unwrap(); // monoSample = 1
    }
    // Terminal shdr
    write_fixed_str(&mut shdr_bytes, "EOS", 20);
    shdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u32::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u8(0).unwrap();
    shdr_bytes.write_i8(0).unwrap();
    shdr_bytes.write_u16::<LittleEndian>(0).unwrap();
    shdr_bytes.write_u16::<LittleEndian>(0).unwrap();

    // Write all pdta subchunks in standard order
    write_chunk(pdta, b"phdr", |w| w.extend_from_slice(&phdr_bytes));
    write_chunk(pdta, b"pbag", |w| w.extend_from_slice(&pbag_bytes));
    write_chunk(pdta, b"pmod", |_w| {}); // empty pmod
    write_chunk(pdta, b"pgen", |w| w.extend_from_slice(&pgen_bytes));
    write_chunk(pdta, b"inst", |w| w.extend_from_slice(&inst_bytes));
    write_chunk(pdta, b"ibag", |w| w.extend_from_slice(&ibag_bytes));
    write_chunk(pdta, b"imod", |_w| {}); // empty imod
    write_chunk(pdta, b"igen", |w| w.extend_from_slice(&igen_bytes));
    write_chunk(pdta, b"shdr", |w| w.extend_from_slice(&shdr_bytes));
}
