mod b64;
mod csv;
mod gen_pass;
mod http_serve;
mod text;

use crate::CmdExector;

use self::csv::CsvOpts;
pub use b64::{Base64Format, Base64SubCmd};
pub use csv::OutputFormat;
use gen_pass::GenPassOpts;
pub use http_serve::HttpServeSubCmd;
use std::path::{Path, PathBuf};
pub use text::{TextSignFormat, TextSubCmd};

use clap::{Parser, Subcommand};
/// rcli csv -i input.csv -o output.csv -d ',' --header
#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub subcmd: SubCmd,
}

/// Subcommands
#[derive(Debug, Subcommand)]
pub enum SubCmd {
    #[command(about = "csv subcommand")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Base64 encode/decode subcommand")]
    Base64(Base64SubCmd),
    #[command(subcommand, about = "Text sign/verify subcommand")]
    Text(TextSubCmd),
    #[command(subcommand, about = "http serve")]
    Http(HttpServeSubCmd),
}

impl CmdExector for SubCmd {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCmd::Csv(opts) => opts.execute().await,
            SubCmd::GenPass(opts) => opts.execute().await,
            SubCmd::Base64(subcmd) => subcmd.execute().await,
            SubCmd::Text(subcmd) => subcmd.execute().await,
            SubCmd::Http(cmd) => cmd.execute().await,
        }
    }
}

/// Verify if the file exists
fn verify_file(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

/// Verify if the direction exists
fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = PathBuf::from(path);
    if p.exists() && p.is_dir() {
        Ok(p)
    } else {
        Err("path is not exist or not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert!(verify_file("-").is_ok());
        assert!(verify_file("*").is_err());
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("not-exist").is_err());
    }
}
