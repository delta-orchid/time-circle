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
/// Melodic presets:
/// - 0: Acoustic Grand Piano
/// - 4: Electric Piano (Rhodes style)
/// - 16: Drawbar Organ
/// - 27: Electric Guitar (Clean)
/// - 32: Acoustic Upright Bass
/// - 33: Electric Fingered Bass
/// Drum kit:
/// - Bank 128, Preset 0: GM Drums (Kick, Snare, Hi-hat, Ride, Tom, Rimshot)
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

    // Sample 0: Piano (Multi-harmonic decaying tone, Root C4 = 60, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63; // C4
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.2).exp();
            let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.6
                + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.3
                + (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.15
                + (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin() * 0.08;
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
            let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.7
                + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.25
                + (2.0 * std::f32::consts::PI * freq * 7.0 * t).sin() * 0.3 * bell_env;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("EPiano_C4", 60, None, data);
    }

    // Sample 2: Organ (Looped tone with drawbar harmonics: 8', 4', 2 2/3', 2', Root C4 = 60, ~0.6s)
    {
        let len = (sample_rate as f32 * 0.6) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 261.63;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.5
                + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.3
                + (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.2
                + (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin() * 0.15;
            let sample = (wave * 20000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        let cycle_samples = (sample_rate as f32 / freq).round() as u32;
        let loop_start = len as u32 - (cycle_samples * 10).min(len as u32 / 2);
        add_sample("Organ_C4", 60, Some(loop_start), data);
    }

    // Sample 3: Guitar (Plucked string with fast attack & moderate decay, Root C3 = 48, ~1.0s)
    {
        let len = (sample_rate as f32 * 1.0) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 130.81; // C3
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 3.5).exp();
            let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.6
                + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.35
                + (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.25
                + (2.0 * std::f32::consts::PI * freq * 4.0 * t).sin() * 0.15;
            let sample = (wave * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Guitar_C3", 48, None, data);
    }

    // Sample 4: Bass (Punchy low root with sub and 2nd harmonic, Root C2 = 36, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let freq = 65.41; // C2
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 2.8).exp();
            let wave = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.75
                + (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.35
                + (2.0 * std::f32::consts::PI * freq * 3.0 * t).sin() * 0.12;
            let sample = (wave * env * 26000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Bass_C2", 36, None, data);
    }

    // Drum Sample 5: Kick Drum (Pitch sweep 150 Hz -> 45 Hz + punch, Root 36, ~0.4s)
    {
        let len = (sample_rate as f32 * 0.4) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 9.0).exp();
            let freq = 45.0 + 105.0 * (-t * 30.0).exp();
            phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
            let click = if t < 0.005 { (1.0 - t / 0.005) * 0.5 } else { 0.0 };
            let wave = phase.sin() * 0.9 + click;
            let sample = (wave * env * 28000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Kick", 36, None, data);
    }

    // Drum Sample 6: Snare Drum (Body tone 185 Hz + noise snare wires, Root 38, ~0.35s)
    {
        let len = (sample_rate as f32 * 0.35) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 12345;
        let freq = 185.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * (-t * 14.0).exp();
            let tone = (2.0 * std::f32::consts::PI * freq * t).sin() * (-t * 22.0).exp();
            let wave = tone * 0.5 + noise * 0.7;
            let sample = (wave * 25000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Snare", 38, None, data);
    }

    // Drum Sample 7: Closed Hi-Hat (High-passed metallic noise, Root 42, ~0.08s)
    {
        let len = (sample_rate as f32 * 0.08) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 98765;
        let mut last_val: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let raw_noise = (seed as f32 / 4294967295.0) * 2.0 - 1.0;
            // High pass filter
            let hp = raw_noise - last_val;
            last_val = raw_noise * 0.5;
            let env = (-t * 60.0).exp();
            let sample = (hp * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_HiHat", 42, None, data);
    }

    // Drum Sample 8: Ride Cymbal (Metallic shimmering ringing decay, Root 51, ~1.2s)
    {
        let len = (sample_rate as f32 * 1.2) as usize;
        let mut data = Vec::with_capacity(len);
        let mut seed: u32 = 45678;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let noise = ((seed as f32 / 4294967295.0) * 2.0 - 1.0) * 0.4;
            let f1 = (2.0 * std::f32::consts::PI * 587.0 * t).sin();
            let f2 = (2.0 * std::f32::consts::PI * 845.0 * t).sin();
            let f3 = (2.0 * std::f32::consts::PI * 1320.0 * t).sin();
            let wave = (f1 + f2 + f3 + noise) * 0.25;
            let env = (-t * 3.0).exp();
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Ride", 51, None, data);
    }

    // Drum Sample 9: Tom (Pitched decay 110 Hz -> 85 Hz, Root 45, ~0.45s)
    {
        let len = (sample_rate as f32 * 0.45) as usize;
        let mut data = Vec::with_capacity(len);
        let mut phase: f32 = 0.0;
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 7.0).exp();
            let freq = 85.0 + 35.0 * (-t * 20.0).exp();
            phase += 2.0 * std::f32::consts::PI * freq / sample_rate as f32;
            let sample = (phase.sin() * env * 24000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Tom", 45, None, data);
    }

    // Drum Sample 10: Rimshot (Short woody click, Root 37, ~0.06s)
    {
        let len = (sample_rate as f32 * 0.06) as usize;
        let mut data = Vec::with_capacity(len);
        for i in 0..len {
            let t = i as f32 / sample_rate as f32;
            let env = (-t * 90.0).exp();
            let wave = (2.0 * std::f32::consts::PI * 800.0 * t).sin();
            let sample = (wave * env * 22000.0).clamp(-32767.0, 32767.0) as i16;
            data.push(sample);
        }
        add_sample("Drum_Rim", 37, None, data);
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
    // RIFF chunks are padded to 2-byte boundary
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
    // We define:
    // Instruments:
    // 0: Grand Piano (Sample 0)
    // 1: EPiano (Sample 1)
    // 2: Organ (Sample 2)
    // 3: Guitar (Sample 3)
    // 4: Bass Upright (Sample 4)
    // 5: Bass Electric (Sample 4)
    // 6: Drum Kit (Samples 5..10 mapped to specific drum keys)
    // Total 7 instruments.

    // Presets:
    // Preset 0, Bank 0: Grand Piano -> Inst 0
    // Preset 4, Bank 0: EPiano -> Inst 1
    // Preset 16, Bank 0: Organ -> Inst 2
    // Preset 27, Bank 0: Electric Guitar -> Inst 3
    // Preset 32, Bank 0: Upright Bass -> Inst 4
    // Preset 33, Bank 0: Electric Bass -> Inst 5
    // Preset 0, Bank 128: GM Drum Kit -> Inst 6
    // Plus terminal EOS / EOP records.

    // --- Instruments & Zones ---
    // Let's compute instrument zones:
    // Inst 0..5 have 1 zone each (key range 0..127, pointing to sample 0..4)
    // Inst 6 (Drum Kit) has multiple zones:
    // - Kick: keys 35..36 -> Sample 5
    // - Snare: keys 38..40 -> Sample 6
    // - Rimshot: key 37 -> Sample 10
    // - HiHat: keys 42, 44, 46 -> Sample 7
    // - Ride: keys 51, 53, 59 -> Sample 8
    // - Tom: keys 41, 43, 45, 47, 48, 50 -> Sample 9
    // Drum zones: 6 zones.
    // Total instrument zones = 6 (melodic) + 6 (drum) = 12 zones.

    struct InstZoneDef {
        low_key: u8,
        high_key: u8,
        sample_idx: u16,
        root_key: Option<u8>,
    }

    let mut inst_defs: Vec<Vec<InstZoneDef>> = Vec::new();

    // 0: Piano
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 0, root_key: None }]);
    // 1: EPiano
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 1, root_key: None }]);
    // 2: Organ
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 2, root_key: None }]);
    // 3: Guitar
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 3, root_key: None }]);
    // 4: Upright Bass
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 4, root_key: None }]);
    // 5: Electric Bass
    inst_defs.push(vec![InstZoneDef { low_key: 0, high_key: 127, sample_idx: 4, root_key: None }]);
    // 6: Drum Kit
    inst_defs.push(vec![
        InstZoneDef { low_key: 35, high_key: 36, sample_idx: 5, root_key: Some(36) }, // Kick
        InstZoneDef { low_key: 37, high_key: 37, sample_idx: 10, root_key: Some(37) }, // Rim
        InstZoneDef { low_key: 38, high_key: 40, sample_idx: 6, root_key: Some(38) }, // Snare
        InstZoneDef { low_key: 42, high_key: 46, sample_idx: 7, root_key: Some(42) }, // Hi-Hat
        InstZoneDef { low_key: 41, high_key: 50, sample_idx: 9, root_key: Some(45) }, // Toms
        InstZoneDef { low_key: 51, high_key: 80, sample_idx: 8, root_key: Some(51) }, // Ride & Cymbals
    ]);

    // Build igen (Instrument Generators)
    let mut igen_bytes = Vec::new();
    let mut ibag_bytes = Vec::new();
    let mut current_igen_idx: u16 = 0;
    let mut current_ibag_idx: u16 = 0;

    let mut inst_bag_indices = Vec::new();

    for zones in &inst_defs {
        inst_bag_indices.push(current_ibag_idx);
        for z in zones {
            // Write ibag entry
            ibag_bytes.write_u16::<LittleEndian>(current_igen_idx).unwrap(); // wGenNdx
            ibag_bytes.write_u16::<LittleEndian>(0).unwrap(); // wModNdx
            current_ibag_idx += 1;

            // Generator 1: keyRange (low, high)
            igen_bytes.write_u16::<LittleEndian>(GEN_KEY_RANGE).unwrap();
            igen_bytes.write_u8(z.low_key).unwrap();
            igen_bytes.write_u8(z.high_key).unwrap();
            current_igen_idx += 1;

            if let Some(rk) = z.root_key {
                // Generator: overridingRootKey
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
    // Terminal ibag entry
    ibag_bytes.write_u16::<LittleEndian>(current_igen_idx).unwrap();
    ibag_bytes.write_u16::<LittleEndian>(0).unwrap();
    // Terminal igen entry
    igen_bytes.write_u16::<LittleEndian>(0).unwrap();
    igen_bytes.write_u16::<LittleEndian>(0).unwrap();

    // Build inst chunk
    let mut inst_bytes = Vec::new();
    let inst_names = [
        "Acoustic Grand Piano",
        "Electric Piano",
        "Drawbar Organ",
        "Electric Guitar",
        "Acoustic Upright Bass",
        "Electric Fingered Bass",
        "GM Drum Kit",
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
        PresetDef { name: "Acoustic Grand", patch: 0, bank: 0, inst_idx: 0 },
        PresetDef { name: "Electric Piano", patch: 4, bank: 0, inst_idx: 1 },
        PresetDef { name: "Drawbar Organ", patch: 16, bank: 0, inst_idx: 2 },
        PresetDef { name: "Electric Guitar", patch: 27, bank: 0, inst_idx: 3 },
        PresetDef { name: "Upright Bass", patch: 32, bank: 0, inst_idx: 4 },
        PresetDef { name: "Electric Bass", patch: 33, bank: 0, inst_idx: 5 },
        PresetDef { name: "Standard Drum Kit", patch: 0, bank: 128, inst_idx: 6 },
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

    // Now write all pdta subchunks in standard order
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
