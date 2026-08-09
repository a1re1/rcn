use anyhow::Result;
use clap::Parser;
use rcn_cli::{Cli, run};

fn main() -> Result<()> {
    // Die quietly on SIGPIPE (e.g. `rcn list | head`) instead of panicking
    // when stdout closes mid-write.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }
    let cli = Cli::parse();
    run(cli)
}
