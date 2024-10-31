use std::{fmt::Display, str::FromStr};

use super::verify_file;
use clap::{Args, Subcommand};
#[derive(Debug, Subcommand)]
pub enum Base64SubCmd {
    #[command(name = "encode", about = "encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Args)]
pub struct Base64EncodeOpts {
    /// Input string
    #[arg(short, long, value_name = "input", value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// Base64 format
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,

    /// is no padding
    #[arg(long)]
    pub no_padding: bool,
}

#[derive(Debug, Args)]
pub struct Base64DecodeOpts {
    /// Input base64 string
    #[arg(short, long, value_name = "input", value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// Base64 format
    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,

    /// is no padding
    #[arg(long)]
    pub no_padding: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Base64Format::Standard => write!(f, "standard"),
            Base64Format::UrlSafe => write!(f, "urlsafe"),
        }
    }
}
