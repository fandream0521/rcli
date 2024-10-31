use anyhow::Result;
use std::fs;
use std::io::Read;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    Ok(if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(fs::File::open(input)?)
    })
}
