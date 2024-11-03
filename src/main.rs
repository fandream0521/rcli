use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCmd, CliOpts,
    HttpServeSubCmd, SubCmd, TextSignFormat, TextSubCmd,
};
use zxcvbn::zxcvbn;
/// rcli csv -i input.csv -o output.csv -d ',' --header
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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
            let password = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            let result = zxcvbn(&password, &[]);
            eprintln!("Password strength: {}", result.score());
            println!("{}", password);
        }
        SubCmd::Base64(subcmd) => match subcmd {
            Base64SubCmd::Encode(opts) => {
                let encoded = process_encode(&opts.input, opts.format, opts.no_padding)?;
                println!("encoded string: {}", encoded);
            }
            Base64SubCmd::Decode(opts) => {
                let decoded = process_decode(&opts.input, opts.format, opts.no_padding)?;
                println!("decoded string: {:?}", String::from_utf8_lossy(&decoded));
            }
        },
        SubCmd::Text(subcmd) => match subcmd {
            TextSubCmd::Sign(opts) => {
                let base64_sign = process_text_sign(&opts.key, &opts.input, opts.format)?;
                println!("sign: {}", base64_sign);
            }
            TextSubCmd::Verify(opts) => {
                let result = process_text_verify(&opts.key, &opts.input, &opts.sign, opts.format)?;
                if result {
                    println!("\nSignature is valid");
                } else {
                    println!("\nSignature is invalid");
                }
            }
            TextSubCmd::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                println!("Key generated");
                match opts.format {
                    TextSignFormat::Blake3 => {
                        let name = opts.output.join("blake3.txt");
                        std::fs::write(name, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = opts.output.join("ed25519.sk");
                        std::fs::write(name, &key[0])?;
                        let name = opts.output.join("ed25519.pk");
                        std::fs::write(name, &key[1])?;
                    }
                }
            }
        },
        SubCmd::Http(cmd) => {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()?;
            runtime.block_on(async move {
                match cmd {
                    HttpServeSubCmd::Serve(opts) => {
                        process_http_serve(opts.dir, opts.port).await?;
                    }
                };
                Ok::<(), anyhow::Error>(())
            })?;
        }
    }
    Ok(())
}
