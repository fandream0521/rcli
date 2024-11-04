use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

use crate::{
    process_text_decrypt, process_text_encrypt, process_text_generate, process_text_sign,
    process_text_verify, CmdExector,
};

use super::{verify_file, verify_path};
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;
#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExector)]
pub enum TextSubCmd {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key")]
    Generate(TextGenerateOpts),
    #[command(about = "Encrypt a message with a private/shared key")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message with a private/shared key")]
    Decrypt(TextDecryptOpts),
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

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let base64_sign = process_text_sign(&self.key, &self.input, self.format)?;
        println!("sign: {}", base64_sign);
        Ok(())
    }
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

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = process_text_verify(&self.key, &self.input, &self.sign, self.format)?;
        if result {
            println!("\nSignature is valid");
        } else {
            println!("\nSignature is invalid");
        }
        Ok(())
    }
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

impl CmdExector for TextGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_generate(self.format)?;
        println!("Key generated");
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.txt");
                std::fs::write(name, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let name = self.output.join("ed25519.sk");
                std::fs::write(name, &key[0])?;
                let name = self.output.join("ed25519.pk");
                std::fs::write(name, &key[1])?;
            }
            TextSignFormat::ChaCha20Poly1305 => {
                let name = self.output.join("chacha20poly1305.key");
                std::fs::write(name, &key[0])?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TextEncryptOpts {
    /// Input string
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// key to sign with
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    /// format of signature
    #[arg(short, long, default_value = "chacha20poly1305", value_parser = TextEncryptFormat::from_str)]
    pub format: TextEncryptFormat,

    /// Output file
    #[arg(short, long, default_value = "fixtures/encrypted.txt")]
    pub output: String,
}

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let base64_encrypt = process_text_encrypt(&self.key, &self.input, self.format)?;
        println!("base64_encrypt: {:?}", base64_encrypt);
        fs::write(&self.output, base64_encrypt)?;
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct TextDecryptOpts {
    /// Input base64 string
    #[arg(short, long, value_name = "input", value_parser = verify_file, default_value = "-")]
    pub input: String,

    /// key to verify with
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,

    /// format of signature
    #[arg(short, long, default_value = "chacha20poly1305", value_parser = TextEncryptFormat::from_str)]
    pub format: TextEncryptFormat,

    /// Output file
    #[arg(short, long, default_value = "fixtures/decrypted.txt")]
    pub output: String,
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = process_text_decrypt(&self.key, &self.input, self.format)?;
        println!("decrypted: {:?}", String::from_utf8_lossy(&decrypted));
        fs::write(&self.output, decrypted)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    ChaCha20Poly1305,
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(Self::Blake3),
            "ed25519" => Ok(Self::Ed25519),
            "chacha20poly1305" => Ok(Self::ChaCha20Poly1305),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blake3 => write!(f, "blake3"),
            Self::Ed25519 => write!(f, "ed25519"),
            Self::ChaCha20Poly1305 => write!(f, "chacha20poly1305"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextEncryptFormat {
    ChaCha20Poly1305,
}

impl FromStr for TextEncryptFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "chacha20poly1305" => Ok(Self::ChaCha20Poly1305),
            _ => Err(format!("Invalid format: {}", s)),
        }
    }
}

impl Display for TextEncryptFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ChaCha20Poly1305 => write!(f, "chacha20poly1305"),
        }
    }
}
