use clap::{Args, Parser};

/// Produce a Zipfian word count from any text input
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Verbose output
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// The path to a file to read
    #[arg(short, long)]
    pub path: Option<std::path::PathBuf>,

    /// Max number of rank ordered word counts
    #[arg(short, long, default_value_t = 20)]
    pub number: usize,
}
