use std::{fmt::Display, str::FromStr};

use clap::Args;

use crate::{process_csv, CmdExector};

use super::verify_file;

#[derive(Debug, Args)]
#[command(name = "csv", about = "csv subcommand")]
pub struct CsvOpts {
    /// Input CSV file
    #[arg(short, long, value_name = "input", value_parser = verify_file)]
    pub input: String,

    /// Output JSON file
    #[arg(short, long, value_name = "output", value_parser = verify_output_format)]
    pub output: Option<String>,

    /// Delimiter
    #[arg(short, long, value_name = "delimiter", default_value_t = ',')]
    pub delimiter: char,

    /// Has header or not
    #[arg(long, default_value_t = true)]
    pub header: bool,

    /// format for output
    #[arg(long, default_value = "json", value_parser = OutputFormat::from_str)]
    pub format: OutputFormat,
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = match self.output {
            Some(output) => output,
            None => format!("output.{}", self.format),
        };
        process_csv(&self.input, &output, self.format)
    }
}

/// Output format
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn verify_output_format(output: &str) -> Result<String, String> {
    let format = output.to_lowercase();
    let Some(last_dot_index) = format.rfind('.') else {
        return Err(format!("Invalid format: {}", output));
    };
    let ext = &format[last_dot_index + 1..];
    let _: OutputFormat = ext.parse()?;
    Ok(output.to_string())
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let format = s.to_lowercase();
        match format.as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
