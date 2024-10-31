use anyhow::Result;
use base64::prelude::*;
use std::io::Read;

use crate::{get_reader, Base64Format};

pub fn process_encode(input: &str, format: Base64Format, no_padding: bool) -> Result<String> {
    let mut reader = get_reader(input)?;
    let mut buff = Vec::new();
    reader.read_to_end(&mut buff)?;
    let encoded = match format {
        Base64Format::Standard => {
            if no_padding {
                BASE64_STANDARD_NO_PAD.encode(buff)
            } else {
                BASE64_STANDARD.encode(buff)
            }
        }
        Base64Format::UrlSafe => {
            if no_padding {
                BASE64_URL_SAFE_NO_PAD.encode(buff)
            } else {
                BASE64_URL_SAFE.encode(buff)
            }
        }
    };
    println!("encoded string: {}", encoded);
    Ok(encoded)
}

pub fn process_decode(input: &str, format: Base64Format, no_padding: bool) -> Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let mut buff = String::new();
    reader.read_to_string(&mut buff)?;
    let buff = buff.trim();
    let decoded = match format {
        Base64Format::Standard => {
            if no_padding {
                BASE64_STANDARD_NO_PAD.decode(buff)?
            } else {
                BASE64_STANDARD.decode(buff)?
            }
        }
        Base64Format::UrlSafe => {
            if no_padding {
                BASE64_URL_SAFE_NO_PAD.decode(buff)?
            } else {
                BASE64_URL_SAFE.decode(buff)?
            }
        }
    };

    println!("decoded string: {:?}", String::from_utf8_lossy(&decoded));
    Ok(decoded)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_encode_standard() {
        let input = "fixtures/encode.txt";
        let format = Base64Format::Standard;
        let no_padding = false;
        let encoded = process_encode(input, format, no_padding).unwrap();
        assert_eq!("aGVsbG8sd29ybGQK", encoded);
        let output = "fixtures/decode.txt";
        let decoded = process_decode(output, format, no_padding).unwrap();
        assert_eq!("hello,world", String::from_utf8_lossy(&decoded));
    }
}
