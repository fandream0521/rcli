use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, CliOpts, SubCmd};
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
    }
    Ok(())
}
