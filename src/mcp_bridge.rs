#[cfg(target_arch = "wasm32")]
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

pub const DEFAULT_WS_URL: &str = "ws://127.0.0.1:3847";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum McpInboundCommand {
    #[serde(rename = "get_state")]
    GetState { request_id: String },

    #[serde(rename = "transport")]
    Transport {
        request_id: String,
        action: String, // "play", "pause", "stop", "jump"
        bar: Option<usize>,
    },

    #[serde(rename = "set_progression")]
    SetProgression {
        request_id: String,
        key: Option<String>,
        preset: Option<String>,
    },

    #[serde(rename = "transpose")]
    Transpose {
        request_id: String,
        key: String,
    },

    #[serde(rename = "set_bar_chords")]
    SetBarChords {
        request_id: String,
        bar_index: usize,
        chords: Vec<String>,
    },

    #[serde(rename = "mutate")]
    Mutate {
        request_id: String,
        spice: Option<f32>,
    },

    #[serde(rename = "turnaround")]
    Turnaround { request_id: String },

    #[serde(rename = "audition_chord")]
    AuditionChord {
        request_id: String,
        chord: String,
    },

    #[serde(rename = "update_mixer")]
    UpdateMixer {
        request_id: String,
        bpm: Option<f32>,
        swing_pct: Option<f32>,
        chord_enabled: Option<bool>,
        chord_instrument: Option<String>,
        chord_style: Option<String>,
        chord_volume: Option<f32>,
        bass_enabled: Option<bool>,
        bass_instrument: Option<String>,
        bass_style: Option<String>,
        bass_volume: Option<f32>,
        drum_enabled: Option<bool>,
        drum_kit: Option<String>,
        drum_preset: Option<String>,
        drum_volume: Option<f32>,
        percussion_inst: Option<String>,
        percussion_volume: Option<f32>,
        turnaround_drum_fill: Option<bool>,
    },

    #[serde(rename = "export_midi")]
    ExportMidi {
        request_id: String,
        choruses: Option<usize>,
        trigger_download: Option<bool>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum McpOutboundMessage {
    #[serde(rename = "state_snapshot")]
    StateSnapshot { state: SessionState },

    #[serde(rename = "command_response")]
    CommandResponse {
        request_id: String,
        ok: bool,
        message: String,
        state: Option<SessionState>,
        midi_base64: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub key: String,
    pub preset: String,
    pub category: String,
    pub bar_count: usize,
    pub bars: Vec<BarSummary>,
    pub settings: MixerSummary,
    pub is_playing: bool,
    pub current_tick: u32,
    pub current_bar: usize,
    pub spice_level: f32,
    pub chorus_export_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarSummary {
    pub index: usize,
    pub slots: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixerSummary {
    pub bpm: f32,
    pub swing_pct: u32,
    pub chord_enabled: bool,
    pub chord_instrument: String,
    pub chord_style: String,
    pub chord_volume: f32,
    pub bass_enabled: bool,
    pub bass_instrument: String,
    pub bass_style: String,
    pub bass_volume: f32,
    pub drum_enabled: bool,
    pub drum_kit: String,
    pub drum_preset: String,
    pub drum_volume: f32,
    pub percussion_inst: String,
    pub percussion_volume: f32,
    pub turnaround_drum_fill: bool,
}

// -------------------------------------------------------------
// WebAssembly implementation (web-sys::WebSocket)
// -------------------------------------------------------------
#[cfg(target_arch = "wasm32")]
pub struct McpBridge {
    pub url: String,
    pub auto_connect: bool,
    pub is_connected: Arc<Mutex<bool>>,
    ws: Option<web_sys::WebSocket>,
    inbox: Arc<Mutex<VecDeque<McpInboundCommand>>>,
    last_connect_attempt: f64,
}

#[cfg(target_arch = "wasm32")]
impl McpBridge {
    pub fn new(url: &str) -> Self {
        let is_connected = Arc::new(Mutex::new(false));
        let inbox = Arc::new(Mutex::new(VecDeque::new()));

        let mut bridge = Self {
            url: url.to_string(),
            auto_connect: true,
            is_connected,
            ws: None,
            inbox,
            last_connect_attempt: 0.0,
        };
        bridge.connect();
        bridge
    }

    pub fn connect(&mut self) {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;

        if let Ok(ws) = web_sys::WebSocket::new(&self.url) {
            let conn_flag = Arc::clone(&self.is_connected);
            let inbox_clone = Arc::clone(&self.inbox);

            // onopen
            let conn_onopen = Arc::clone(&conn_flag);
            let onopen_callback = Closure::<dyn FnMut()>::new(move || {
                if let Ok(mut c) = conn_onopen.lock() {
                    *c = true;
                }
            });
            ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
            onopen_callback.forget();

            // onclose
            let conn_onclose = Arc::clone(&conn_flag);
            let onclose_callback = Closure::<dyn FnMut(web_sys::CloseEvent)>::new(move |_| {
                if let Ok(mut c) = conn_onclose.lock() {
                    *c = false;
                }
            });
            ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
            onclose_callback.forget();

            // onerror
            let conn_onerror = Arc::clone(&conn_flag);
            let onerror_callback = Closure::<dyn FnMut(web_sys::ErrorEvent)>::new(move |_| {
                if let Ok(mut c) = conn_onerror.lock() {
                    *c = false;
                }
            });
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();

            // onmessage
            let onmessage_callback = Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |e: web_sys::MessageEvent| {
                if let Some(txt) = e.data().as_string() {
                    if let Ok(cmd) = serde_json::from_str::<McpInboundCommand>(&txt) {
                        if let Ok(mut in_queue) = inbox_clone.lock() {
                            in_queue.push_back(cmd);
                        }
                    }
                }
            });
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();

            self.ws = Some(ws);
        }
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.lock().map(|c| *c).unwrap_or(false)
    }

    pub fn poll_commands(&mut self) -> Vec<McpInboundCommand> {
        let mut out = Vec::new();
        if let Ok(mut queue) = self.inbox.lock() {
            while let Some(cmd) = queue.pop_front() {
                out.push(cmd);
            }
        }

        // Periodic reconnect attempt if auto_connect is on
        let now = js_sys::Date::now();
        if self.auto_connect && !self.is_connected() && (now - self.last_connect_attempt > 3000.0) {
            self.last_connect_attempt = now;
            self.connect();
        }

        out
    }

    pub fn send(&mut self, msg: &McpOutboundMessage) {
        if self.is_connected() {
            if let Some(ref ws) = self.ws {
                if let Ok(json) = serde_json::to_string(msg) {
                    let _ = ws.send_with_str(&json);
                }
            }
        }
    }
}

// -------------------------------------------------------------
// Native desktop implementation (tungstenite background thread)
// -------------------------------------------------------------
#[cfg(not(target_arch = "wasm32"))]
pub struct McpBridge {
    #[allow(dead_code)]
    pub url: String,
    #[allow(dead_code)]
    pub auto_connect: bool,
    pub is_connected: Arc<Mutex<bool>>,
    inbox_rx: std::sync::mpsc::Receiver<McpInboundCommand>,
    outbox_tx: std::sync::mpsc::Sender<McpOutboundMessage>,
}

#[cfg(not(target_arch = "wasm32"))]
impl McpBridge {
    pub fn new(url: &str) -> Self {
        let (in_tx, in_rx) = std::sync::mpsc::channel();
        let (out_tx, out_rx) = std::sync::mpsc::channel::<McpOutboundMessage>();
        let is_connected = Arc::new(Mutex::new(false));
        let is_connected_thread = Arc::clone(&is_connected);
        let url_thread = url.to_string();

        std::thread::spawn(move || {
            loop {
                match tungstenite::connect(&url_thread) {
                    Ok((mut socket, _)) => {
                        if let Ok(mut c) = is_connected_thread.lock() {
                            *c = true;
                        }

                        // Configure non-blocking socket
                        if let tungstenite::stream::MaybeTlsStream::Plain(ref s) = socket.get_ref() {
                            let _ = s.set_nonblocking(true);
                        }

                        loop {
                            // Try reading
                            match socket.read() {
                                Ok(tungstenite::Message::Text(txt)) => {
                                    if let Ok(cmd) = serde_json::from_str::<McpInboundCommand>(&txt) {
                                        let _ = in_tx.send(cmd);
                                    }
                                }
                                Ok(tungstenite::Message::Close(_)) => break,
                                Err(tungstenite::Error::Io(ref e))
                                    if e.kind() == std::io::ErrorKind::WouldBlock => {}
                                Err(_) => break,
                                _ => {}
                            }

                            // Try writing
                            while let Ok(msg) = out_rx.try_recv() {
                                if let Ok(json) = serde_json::to_string(&msg) {
                                    if socket.send(tungstenite::Message::Text(json)).is_err() {
                                        break;
                                    }
                                }
                            }

                            std::thread::sleep(std::time::Duration::from_millis(15));
                        }

                        if let Ok(mut c) = is_connected_thread.lock() {
                            *c = false;
                        }
                    }
                    Err(_) => {
                        std::thread::sleep(std::time::Duration::from_millis(3000));
                    }
                }
            }
        });

        Self {
            url: url.to_string(),
            auto_connect: true,
            is_connected,
            inbox_rx: in_rx,
            outbox_tx: out_tx,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.lock().map(|c| *c).unwrap_or(false)
    }

    pub fn poll_commands(&mut self) -> Vec<McpInboundCommand> {
        let mut out = Vec::new();
        while let Ok(cmd) = self.inbox_rx.try_recv() {
            out.push(cmd);
        }
        out
    }

    pub fn send(&mut self, msg: &McpOutboundMessage) {
        let _ = self.outbox_tx.send(msg.clone());
    }
}
