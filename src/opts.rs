use std::path::Path;

use clap::{Args, Parser, Subcommand};
/// rcli csv -i input.csv -o output.csv -d ',' --header
#[derive(Debug, Parser)]
#[command(name = "rcli", version, about, long_about = None)]
pub struct CliOpts {
    #[command(subcommand)]
    pub subcmd: SubCmd,
}

#[derive(Debug, Subcommand)]
pub enum SubCmd {
    #[command(name = "csv", about = "csv subcommand")]
    Csv(CsvOpts),
}

#[derive(Debug, Args)]
#[command(name = "csv", about = "csv subcommand")]
pub struct CsvOpts {
    #[arg(short, long, value_name = "input", value_parser = verify_file_exists)]
    pub input: String,

    #[arg(short, long, value_name = "output", default_value = "output.json")]
    pub output: String,

    #[arg(short, long, value_name = "delimiter", default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
