use std::path::PathBuf;

use clap::{command, Args, Subcommand};

use super::verify_path;

#[derive(Debug, Subcommand)]
pub enum HttpServeSubCmd {
    #[command(about = "serve http")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Args)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = ".", value_parser = verify_path)]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
