mod b64;
mod csv;
mod gen_pass;

use self::csv::CsvOpts;
pub use b64::{Base64Format, Base64SubCmd};
pub use csv::OutputFormat;
use gen_pass::GenPassOpts;
use std::path::Path;

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
    #[command(name = "csv", about = "csv subcommand")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCmd),
}

/// Verify if the file exists
fn verify_file_exists(filename: &str) -> Result<String, String> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("File not found: {}", filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert!(verify_file_exists("-").is_ok());
        assert!(verify_file_exists("*").is_err());
        assert!(verify_file_exists("Cargo.toml").is_ok());
        assert!(verify_file_exists("not-exist").is_err());
    }
}
