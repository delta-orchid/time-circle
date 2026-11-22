mod arranger;
mod audio;
mod midi_export;
mod soundfont_gen;
mod theory;
mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 780.0])
            .with_min_inner_size([880.0, 600.0])
            .with_title("Time Circle — Chord Progression & Groove Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Time Circle",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::TimeCircleApp::new(cc)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use theory::{
        PitchClass, FormPreset, FormCategory, Chord, ChordQuality, Progression,
        mutate_progression, mutate_turnaround, degree_presets_for_category,
    };
    use arranger::{
        ArrangerSettings, BassInstrument, BassStyle, ChordCompingStyle, ChordInstrument,
        DrumBeatPreset, DrumKit, PercussionInstrument, generate_arrangement_notes,
        GM_PERC_COWBELL, GM_PERC_HAND_CLAP, GM_PERC_SHAKER, GM_PERC_TAMBOURINE,
    };
    use soundfont_gen::generate_gm_soundfont;
    use midi_export::build_smf_bytes;

    #[test]
    fn test_all_form_presets_and_transposition() {
        assert_eq!(FormPreset::ALL.len(), 17);

        for preset in FormPreset::ALL {
            let prog = Progression::new_preset(PitchClass::E, preset);
            assert_eq!(prog.bars.len(), preset.bar_count());
            assert_eq!(prog.key, PitchClass::E);
            assert_eq!(prog.bar_count(), preset.bar_count());

            // Test transposition
            let mut prog_g = prog.clone();
            prog_g.transpose(PitchClass::G);
            assert_eq!(prog_g.key, PitchClass::G);
            assert_eq!(prog_g.bars.len(), preset.bar_count());

            // Test category filtering
            let category_presets = FormPreset::presets_for_category(preset.category());
            assert!(category_presets.contains(&preset));
        }

        // Section labels check
        assert_eq!(FormPreset::PopVerseChorus16.section_label(0), Some("A: VERSE"));
        assert_eq!(FormPreset::PopVerseChorus16.section_label(8), Some("B: CHORUS"));
        assert_eq!(FormPreset::PopVerseChorus16.section_label(1), None);

        assert_eq!(FormPreset::SoulAaba16.section_label(0), Some("A1"));
        assert_eq!(FormPreset::SoulAaba16.section_label(8), Some("B: BRIDGE"));
    }

    #[test]
    fn test_chord_qualities_voicing() {
        assert_eq!(ChordQuality::ALL.len(), 16);

        let c_maj = Chord::new(PitchClass::C, ChordQuality::Maj);
        let voicing_maj = c_maj.voicing(60);
        assert!(!voicing_maj.is_empty());

        let a_min = Chord::new(PitchClass::A, ChordQuality::Min);
        let voicing_min = a_min.voicing(60);
        assert!(!voicing_min.is_empty());

        let g_sus4 = Chord::new(PitchClass::G, ChordQuality::Sus4);
        assert_eq!(g_sus4.name(false), "Gsus4");

        let d_add9 = Chord::new(PitchClass::D, ChordQuality::Add9);
        assert_eq!(d_add9.name(false), "Dadd9");

        let e_7sus4 = Chord::new(PitchClass::E, ChordQuality::Dom7Sus4);
        assert_eq!(e_7sus4.name(false), "E7sus4");
    }

    #[test]
    fn test_mutations_and_turnaround() {
        // Blues mutation & turnaround
        let mut prog_blues = Progression::new_preset(PitchClass::A, FormPreset::Standard12);
        mutate_progression(&mut prog_blues, 0.8);
        assert_eq!(prog_blues.bars.len(), 12);

        mutate_turnaround(&mut prog_blues);
        assert_eq!(prog_blues.bars.len(), 12);

        // Pop/Rock mutation (chord extensions upgrade)
        let mut prog_pop = Progression::new_preset(PitchClass::C, FormPreset::PopAxis4);
        mutate_progression(&mut prog_pop, 0.9);
        assert_eq!(prog_pop.bars.len(), 4);

        // Non-blues turnaround should NOT overwrite chords
        let bars_before = prog_pop.bars.clone();
        mutate_turnaround(&mut prog_pop);
        assert_eq!(prog_pop.bars, bars_before);
    }

    #[test]
    fn test_degree_presets_per_category() {
        for cat in FormCategory::ALL {
            let degrees = degree_presets_for_category(cat, PitchClass::C);
            assert!(!degrees.is_empty(), "Category {:?} should provide degree buttons", cat);
        }
    }

    #[test]
    fn test_arranger_new_accompaniment_styles() {
        let prog = Progression::new_preset(PitchClass::C, FormPreset::PopAxis4);

        let mut settings = ArrangerSettings::default();
        settings.chord_style = ChordCompingStyle::PopEighthPulse;
        settings.bass_style = BassStyle::EighthRootDrive;
        settings.drum_preset = DrumBeatPreset::PopRockBackbeat;

        let notes = generate_arrangement_notes(&prog, &settings);
        assert!(!notes.is_empty());
        assert!(notes.iter().any(|n| n.channel == 0));
        assert!(notes.iter().any(|n| n.channel == 1));
        assert!(notes.iter().any(|n| n.channel == 9));

        // Test Soul styles
        settings.chord_style = ChordCompingStyle::NeoSoulSyncopated;
        settings.bass_style = BassStyle::MotownSoulGroove;
        settings.drum_preset = DrumBeatPreset::NeoSoulPocket;

        let notes_soul = generate_arrangement_notes(&prog, &settings);
        assert!(!notes_soul.is_empty());

        // Test Motown soul drum preset
        settings.drum_preset = DrumBeatPreset::MotownSoulBeat;
        let notes_motown = generate_arrangement_notes(&prog, &settings);
        assert!(!notes_motown.is_empty());
    }

    #[test]
    fn test_in_memory_soundfont_generation() {
        let bytes = generate_gm_soundfont();
        assert!(!bytes.is_empty());
        let mut cursor = Cursor::new(bytes);
        match rustysynth::SoundFont::new(&mut cursor) {
            Ok(_) => println!("SoundFont parsed successfully!"),
            Err(e) => panic!("SoundFont parser failed with error: {:?}", e),
        }
    }

    #[test]
    fn test_midi_smf_export_roundtrip() {
        let prog = Progression::new_preset(PitchClass::F, FormPreset::PopVerseChorus16);
        let mut settings = ArrangerSettings::default();
        settings.drum_kit = DrumKit::TR808Electronic;
        settings.percussion_inst = PercussionInstrument::Cowbell;
        let smf_bytes = build_smf_bytes(&prog, &settings, 2).expect("SMF export should succeed");

        assert!(!smf_bytes.is_empty());
        let parsed = midly::Smf::parse(&smf_bytes);
        assert!(parsed.is_ok(), "SMF file should be valid standard MIDI");
        let smf = parsed.unwrap();
        assert!(smf.tracks.len() >= 3, "SMF should contain conductor, chords, bass, drums tracks");
    }

    #[test]
    fn test_expanded_chord_and_bass_instruments() {
        assert_eq!(ChordInstrument::ALL.len(), 10);
        for inst in ChordInstrument::ALL {
            assert!(!inst.name().is_empty());
            assert!(inst.program_number() <= 127);
        }

        assert_eq!(BassInstrument::ALL.len(), 8);
        for inst in BassInstrument::ALL {
            assert!(!inst.name().is_empty());
            assert!(inst.program_number() <= 127);
        }
    }

    #[test]
    fn test_drum_kits_and_percussion_layers() {
        assert_eq!(DrumKit::ALL.len(), 6);
        for kit in DrumKit::ALL {
            assert!(!kit.name().is_empty());
            assert!(kit.program_number() <= 127);
        }

        assert_eq!(PercussionInstrument::ALL.len(), 5);
        for p in PercussionInstrument::ALL {
            assert!(!p.name().is_empty());
        }

        let prog = Progression::new_preset(PitchClass::C, FormPreset::Standard12);

        // Test Tambourine scheduling
        let mut settings = ArrangerSettings::default();
        settings.percussion_inst = PercussionInstrument::Tambourine;
        settings.percussion_volume = 0.8;
        let notes_tamb = generate_arrangement_notes(&prog, &settings);
        assert!(notes_tamb.iter().any(|n| n.channel == 9 && n.note == GM_PERC_TAMBOURINE));

        // Test Cowbell scheduling
        settings.percussion_inst = PercussionInstrument::Cowbell;
        let notes_cow = generate_arrangement_notes(&prog, &settings);
        assert!(notes_cow.iter().any(|n| n.channel == 9 && n.note == GM_PERC_COWBELL));

        // Test Handclap scheduling
        settings.percussion_inst = PercussionInstrument::HandClap;
        let notes_clap = generate_arrangement_notes(&prog, &settings);
        assert!(notes_clap.iter().any(|n| n.channel == 9 && n.note == GM_PERC_HAND_CLAP));

        // Test Shaker scheduling
        settings.percussion_inst = PercussionInstrument::Shaker;
        let notes_shaker = generate_arrangement_notes(&prog, &settings);
        assert!(notes_shaker.iter().any(|n| n.channel == 9 && n.note == GM_PERC_SHAKER));
    }
}
