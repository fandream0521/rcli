use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::prelude::*;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::path::Path;
use std::{fs, io::Read};

use super::process_gen_pass;

pub trait TextSigner {
    /// Sign the content of the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerifier {
    /// Verify the content of the reader with the signature
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)?;
        Ok(blake3::keyed_hash(&self.key, &buff).as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)?;
        let hashed_key = blake3::keyed_hash(&self.key, &buff);
        let hash = hashed_key.as_bytes();
        Ok(hash == sign)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_from(&key[..32])
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_gen_pass(32, true, true, true, true)?;
        Ok(vec![key.into_bytes()])
    }
}

#[derive(Debug)]
pub struct Ed25519Signer {
    key: SigningKey,
}
#[derive(Debug)]
pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)?;
        Ok(self.key.sign(&buff).to_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_from(&key[..])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Ok(vec![
            signing_key.to_bytes().to_vec(),
            verifying_key.to_bytes().to_vec(),
        ])
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool> {
        let mut buff = Vec::new();
        reader.read_to_end(&mut buff)?;
        let signature = ed25519_dalek::Signature::from_bytes(sign.try_into()?);
        Ok(self.key.verify(&buff, &signature).is_ok())
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_from(&key[..])
    }
}

pub fn process_text_sign(key: &str, input: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let sign = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let base64_sign = BASE64_URL_SAFE_NO_PAD.encode(sign);
    Ok(base64_sign)
}

pub fn process_text_verify(
    key: &str,
    input: &str,
    sign: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sign = BASE64_URL_SAFE_NO_PAD.decode(sign)?;
    let result = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
    };
    Ok(result)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl Blake3 {
    fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
}

impl TryFrom<&[u8]> for Blake3 {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() != 32 {
            anyhow::bail!("Invalid key length");
        }
        let key = value.try_into()?;
        Ok(Self::new(key))
    }
}

impl Ed25519Signer {
    fn new(key: SigningKey) -> Self {
        Self { key }
    }
}

impl TryFrom<&[u8]> for Ed25519Signer {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(value.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
}

impl TryFrom<&[u8]> for Ed25519Verifier {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(value.try_into()?)?;
        Ok(Self::new(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text_sign_blake3() -> Result<()> {
        let input = "fixtures/input.txt";
        let key = "fixtures/blake3.txt";
        let sign = process_text_sign(key, input, TextSignFormat::Blake3)?;

        let val = process_text_verify(key, input, &sign, TextSignFormat::Blake3)?;
        assert!(val);
        Ok(())
    }

    #[test]
    fn test_process_text_sign_ed25519() -> Result<()> {
        let input = "fixtures/input.txt";
        let sk = "fixtures/ed25519.sk";
        let sign = process_text_sign(sk, input, TextSignFormat::Ed25519)?;

        let pk = "fixtures/ed25519.pk";
        let val = process_text_verify(pk, input, &sign, TextSignFormat::Ed25519)?;
        assert!(val);
        Ok(())
    }
}
