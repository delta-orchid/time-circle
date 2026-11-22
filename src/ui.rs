use egui::{
    vec2, Align2, Color32, FontFamily, FontId, Pos2, Sense, Stroke, Ui,
};
use std::f32::consts::PI;

use crate::arranger::{
    ArrangerSettings, BassInstrument, BassStyle, ChordCompingStyle, ChordInstrument,
    DrumBeatPreset, TICKS_PER_BAR,
};
use crate::audio::AudioEngine;
use crate::theory::{
    mutate_progression, mutate_turnaround, BluesFormPreset, Chord, ChordQuality,
    PitchClass, Progression,
};

pub struct TimeCircleApp {
    pub prog: Progression,
    pub settings: ArrangerSettings,
    pub audio: Result<AudioEngine, String>,
    pub selected_bar: usize,
    pub selected_slot: usize,
    pub spice_level: f32,
    pub chorus_export_count: usize,
    pub export_status_message: Option<(String, f32)>, // message, expiry time
}

impl TimeCircleApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let prog = Progression::new_preset(PitchClass::A, BluesFormPreset::Standard12);
        let settings = ArrangerSettings::default();

        let audio = AudioEngine::new();
        if let Ok(ref eng) = audio {
            eng.update_arrangement(&prog, &settings);
        }

        Self {
            prog,
            settings,
            audio,
            selected_bar: 0,
            selected_slot: 0,
            spice_level: 0.5,
            chorus_export_count: 2,
            export_status_message: None,
        }
    }

    pub fn sync_audio_arrangement(&self) {
        if let Ok(ref eng) = self.audio {
            eng.update_arrangement(&self.prog, &self.settings);
        }
    }
}

impl eframe::App for TimeCircleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaint while playing for smooth playhead animation
        let is_playing = self.audio.as_ref().map(|a| a.is_playing()).unwrap_or(false);
        if is_playing {
            ctx.request_repaint();
        }

        // Top Control / Transport Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.render_top_bar(ui);
        });

        // Bottom Status / Message Bar
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("SoundFont: Built-in GM SoundFont");
                ui.separator();
                if let Some((ref msg, _)) = self.export_status_message {
                    ui.colored_label(Color32::from_rgb(100, 255, 160), msg);
                } else {
                    ui.weak("Time Circle ready. Click sectors on the wheel to edit chords.");
                }
            });
        });

        // Right Inspector Panel
        egui::SidePanel::right("inspector_panel")
            .min_width(320.0)
            .default_width(340.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_inspector(ui);
                });
            });

        // Center Radial Time Circle
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_radial_canvas(ui);
        });
    }
}

