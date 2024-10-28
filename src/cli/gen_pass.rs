use clap::Args;

#[derive(Debug, Args)]
#[command(name = "genpass", about = "generate password")]
pub struct GenPassOpts {
    /// Length of the password
    #[arg(short, long, default_value = "16")]
    pub length: u8,

    /// Include lowercase letters
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    /// Include uppercase letters
    #[arg(short, long, default_value_t = false)]
    pub uppercase: bool,

    /// Include numbers
    #[arg(short, long, default_value_t = false)]
    pub number: bool,

    /// Include symbol characters
    #[arg(short, long, default_value_t = false)]
    pub symbol: bool,
}
