use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "prq",
    version,
    about = "A terminal UI for browsing open PRs and their review/check status."
)]
pub struct Cli {
    /// Auto-refresh interval in seconds.
    #[arg(long, default_value_t = 60)]
    pub refresh_interval: u64,

    /// Disable periodic auto-refresh. Only `r` will refresh.
    #[arg(long, default_value_t = false)]
    pub no_auto_refresh: bool,

    /// Maximum number of PRs to fetch.
    #[arg(long, default_value_t = 100)]
    pub limit: u32,
}
