//! # trackex — Audio Extraction & Transcoder
//!
//! `trackex` is a library (and accompanying CLI) for extracting audio
//! tracks from media files and converting them to various formats.
//!
//! ## Feature flags
//!
//! | Feature | Description                          |
//! |---------|--------------------------------------|
//! | `wav`   | WAV (PCM 16-bit) output *(default)*  |
//! | `mp3`   | MP3 output via LAME                  |
//! | `ogg`   | Ogg Vorbis output                    |
//! | `flac`  | FLAC lossless output                 |
//! | `cli`   | CLI binary with clap *(default)*     |
//! | `serde` | Serialize/Deserialize for config types|
//!
//! ## Quick example (library)
//!
//! ```no_run
//! use trackex::config::AudioConfig;
//! use trackex::format::OutputFormat;
//! use trackex::extract_audio;
//!
//! let input_data = std::fs::read("video.mp4").unwrap();
//! let cfg = AudioConfig {
//!     input_data,
//!     format: OutputFormat::Wav,
//!     sample_rate: None,
//!     channels: None,
//! };
//!
//! let wav_bytes = extract_audio(&cfg).expect("extraction failed");
//! std::fs::write("audio.wav", &wav_bytes).unwrap();
//! ```

pub mod audio;
pub mod config;
pub mod error;
pub mod format;

// Re-export the most commonly used items at the crate root.
pub use audio::extract_audio;
pub use config::AudioConfig;
pub use error::{Result, TrackExError};
