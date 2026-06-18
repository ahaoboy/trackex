# trackex

Extract audio tracks from video files and convert to various formats.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Installation

```sh
cargo install trackex
```

## Usage

```sh
# Extract audio from a video file
trackex video.mp4 audio.wav
trackex concert.mkv music.mp3
trackex movie.mov soundtrack.ogg
trackex recording.webm speech.flac
```

The output format is auto-detected from the output file extension.

### Options

```
trackex [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Path to the input media file (video or audio)
  <OUTPUT>  Path for the output audio file (format inferred from extension)

Options:
  -r, --sample-rate <HZ>  Target sample rate in Hz (default: preserve original)
  -c, --channels <N>      Target channel count (default: preserve original)
  -h, --help              Print help
  -V, --version           Print version
```

## Supported formats

| Format | Extension | Feature flag | Backend |
|--------|-----------|-------------|---------|
| WAV    | `.wav`    | `wav`       | Manual (PCM 16-bit) |
| MP3    | `.mp3`    | `mp3`       | [OxiAudio](https://crates.io/crates/oxiaudio) (LAME) |
| Ogg Vorbis | `.ogg` | `ogg`    | [vorbis-encoder](https://crates.io/crates/vorbis-encoder) |
| FLAC   | `.flac`   | `flac`      | [oxideav-flac](https://crates.io/crates/oxideav-flac) |

## Feature flags

| Feature | Default | Description |
|---------|---------|-------------|
| `wav`   | yes     | WAV (PCM 16-bit) output |
| `mp3`   | yes     | MP3 output via LAME |
| `ogg`   | yes     | Ogg Vorbis output |
| `flac`  | yes     | FLAC lossless output |
| `cli`   | yes     | CLI binary (clap) |
| `serde` | yes     | Serialize/Deserialize for config types |

Build with only specific formats:

```sh
cargo build --no-default-features --features wav,cli
```

## Library usage

```rust
use trackex::config::AudioConfig;
use trackex::format::OutputFormat;
use trackex::extract_audio;

let input_data = std::fs::read("video.mp4")?;
let cfg = AudioConfig {
    input_data,
    format: OutputFormat::Wav,
    sample_rate: None,
    channels: None,
};

let wav_bytes = extract_audio(&cfg)?;
std::fs::write("audio.wav", &wav_bytes)?;
```

## License

MIT
