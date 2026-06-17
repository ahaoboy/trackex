//! Error types for the trackex library.
//!
//! All errors that can occur during audio extraction and encoding
//! are represented by the [`TrackExError`] enum.

use thiserror::Error;

/// The main error type for all trackex operations.
///
/// Covers IO errors, media format errors, encoding errors,
/// and configuration/validation issues.
#[derive(Debug, Error)]
pub enum TrackExError {
    /// Wraps a standard IO error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps a Symphonia format/demux error.
    #[error("media format error: {0}")]
    Format(#[from] symphonia::core::errors::Error),

    /// Wraps a WAV (hound) encoding error.
    #[cfg(feature = "wav")]
    #[error("WAV encoding error: {0}")]
    Wav(#[from] hound::Error),

    /// The media file contains no audio track.
    #[error("no audio track found in the media file")]
    NoAudioTrack,

    /// The audio track is missing required codec parameters.
    #[error("missing codec parameters for audio track")]
    MissingCodecParams,

    /// The codec parameters exist but lack audio-specific info
    /// (sample rate, channel layout, etc.).
    #[error("audio stream parameters not available (sample rate / channels)")]
    MissingAudioParams,

    /// Thrown when a requested output format is not compiled in
    /// (missing cargo feature flag) or is otherwise unsupported.
    #[error("unsupported output format: {0}")]
    UnsupportedFormat(String),

    /// The output file extension is not a recognised audio format.
    #[error("unknown output extension '.{0}' — expected wav, mp3, or ogg")]
    UnknownExtension(String),

    /// Generic audio encoding error from format-specific backends.
    #[error("audio encoding error: {0}")]
    Encoding(String),
}

/// Convenience alias for `Result<T, TrackExError>`.
pub type Result<T> = std::result::Result<T, TrackExError>;