impl TimeCircleApp {
    fn render_top_bar(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.heading("TIME CIRCLE");
            ui.separator();

            let is_playing = self.audio.as_ref().map(|a| a.is_playing()).unwrap_or(false);

            if ui.button(if is_playing { "⏸ Pause" } else { "▶ Play" }).clicked() {
                if let Ok(ref eng) = self.audio {
                    eng.set_playing(!is_playing);
                }
            }

            if ui.button("⏹ Stop").clicked() {
                if let Ok(ref eng) = self.audio {
                    eng.set_playing(false);
                    eng.set_tick(0);
                }
            }

            ui.separator();

            // BPM Slider
            ui.label("BPM:");
            let mut bpm = self.settings.bpm;
            if ui.add(egui::Slider::new(&mut bpm, 45.0..=220.0).text("")).changed() {
                self.settings.bpm = bpm;
                self.sync_audio_arrangement();
            }

            // Swing Slider
            ui.label("Swing:");
            let mut swing_pct = (self.settings.swing_ratio * 100.0).round() as u32;
            if ui.add(egui::Slider::new(&mut swing_pct, 50..=75).suffix("%")).changed() {
                self.settings.swing_ratio = swing_pct as f32 / 100.0;
                self.sync_audio_arrangement();
            }

            ui.separator();

            // Key Selector
            ui.label("Key:");
            egui::ComboBox::from_id_salt("key_select")
                .selected_text(self.prog.key.name(false))
                .show_ui(ui, |ui| {
                    for key in PitchClass::ALL {
                        if ui.selectable_value(&mut self.prog.key, key, key.name(false)).clicked() {
                            self.prog.transpose(key);
                            self.sync_audio_arrangement();
                        }
                    }
                });

            // Preset Selector
            ui.label("Form:");
            egui::ComboBox::from_id_salt("form_preset")
                .selected_text(self.prog.preset.name())
                .show_ui(ui, |ui| {
                    for preset in BluesFormPreset::ALL {
                        if ui.selectable_value(&mut self.prog.preset, preset, preset.name()).clicked() {
                            self.prog = Progression::new_preset(self.prog.key, preset);
                            self.selected_bar = 0;
                            self.selected_slot = 0;
                            self.sync_audio_arrangement();
                        }
                    }
                });

            ui.separator();

            // Generator Buttons
            if ui.button("⚡ Mutate").on_hover_text("Apply blues substitutions and extensions").clicked() {
                mutate_progression(&mut self.prog, self.spice_level);
                self.sync_audio_arrangement();
            }

            if ui.button("🔄 Turnaround").on_hover_text("Generate a classic turnaround in the final bars").clicked() {
                mutate_turnaround(&mut self.prog);
                self.sync_audio_arrangement();
            }

            ui.separator();

            // MIDI Export Button
            ui.label("Choruses:");
            ui.add(egui::DragValue::new(&mut self.chorus_export_count).range(1..=16));

            if ui.button("💾 Export .mid").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Standard MIDI File", &["mid", "midi"])
                    .set_file_name(&format!("blues_{}.mid", self.prog.key.name(false).to_lowercase()))
                    .save_file()
                {
                    match crate::midi_export::export_smf_file(
                        &path,
                        &self.prog,
                        &self.settings,
                        self.chorus_export_count,
                    ) {
                        Ok(_) => {
                            self.export_status_message = Some((
                                format!("Exported {} successfully!", path.display()),
                                5.0,
                            ));
                        }
                        Err(e) => {
                            self.export_status_message = Some((format!("Export failed: {}", e), 5.0));
                        }
                    }
                }
            }
        });
    }

    fn render_radial_canvas(&mut self, ui: &mut Ui) {
        let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
        let rect = response.rect;
        let center = rect.center();
        let radius = (rect.width().min(rect.height()) * 0.46).max(80.0);

        let total_bars = self.prog.bars.len();
        let total_ticks = (total_bars as u32) * TICKS_PER_BAR;
        let cur_tick = self.audio.as_ref().map(|a| a.current_tick()).unwrap_or(0);

        let cur_bar_idx = (cur_tick / TICKS_PER_BAR) as usize % total_bars;
        let cur_beat_in_bar = ((cur_tick % TICKS_PER_BAR) / crate::arranger::TICKS_PER_BEAT) as usize;

        // Radius rings
        let r_outer = radius;
        let r_chord_inner = radius * 0.68;
        let r_bass_inner = radius * 0.50;
        let r_drum_inner = radius * 0.32;
        let r_hub = radius * 0.22;

        // Background circle
        painter.circle_filled(center, r_outer + 4.0, Color32::from_rgb(18, 20, 24));
        painter.circle_stroke(center, r_outer + 4.0, Stroke::new(1.5_f32, Color32::from_rgb(45, 52, 64)));

        // Handle Click on Canvas for Bar Selection
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let d = pos - center;
                let dist = d.length();
                if dist >= r_chord_inner && dist <= r_outer {
                    // Compute angle in clockwise from 12 o'clock
                    let mut angle = d.y.atan2(d.x) + (PI * 0.5);
                    if angle < 0.0 {
                        angle += 2.0 * PI;
                    }
                    let bar_angle = (2.0 * PI) / total_bars as f32;
                    let bar_clicked = (angle / bar_angle).floor() as usize % total_bars;
                    self.selected_bar = bar_clicked;

                    // Also select sub-slot if divided
                    let slot_count = self.prog.bars[bar_clicked].slots.len();
                    let offset_in_bar = angle - (bar_clicked as f32 * bar_angle);
                    let slot_idx = ((offset_in_bar / bar_angle) * slot_count as f32).floor() as usize;
                    self.selected_slot = slot_idx.min(slot_count - 1);

                    // Preview selected chord
                    if let Ok(ref eng) = self.audio {
                        let chord = self.prog.bars[self.selected_bar].slots[self.selected_slot];
                        eng.preview_chord(&chord);
                    }
                }
            }
        }

        // Draw Bar Sectors (Outer Ring)
        let bar_angle = (2.0 * PI) / total_bars as f32;

        for bar_idx in 0..total_bars {
            let bar = &self.prog.bars[bar_idx];
            let start_ang = (bar_idx as f32 * bar_angle) - (PI * 0.5);
            let slot_count = bar.slots.len();
            let slot_angle = bar_angle / slot_count as f32;

            for (s_idx, chord) in bar.slots.iter().enumerate() {
                let s_start = start_ang + (s_idx as f32 * slot_angle);
                let s_end = s_start + slot_angle;

                let is_current = bar_idx == cur_bar_idx
                    && (slot_count == 1
                        || (slot_count == 2 && (s_idx == 0 && cur_beat_in_bar < 2 || s_idx == 1 && cur_beat_in_bar >= 2))
                        || (slot_count == 4 && s_idx == cur_beat_in_bar));
                let is_selected = bar_idx == self.selected_bar && s_idx == self.selected_slot;

                // Sector base color depending on harmonic function
                let mut base_col = chord_harmonic_color(chord.root, self.prog.key);
                if is_current {
                    base_col = Color32::from_rgb(
                        base_col.r().saturating_add(60),
                        base_col.g().saturating_add(60),
                        base_col.b().saturating_add(60),
                    );
                }
                if is_selected {
                    base_col = Color32::from_rgb(255, 240, 100);
                }

                draw_arc_sector(
                    &painter,
                    center,
                    r_chord_inner,
                    r_outer,
                    s_start,
                    s_end,
                    base_col,
                    Stroke::new(1.0_f32, Color32::from_rgb(25, 30, 38)),
                );

                // Draw Chord Symbol text
                let mid_ang = (s_start + s_end) * 0.5;
                let text_r = (r_chord_inner + r_outer) * 0.5;
                let text_pos = center + vec2(mid_ang.cos() * text_r, mid_ang.sin() * text_r);
                let text_color = if is_selected {
                    Color32::from_rgb(15, 15, 20)
                } else {
                    Color32::WHITE
                };

                let chord_name = chord.name(false);
                painter.text(
                    text_pos,
                    Align2::CENTER_CENTER,
                    chord_name,
                    FontId::new(14.0, FontFamily::Proportional),
                    text_color,
                );

                // Bar number on first slot
                if s_idx == 0 {
                    let num_r = r_outer - 12.0;
                    let num_pos = center + vec2(s_start.cos() * num_r, s_start.sin() * num_r);
                    painter.text(
                        num_pos,
                        Align2::CENTER_CENTER,
                        format!("{}", bar_idx + 1),
                        FontId::new(10.0, FontFamily::Monospace),
                        Color32::from_rgba_unmultiplied(255, 255, 255, 140),
                    );
                }
            }
        }

        // Draw Bass Ring (Middle Ring)
        painter.circle_stroke(center, r_chord_inner, Stroke::new(1.0_f32, Color32::from_rgb(60, 65, 75)));
        painter.circle_stroke(center, r_bass_inner, Stroke::new(1.0_f32, Color32::from_rgb(45, 50, 60)));

        let bass_active_color = if self.settings.bass_enabled {
            Color32::from_rgb(50, 140, 240)
        } else {
            Color32::from_rgb(40, 50, 65)
        };
        // Draw 4 beat ticks per bar on bass ring
        for b in 0..(total_bars * 4) {
            let beat_ang = ((b as f32 / (total_bars * 4) as f32) * 2.0 * PI) - (PI * 0.5);
            let p1 = center + vec2(beat_ang.cos() * r_bass_inner, beat_ang.sin() * r_bass_inner);
            let p2 = center + vec2(beat_ang.cos() * r_chord_inner, beat_ang.sin() * r_chord_inner);
            let is_cur_beat = b == (cur_bar_idx * 4 + cur_beat_in_bar);
            painter.line_segment(
                [p1, p2],
                Stroke::new(if is_cur_beat { 2.5 } else { 1.0_f32 }, if is_cur_beat { Color32::WHITE } else { bass_active_color }),
            );
        }

        // Draw Drum Ring (Inner Ring)
        painter.circle_stroke(center, r_drum_inner, Stroke::new(1.0_f32, Color32::from_rgb(45, 50, 60)));
        let _drum_active_color = if self.settings.drum_enabled {
            Color32::from_rgb(220, 100, 50)
        } else {
            Color32::from_rgb(50, 40, 35)
        };

        // Inner Hub (Current Bar & Chord Readout)
        painter.circle_filled(center, r_drum_inner, Color32::from_rgb(14, 16, 20));
        painter.circle_filled(center, r_hub, Color32::from_rgb(22, 26, 34));
        painter.circle_stroke(center, r_hub, Stroke::new(2.0_f32, Color32::from_rgb(65, 75, 95)));

        let cur_chord = self.prog.bars[cur_bar_idx].get_chord_at_beat(cur_beat_in_bar);
        painter.text(
            center - vec2(0.0, 12.0),
            Align2::CENTER_CENTER,
            format!("BAR {}", cur_bar_idx + 1),
            FontId::new(11.0, FontFamily::Monospace),
            Color32::from_rgb(160, 180, 200),
        );
        painter.text(
            center + vec2(0.0, 8.0),
            Align2::CENTER_CENTER,
            cur_chord.name(false),
            FontId::new(20.0, FontFamily::Proportional),
            Color32::from_rgb(255, 230, 100),
        );

        // Draw Rotating Playhead Arm
        let playhead_progress = cur_tick as f32 / total_ticks as f32;
        let playhead_ang = (playhead_progress * 2.0 * PI) - (PI * 0.5);
        let ph_end = center + vec2(playhead_ang.cos() * (r_outer + 6.0), playhead_ang.sin() * (r_outer + 6.0));
        let ph_start = center + vec2(playhead_ang.cos() * r_hub, playhead_ang.sin() * r_hub);

        // Glowing needle
        painter.line_segment([ph_start, ph_end], Stroke::new(3.0_f32, Color32::from_rgb(255, 80, 80)));
        painter.circle_filled(ph_end, 4.0, Color32::from_rgb(255, 120, 120));
    }

    fn render_inspector(&mut self, ui: &mut Ui) {
        ui.heading("Inspector & Mixer");
        ui.add_space(8.0);

        // Selected Bar & Chord Editor
        ui.group(|ui| {
            let total_bars = self.prog.bars.len();
            if self.selected_bar >= total_bars {
                self.selected_bar = 0;
            }

            ui.horizontal(|ui| {
                ui.strong(format!("Bar {} / {}", self.selected_bar + 1, total_bars));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("▶ Jump").on_hover_text("Jump playhead to this bar").clicked() {
                        if let Ok(ref eng) = self.audio {
                            eng.set_tick((self.selected_bar as u32) * TICKS_PER_BAR);
                        }
                    }
                });
            });

            ui.separator();

            // Slots Subdivision Selector (1, 2, or 4 beats)
            let current_slot_count = self.prog.bars[self.selected_bar].slots.len();
            ui.horizontal(|ui| {
                ui.label("Beats Subdivision:");
                for &slots in &[1, 2, 4] {
                    let text = match slots {
                        1 => "1 Chord (4 beats)",
                        2 => "2 Chords (2 beats)",
                        _ => "4 Chords (1 beat)",
                    };
                    if ui.selectable_label(current_slot_count == slots, text).clicked() {
                        let base = self.prog.bars[self.selected_bar].slots[0];
                        self.prog.bars[self.selected_bar].slots = vec![base; slots];
                        self.selected_slot = 0;
                        self.sync_audio_arrangement();
                    }
                }
            });

            // Slot Tabs
            let current_slot_count = self.prog.bars[self.selected_bar].slots.len();
            if current_slot_count > 1 {
                ui.horizontal(|ui| {
                    ui.label("Editing Slot:");
                    for s in 0..current_slot_count {
                        let chord = self.prog.bars[self.selected_bar].slots[s];
                        let label = format!("Beat {}: {}", (s * (4 / current_slot_count)) + 1, chord.name(false));
                        if ui.selectable_label(self.selected_slot == s, label).clicked() {
                            self.selected_slot = s;
                            if let Ok(ref eng) = self.audio {
                                eng.preview_chord(&chord);
                            }
                        }
                    }
                });
            }

            if self.selected_slot >= current_slot_count {
                self.selected_slot = 0;
            }

            let mut cur_chord = self.prog.bars[self.selected_bar].slots[self.selected_slot];
            let mut changed = false;

            ui.add_space(4.0);

            // One-tap Blues Degree Shortcuts
            ui.label("Blues Degree Presets:");
            ui.horizontal_wrapped(|ui| {
                let key = self.prog.key;
                let degrees = [
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
                ];

                for (name, root, quality) in degrees {
                    if ui.button(name).clicked() {
                        cur_chord = Chord::new(root, quality);
                        changed = true;
                    }
                }
            });

            ui.add_space(6.0);

            // Manual Root & Quality Dropdowns
            ui.horizontal(|ui| {
                ui.label("Root:");
                egui::ComboBox::from_id_salt("chord_root_sel")
                    .selected_text(cur_chord.root.name(false))
                    .show_ui(ui, |ui| {
                        for p in PitchClass::ALL {
                            if ui.selectable_value(&mut cur_chord.root, p, p.name(false)).clicked() {
                                changed = true;
                            }
                        }
                    });

                ui.label("Quality:");
                egui::ComboBox::from_id_salt("chord_qual_sel")
                    .selected_text(cur_chord.quality.symbol())
                    .show_ui(ui, |ui| {
                        for q in ChordQuality::ALL {
                            if ui.selectable_value(&mut cur_chord.quality, q, q.symbol()).clicked() {
                                changed = true;
                            }
                        }
                    });

                if ui.button("🔊 Audition").clicked() {
                    if let Ok(ref eng) = self.audio {
                        eng.preview_chord(&cur_chord);
                    }
                }
            });

            if changed {
                self.prog.bars[self.selected_bar].slots[self.selected_slot] = cur_chord;
                self.sync_audio_arrangement();
                if let Ok(ref eng) = self.audio {
                    eng.preview_chord(&cur_chord);
                }
            }
        });

        ui.add_space(8.0);

        // Accompaniment Mixer Cards
        // 1. Chords Comping Card
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong("🎹 Chords Comping");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.checkbox(&mut self.settings.chord_enabled, "Enable").changed() {
                        self.sync_audio_arrangement();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Instrument:");
                egui::ComboBox::from_id_salt("chords_inst_combo")
                    .selected_text(self.settings.chord_instrument.name())
                    .show_ui(ui, |ui| {
                        for inst in ChordInstrument::ALL {
                            if ui.selectable_value(&mut self.settings.chord_instrument, inst, inst.name()).clicked() {
                                self.sync_audio_arrangement();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Comping Style:");
                egui::ComboBox::from_id_salt("chords_style_combo")
                    .selected_text(self.settings.chord_style.name())
                    .show_ui(ui, |ui| {
                        for style in ChordCompingStyle::ALL {
                            if ui.selectable_value(&mut self.settings.chord_style, style, style.name()).clicked() {
                                self.sync_audio_arrangement();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Volume:");
                if ui.add(egui::Slider::new(&mut self.settings.chord_volume, 0.0..=1.0)).changed() {
                    self.sync_audio_arrangement();
                }
            });
        });

        ui.add_space(6.0);

        // 2. Bass Track Card
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong("🎸 Bass Line");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.checkbox(&mut self.settings.bass_enabled, "Enable").changed() {
                        self.sync_audio_arrangement();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Instrument:");
                egui::ComboBox::from_id_salt("bass_inst_combo")
                    .selected_text(self.settings.bass_instrument.name())
                    .show_ui(ui, |ui| {
                        for inst in BassInstrument::ALL {
                            if ui.selectable_value(&mut self.settings.bass_instrument, inst, inst.name()).clicked() {
                                self.sync_audio_arrangement();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Bass Style:");
                egui::ComboBox::from_id_salt("bass_style_combo")
                    .selected_text(self.settings.bass_style.name())
                    .show_ui(ui, |ui| {
                        for style in BassStyle::ALL {
                            if ui.selectable_value(&mut self.settings.bass_style, style, style.name()).clicked() {
                                self.sync_audio_arrangement();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Volume:");
                if ui.add(egui::Slider::new(&mut self.settings.bass_volume, 0.0..=1.0)).changed() {
                    self.sync_audio_arrangement();
                }
            });
        });

        ui.add_space(6.0);

        // 3. Drums Track Card
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.strong("🥁 Drum Beat");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.checkbox(&mut self.settings.drum_enabled, "Enable").changed() {
                        self.sync_audio_arrangement();
                    }
                });
            });

            ui.horizontal(|ui| {
                ui.label("Beat Preset:");
                egui::ComboBox::from_id_salt("drum_preset_combo")
                    .selected_text(self.settings.drum_preset.name())
                    .show_ui(ui, |ui| {
                        for preset in DrumBeatPreset::ALL {
                            if ui.selectable_value(&mut self.settings.drum_preset, preset, preset.name()).clicked() {
                                self.sync_audio_arrangement();
                            }
                        }
                    });
            });

            ui.checkbox(&mut self.settings.turnaround_drum_fill, "Turnaround Drum Fill").changed();

            ui.horizontal(|ui| {
                ui.label("Volume:");
                if ui.add(egui::Slider::new(&mut self.settings.drum_volume, 0.0..=1.0)).changed() {
                    self.sync_audio_arrangement();
                }
            });
        });

        ui.add_space(6.0);

        // Mutation & Spice Settings Card
        ui.group(|ui| {
            ui.strong("🎲 Mutation / Spice");
            ui.horizontal(|ui| {
                ui.label("Spice Level:");
                ui.add(egui::Slider::new(&mut self.spice_level, 0.0..=1.0).text(""));
            });
            ui.weak("Controls frequency of substitutions (#IVdim7, ii-V, 9ths, 13ths) when clicking Mutate.");
        });
    }
}

/// Color codes chords according to functional blues harmony
fn chord_harmonic_color(root: PitchClass, key: PitchClass) -> Color32 {
    let diff = (root.semitones() as i32 - key.semitones() as i32).rem_euclid(12);
    match diff {
        0 => Color32::from_rgb(32, 105, 145),   // Tonic (I) -> Sapphire Blue
        5 => Color32::from_rgb(180, 115, 30),   // Subdominant (IV) -> Amber Orange
        7 => Color32::from_rgb(175, 45, 55),    // Dominant (V) -> Crimson Red
        6 => Color32::from_rgb(140, 45, 140),   // #IVdim7 -> Deep Purple
        2 | 9 => Color32::from_rgb(45, 130, 95),// ii or VI -> Emerald / Teal
        _ => Color32::from_rgb(60, 70, 85),     // Other / Passing -> Charcoal Slate
    }
}

fn draw_arc_sector(
    painter: &egui::Painter,
    center: Pos2,
    inner_r: f32,
    outer_r: f32,
    start_angle: f32,
    end_angle: f32,
    fill: Color32,
    stroke: Stroke,
) {
    let segments = 16;
    let mut points = Vec::with_capacity(segments * 2 + 2);

    // Outer arc
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let ang = start_angle + t * (end_angle - start_angle);
        points.push(center + vec2(ang.cos() * outer_r, ang.sin() * outer_r));
    }
    // Inner arc (reverse)
    for i in (0..=segments).rev() {
        let t = i as f32 / segments as f32;
        let ang = start_angle + t * (end_angle - start_angle);
        points.push(center + vec2(ang.cos() * inner_r, ang.sin() * inner_r));
    }

    painter.add(egui::Shape::convex_polygon(points, fill, stroke));
}
