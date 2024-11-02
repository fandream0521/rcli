use anyhow::Result;
use rand::{seq::SliceRandom, Rng};
const UPPER: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String> {
    if length < 4 {
        return Err(anyhow::anyhow!("Length must be greater than 4"));
    }

    let mut password = Vec::new();
    let mut char_set = Vec::new();
    let mut rng = rand::thread_rng();
    if upper {
        char_set.extend_from_slice(UPPER);
        password.push(UPPER[rng.gen_range(0..UPPER.len())]);
    }
    if lower {
        char_set.extend_from_slice(LOWER);
        password.push(LOWER[rng.gen_range(0..LOWER.len())]);
    }
    if number {
        char_set.extend_from_slice(NUMBER);
        password.push(NUMBER[rng.gen_range(0..NUMBER.len())]);
    }
    if symbol {
        char_set.extend_from_slice(SYMBOL);
        password.push(SYMBOL[rng.gen_range(0..SYMBOL.len())]);
    }

    for _ in 0..(length as usize - password.len()) {
        password.push(char_set[rng.gen_range(0..char_set.len())]);
    }

    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
