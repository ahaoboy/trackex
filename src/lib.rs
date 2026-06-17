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
//! | `serde` | Serialize/Deserialize for config types|
//!
//! ## Quick example (library)
//!
//! ```no_run
//! use std::path::PathBuf;
//! use trackex::config::AudioConfig;
//! use trackex::format::OutputFormat;
//! use trackex::extract_audio;
//!
//! let cfg = AudioConfig {
//!     input: PathBuf::from("video.mp4"),
//!     output: PathBuf::from("audio.wav"),
//!     format: OutputFormat::Wav,
//!     sample_rate: None,
//!     channels: None,
//! };
//!
//! extract_audio(&cfg).expect("extraction failed");
//! ```

pub mod audio;
pub mod config;
pub mod error;
pub mod format;

// Re-export the most commonly used items at the crate root.
pub use audio::extract_audio;
pub use config::AudioConfig;
pub use error::{Result, TrackExError};
