# Time Circle

**Time Circle** is a Rust desktop application (`eframe` / `egui`) for generating, exploring, auditioning, and exporting multi-genre chord progressions (**Blues**, **Pop & Rock**, and **Soul & R&B**) accompanied by realistic drum grooves and walking/electric bass lines.

---

## Features

- **Radial "Time Circle" Canvas**:
  - Arranges the 4, 8, 12, or 16 bars of a progression around a clock-like circular wheel with a real-time rotating playhead.
  - Interactive harmonic function color-coding (Tonic `I/i`, Supertonic `ii`, Mediant `iii/bIII`, Subdominant `IV`, Dominant `V`, Submediant `vi/bVI`, Subtonic `bVII`, and Passing `#IVdim7`).
  - Outer-rim **Section Badges** (`[A: VERSE]`, `[B: CHORUS]`, `[B: BRIDGE]`) for multi-section 16-bar forms.
  - Subdivides each bar into **1, 2, or 4 chords** (up to 1 chord per beat).
  - Concentric animated visualizer rings for **Bass pulses** and **Drum groove hits**.
  - Click any bar/sector to select and edit its chord; click Audition to hear voicings.

- **Multi-Genre Chord Progression Generator**:
  - **17 Curated Presets across 3 Categories**:
    - **Blues (7 Presets)**: Standard 12-Bar Blues, Quick-Change 12-Bar, Chicago Slow 12-Bar (`#IVdim7`), Jazz / Bird Blues 12-Bar, Minor Blues 12-Bar, 8-Bar Blues (*Key to the Highway*), and 16-Bar Extended Blues.
    - **Pop & Rock (5 Presets)**: Pop Axis 4-Bar (`I – V – vi – IV`), 50s Doo-Wop / Ballad (`I – vi – IV – V`), Classic Rock Mixolydian (`I – bVII – IV – I`), Andalusian Cadence (`i – bVII – bVI – V7`), and Pop Verse + Chorus 16-Bar (`vi – IV – I – V` -> `I – V – vi – IV`).
    - **Soul & R&B (5 Presets)**: Minor Soul / Bill Withers (`i7 – V7 – i7 – IV9`), Royal Road Soul (`IVmaj7 – V7 – iii7 – vi7`), Neo-Soul / Gospel Climb 8-Bar (`IVmaj7 – III7 | vi7 – v7 I7`), Motown 8-Bar Groove (`I – ii7 – iii7 – IV | I – vi7 – ii7 – V7`), and 16-Bar Soul AABA.
  - **Comprehensive Harmonic Vocabulary (16 Qualities)**:
    - Triads & Pop/Rock extensions: Major (`Maj`), Minor (`m`), `sus4`, `7sus4`, `add9`.
    - Jazz & Blues extensions: `7`, `9`, `13`, `7alt`, `maj7`, `6`, `m7`, `m6`, `m9`, `m7b5`, `dim7`.
  - **Key Transposition**: Seamless transposition across all 12 keys (`C` through `B`).
  - **⚡ Mutate Button & Spice Slider**:
    - **Blues Forms**: Injects idiomatic blues substitutions (Quick-Change, `#IVdim7` passing chords, secondary dominant `ii-V` prep, 9th/13th extensions).
    - **Pop/Rock & Soul Forms**: Uniformly upgrades chord extensions (`Maj -> Add9/Maj7/Maj6`, `Min -> Min7/Min9`, `Dom7 -> Dom9/Dom13/Dom7Sus4`) scaled by spice level.
  - **🔄 Turnaround Button**: Cycles and generates classic 2-bar blues turnarounds (e.g. `| I7 VI7 | ii7 V7alt |`, tritone walkdowns, and 4-beat harmonic runs) on Blues forms; smartly disabled on non-Blues forms.
  - **Category-Adaptive Degree Shortcut Buttons**: One-tap buttons adapt dynamically to the active genre (Blues degrees vs. Pop/Rock diatonic degrees vs. Soul/R&B lush degrees).

- **Mixer & Accompaniment Engine**:
  - **Chords Track (Channel 1)**:
    - Instruments: Acoustic Grand Piano, Rhodes Electric Piano, Hammond B3 Organ, Electric Guitar.
    - Comping Styles: Sustained Pads, Chicago Shuffle Chops (2 & 4), Slow 12/8 Triplet Arpeggio, Freddie Green Four-on-the-Floor, Charleston Syncopated, New Orleans Funky Stabs, **Pop / Rock 8th-Note Pulse**, **Soul / R&B Neo-Soul Pocket**.
  - **Bass Track (Channel 2)**:
    - Instruments: Upright Acoustic Bass, Fingered Electric Bass.
    - Styles: Walking Bass (Quarter notes with chromatic approach), Texas/Chicago Shuffle Boogie (8th swing), Slow 12/8 Blues Bass, Two-Feel (Root–Fifth), **Rock / Pop Driving 8ths**, **Motown / Soul Syncopated**.
  - **Drum Beat Track (Channel 10)**:
    - Presets: Chicago Shuffle, Slow 12/8 Blues, Texas SRV Double-Shuffle, New Orleans Second Line Funk, Jazz Blues Swing, Straight Rock Blues, **Pop / Classic Rock 8th Backbeat**, **Motown Four-on-the-Floor Soul**, **Neo-Soul / R&B Laid-Back Pocket**.
    - **Turnaround Drum Fill**: Optional automated crescendo tom/snare triplet fill on the final bar leading into the top of the next chorus.
  - **Tempo & Swing**: Variable BPM (`45–220`) and Swing feel slider (`50%` straight to `67%` triplet to `75%` heavy shuffle).

- **Audio & MIDI Export**:
  - **Zero-Config Audio Synthesis**: Built-in in-memory General MIDI SoundFont (`.sf2`) engine via `rustysynth` + `cpal` (no external downloads needed).
  - **Custom SoundFont Support**: "Load .SF2" button allows loading any third-party General MIDI `.sf2` soundbank.
  - **External MIDI Out**: Route live playback directly to an external DAW (Ableton, Logic, Reaper) or hardware synth via macOS IAC Driver / USB MIDI.
  - **SMF Format 1 Export**: Exports multi-track Standard MIDI Files (`.mid`) with separate tracks for Conductor (tempo + time signature + chord text markers on the DAW timeline), Chords, Bass, and Drums, across configurable chorus repetitions (`1x` to `16x`).

---

## Quick Start

### Prerequisites
- Rust & Cargo (1.75+ recommended)

### Build & Run
```bash
# Run tests
cargo test

# Launch the desktop UI
cargo run --release
```

### Basic Workflow
1. Select your desired **Key** (e.g., `C`, `A`, or `E`) and **Form Preset** from the categorized dropdown (e.g., `Pop Axis 4-Bar`, `Motown 8-Bar Groove`, or `Chicago Slow 12-Bar`).
2. Press **▶ Play** to start playback.
3. Switch **Drum Beat** (e.g., `Pop / Classic Rock 8th Backbeat` or `Neo-Soul / R&B Pocket`) and **Comping Style** on the fly.
4. Adjust the **Swing** and **BPM** sliders to taste.
5. Click **⚡ Mutate** to explore musical variations and chord extensions.
6. Click any sector on the Time Circle to edit individual chords, audition voicings, or subdivide bars into 2 or 4 beats.
7. Click **💾 Export .mid** to save the multi-track MIDI file for your DAW.
