use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_gen_pass, CliOpts, SubCmd};
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
    }
    Ok(())
}
