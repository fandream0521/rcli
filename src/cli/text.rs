use std::{fmt::Display, path::PathBuf, str::FromStr};

use super::{verify_file, verify_path};
use clap::{Args, Subcommand};
#[derive(Debug, Subcommand)]
pub enum TextSubCmd {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextGenerateOpts),
}

#[derive(Debug, Args)]
pub struct TextSignOpts {
    /// Input string
    #[arg(short, long,  value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// key to sign with
    #[arg(short, long,  value_parser = verify_file)]
    pub key: String,

    /// format of signature
    #[arg(short, long,  default_value = "blake3", value_parser = TextSignFormat::from_str)]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextVerifyOpts {
    /// Input base64 string
    #[arg(short, long, value_name = "input", value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// key to verify with
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    /// key to verify with
    #[arg(short, long)]
    pub sign: String,

    /// format of signature
    #[arg(short, long, default_value = "blake3", value_parser = TextSignFormat::from_str)]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextGenerateOpts {
    /// format of signature
    #[arg(short, long, default_value = "blake3", value_parser = TextSignFormat::from_str)]
    pub format: TextSignFormat,

    /// Output directory
    #[arg(short, long, value_parser = verify_path, default_value = "fixtures")]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blake3 => write!(f, "blake3"),
            Self::Ed25519 => write!(f, "ed25519"),
        }
    }
}
