mod arranger;
mod audio;
mod mcp_bridge;
mod midi_export;
mod soundfont_gen;
mod theory;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1120.0, 780.0])
            .with_min_inner_size([880.0, 600.0])
            .with_title("Time Circle — Multi-Genre Chord Progression & Groove Generator"),
        ..Default::default()
    };

    eframe::run_native(
        "Time Circle",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::TimeCircleApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;

    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("time_circle_canvas")
            .expect("Failed to find canvas with id 'time_circle_canvas'")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Failed to cast element to HtmlCanvasElement");

        let runner = eframe::WebRunner::new();
        let _ = runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(ui::TimeCircleApp::new(cc)))),
            )
            .await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use theory::{PitchClass, BluesFormPreset, Progression, mutate_progression, mutate_turnaround};
    use arranger::{ArrangerSettings, generate_arrangement_notes};
    use soundfont_gen::generate_gm_soundfont;
    use midi_export::build_smf_bytes;

    #[test]
    fn test_blues_presets_and_transposition() {
        for preset in BluesFormPreset::ALL {
            let prog = Progression::new_preset(PitchClass::E, preset);
            assert_eq!(prog.bars.len(), preset.bar_count());
            assert_eq!(prog.key, PitchClass::E);

            // Test transposition
            let mut prog_g = prog.clone();
            prog_g.transpose(PitchClass::G);
            assert_eq!(prog_g.key, PitchClass::G);
            assert_eq!(prog_g.bars.len(), preset.bar_count());
        }
    }

    #[test]
    fn test_mutations_and_turnaround() {
        let mut prog = Progression::new_preset(PitchClass::A, BluesFormPreset::Standard12);
        mutate_progression(&mut prog, 0.8);
        assert_eq!(prog.bars.len(), 12);

        mutate_turnaround(&mut prog);
        assert_eq!(prog.bars.len(), 12);
    }

    #[test]
    fn test_arranger_and_event_generation() {
        let prog = Progression::new_preset(PitchClass::C, BluesFormPreset::Standard12);
        let settings = ArrangerSettings::default();
        let notes = generate_arrangement_notes(&prog, &settings);

        assert!(!notes.is_empty(), "Arrangement should generate notes");
        let has_chords = notes.iter().any(|n| n.channel == 0);
        let has_bass = notes.iter().any(|n| n.channel == 1);
        let has_drums = notes.iter().any(|n| n.channel == 9);

        assert!(has_chords, "Should generate chords on channel 0");
        assert!(has_bass, "Should generate bass on channel 1");
        assert!(has_drums, "Should generate drums on channel 9");
    }

    #[test]
    fn test_in_memory_soundfont_generation() {
        let bytes = generate_gm_soundfont();
        assert!(!bytes.is_empty());
        println!("SF2 length = {}, bytes 12..90: {:?}", bytes.len(), &bytes[12..90]);
        let mut cursor = Cursor::new(bytes);
        match rustysynth::SoundFont::new(&mut cursor) {
            Ok(_) => println!("SoundFont parsed successfully!"),
            Err(e) => panic!("SoundFont parser failed with error: {:?}", e),
        }
    }

    #[test]
    fn test_midi_smf_export_roundtrip() {
        let prog = Progression::new_preset(PitchClass::F, BluesFormPreset::QuickChange12);
        let settings = ArrangerSettings::default();
        let smf_bytes = build_smf_bytes(&prog, &settings, 2).expect("SMF export should succeed");

        assert!(!smf_bytes.is_empty());
        let parsed = midly::Smf::parse(&smf_bytes);
        assert!(parsed.is_ok(), "SMF file should be valid standard MIDI");
        let smf = parsed.unwrap();
        assert!(smf.tracks.len() >= 3, "SMF should contain conductor, chords, bass, drums tracks");
    }
}
