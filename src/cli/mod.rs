mod csv;
mod gen_pass;

pub use csv::*;
pub use gen_pass::*;

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
}
