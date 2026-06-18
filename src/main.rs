//! Trackex CLI — extract audio tracks from video files.
//!
//! Requires the `cli` cargo feature (enabled by default).
//!
//! ```text
//! trackex video.mp4 audio.wav
//! trackex concert.mkv music.mp3
//! trackex movie.mov soundtrack.ogg
//! ```

#[cfg(feature = "cli")]
mod cli {
    use std::path::PathBuf;

    use clap::Parser;

    use trackex::config::AudioConfig;
    use trackex::format::OutputFormat;

    /// Extract audio tracks from video files and convert to various formats.
    ///
    /// The output format is auto-detected from the output file extension.
    #[derive(Parser, Debug)]
    #[command(name = "trackex", version, about, long_about = None)]
    struct Cli {
        /// Path to the input media file (video or audio)
        #[arg(value_name = "INPUT")]
        input: PathBuf,

        /// Path for the output audio file (format inferred from extension)
        #[arg(value_name = "OUTPUT")]
        output: PathBuf,

        /// Target sample rate in Hz (default: preserve original)
        #[arg(short = 'r', long = "sample-rate", value_name = "HZ")]
        sample_rate: Option<u32>,

        /// Target channel count (default: preserve original)
        #[arg(short = 'c', long = "channels", value_name = "N")]
        channels: Option<u16>,
    }

    pub fn run() {
        let cli = Cli::parse();

        // Auto-detect output format from the file extension.
        let format = match cli.output.extension().and_then(|s| s.to_str()) {
            Some(ext) => match OutputFormat::from_extension(ext) {
                Ok(fmt) => fmt,
                Err(e) => {
                    eprintln!("ERROR: {e}");
                    std::process::exit(1);
                }
            },
            None => {
                eprintln!(
                    "ERROR: output file '{}' has no extension — cannot infer format",
                    cli.output.display()
                );
                std::process::exit(1);
            }
        };

        // Read input file (app layer I/O).
        let input_data = match std::fs::read(&cli.input) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("ERROR: failed to read '{}': {e}", cli.input.display());
                std::process::exit(1);
            }
        };

        let input_size = humansize::format_size(input_data.len(), humansize::BINARY);

        let config = AudioConfig {
            input_data,
            format,
            sample_rate: cli.sample_rate,
            channels: cli.channels,
        };

        // Transcode (library call).
        eprintln!(
            "Extracting audio from {} ({input_size}) ...",
            cli.input.display()
        );
        let output_data = match trackex::extract_audio(&config) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("ERROR: {e}");
                std::process::exit(1);
            }
        };

        // Write output file (app layer I/O).
        if let Err(e) = std::fs::write(&cli.output, &output_data) {
            eprintln!("ERROR: failed to write '{}': {e}", cli.output.display());
            std::process::exit(1);
        }

        let output_size = humansize::format_size(output_data.len(), humansize::BINARY);
        println!(
            "Successfully wrote {} ({output_size})",
            cli.output.display()
        );
    }
}

fn main() {
    #[cfg(feature = "cli")]
    cli::run();

    #[cfg(not(feature = "cli"))]
    {
        eprintln!("The `cli` feature is disabled. Rebuild with `--features cli`.");
        std::process::exit(1);
    }
}
