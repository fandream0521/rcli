use std::path::PathBuf;

use clap::{command, Args, Subcommand};

use crate::{process_http_serve, CmdExector};

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

impl CmdExector for HttpServeSubCmd {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpServeSubCmd::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        };
        Ok(())
    }
}
