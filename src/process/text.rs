use crate::{get_reader, TextSignFormat};
use anyhow::Result;
use base64::prelude::*;
use base64::Engine;
use std::{fs, io::Read};

trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sign: &[u8]) -> Result<bool>;
}

struct Blake3 {
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

pub fn process_text_sign(key: &str, input: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let sign = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let signer = Blake3::try_from(&key[..32])?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let base64_sign = BASE64_URL_SAFE_NO_PAD.encode(sign);
    println!("sign: {}", base64_sign);
    Ok(base64_sign)
}

pub fn process_text_verify(
    key: &str,
    input: &str,
    sign: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sign = fs::read(sign)?;
    let sign = BASE64_URL_SAFE_NO_PAD.decode(sign)?;
    let result = match format {
        TextSignFormat::Blake3 => {
            let key = fs::read(key)?;
            let verifier = Blake3::try_from(&key[..32])?;
            verifier.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    Ok(result)
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
