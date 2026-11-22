# Time Circle

**Time Circle** is a Rust desktop application (`eframe` / `egui`) for generating, exploring, auditioning, and exporting blues chord progressions accompanied by realistic drum grooves and walking bass lines.

---

## Features

- **Radial "Time Circle" Canvas**:
  - Arranges the 8, 12, or 16 bars of a blues progression around a clock-like circular wheel with a real-time rotating playhead.
  - Interactive harmonic function color-coding (Tonic `I`, Subdominant `IV`, Dominant `V`, Passing `#IVdim7`, `ii-V`, and turnaround chords).
  - Subdivides each bar into **1, 2, or 4 chords** (up to 1 chord per beat).
  - Concentric animated visualizer rings for **Bass pulses** and **Drum groove hits**.
  - Click any bar/sector to select and edit its chord; click Audition to hear voicings.

- **Blues Chord Progression Generator**:
  - **7 Curated Presets**: Standard 12-Bar Blues, Quick-Change 12-Bar, Chicago Slow 12-Bar (`#IVdim7`), Jazz / Bird Blues 12-Bar, Minor Blues, 8-Bar Blues (*Key to the Highway*), and 16-Bar Extended Blues.
  - **Key Transposition**: Seamless transposition across all 12 keys (`C` through `B`).
  - **⚡ Mutate Button & Spice Slider**: Injects idiomatic blues substitutions (Quick-Change, `#IVdim7` passing chords, secondary dominant `ii-V` prep, 9th/13th extensions).
  - **🔄 Turnaround Button**: Cycles and generates classic 2-bar blues turnarounds (e.g. `| I7 VI7 | ii7 V7alt |`, tritone walkdowns `| I7 bIII7 | bII7 I7 |`, and 4-beat harmonic runs).
  - **One-Tap Blues Degree Buttons**: Quickly replace or build chords with `I7`, `I9`, `IV7`, `IV9`, `#IVdim7`, `V7`, `V7alt`, `ii7`, `VI7`, `bII7`.

- **Mixer & Accompaniment Engine**:
  - **Chords Track (Channel 1)**:
    - Instruments: Acoustic Grand Piano, Rhodes Electric Piano, Hammond B3 Organ, Electric Guitar.
    - Comping Styles: Sustained Pads, Chicago Shuffle Chops (2 & 4), Slow 12/8 Triplet Arpeggio, Freddie Green Four-on-the-Floor, Charleston Syncopated, New Orleans Funky Stabs.
  - **Bass Track (Channel 2)**:
    - Instruments: Upright Acoustic Bass, Fingered Electric Bass.
    - Styles: Walking Bass (Quarter notes with chromatic approach), Texas/Chicago Shuffle Boogie (8th swing), Slow 12/8 Blues Bass, Two-Feel (Root–Fifth).
  - **Drum Beat Track (Channel 10)**:
    - Presets: Chicago Shuffle, Slow 12/8 Blues, Texas SRV Double-Shuffle, New Orleans Second Line Funk, Jazz Blues Swing, Straight Rock Blues.
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
1. Select your desired **Key** (e.g., `A` or `E`) and **Form Preset** (e.g., `Standard 12-Bar Blues` or `Chicago Slow 12-Bar`).
2. Press **▶ Play** to start playback.
3. Switch **Drum Beat** (e.g., `Chicago Shuffle` or `Texas SRV Double-Shuffle`) and **Comping Style** on the fly.
4. Adjust the **Swing** and **BPM** sliders to taste.
5. Click **⚡ Mutate** or **🔄 Turnaround** to explore musical variations.
6. Click any sector on the Time Circle to edit individual chords or subdivide bars into 2 or 4 beats.
7. Click **💾 Export .mid** to save the multi-track MIDI file for your DAW.
