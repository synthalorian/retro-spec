use clap::Parser;
use std::path::PathBuf;

/// Walk your code as a 3D neon city — every commit is a building, every branch is a boulevard.
#[derive(Parser, Debug)]
#[command(name = "retro-spec", version, about)]
pub struct Cli {
    /// Path to the git repository to visualize
    #[arg(short = 'r', long = "repo", default_value = ".")]
    pub repo: PathBuf,

    /// Visual theme (synthwave84, matrix, chrome)
    #[arg(short = 't', long = "theme", default_value = "synthwave84")]
    pub theme: String,

    /// Scrub to a specific point in history (ISO 8601 date)
    #[arg(long = "at")]
    pub at_time: Option<String>,

    /// Export flythrough video to this path
    #[arg(long = "export")]
    pub export: Option<PathBuf>,

    /// Duration of flythrough video in seconds
    #[arg(long = "duration", default_value = "30")]
    pub duration: f32,

    /// Export a still screenshot to this path
    #[arg(long = "screenshot")]
    pub screenshot: Option<PathBuf>,

    /// Screenshot resolution (width x height)
    #[arg(long = "resolution", default_value = "3840x2160")]
    pub resolution: String,

    /// Print repo stats and exit (no 3D view)
    #[arg(long = "stats")]
    pub stats: bool,

    /// Start in windowed mode (don't fullscreen)
    #[arg(long = "windowed")]
    pub windowed: bool,
}
