use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, Base64SubCmd, CliOpts, SubCmd,
};
/// rcli csv -i input.csv -o output.csv -d ',' --header
fn main() -> Result<()> {
    let cli = CliOpts::parse();
    match cli.subcmd {
        SubCmd::Csv(opts) => {
            let output = match opts.output {
                Some(output) => output,
                None => format!("output.{}", opts.format),
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCmd::GenPass(opts) => {
            process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCmd::Base64(subcmd) => match subcmd {
            Base64SubCmd::Encode(opts) => {
                process_encode(&opts.input, opts.format, opts.no_padding)?;
            }
            Base64SubCmd::Decode(opts) => {
                process_decode(&opts.input, opts.format, opts.no_padding)?;
            }
        },
    }
    Ok(())
}
