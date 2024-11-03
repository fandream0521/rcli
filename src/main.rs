use anyhow::Result;
use clap::Parser;
use rcli::{CliOpts, CmdExector, SubCmd};

/// rcli csv -i input.csv -o output.csv -d ',' --header
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = CliOpts::parse();
    match cli.subcmd {
        SubCmd::Csv(opts) => opts.execute().await,
        SubCmd::GenPass(opts) => opts.execute().await,
        SubCmd::Base64(subcmd) => subcmd.execute().await,
        SubCmd::Text(subcmd) => subcmd.execute().await,
        SubCmd::Http(cmd) => cmd.execute().await,
    }
}
