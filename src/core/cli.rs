use crate::core::types::CryptoPair;
use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Base cryptocurrency (e.g., BTC)
    #[arg(short, long)]
    pub base: Option<String>,

    /// Quote cryptocurrency (e.g., USDT)
    #[arg(short, long)]
    pub quote: Option<String>,
}

pub fn get_args() -> Result<Args> {
    let mut args = Args::parse();

    if args.base.is_none() {
        args.base = Some(prompt("Enter the base cryptocurrency (e.g., BTC): ")?);
    }

    if args.quote.is_none() {
        args.quote = Some(prompt("Enter the quote cryptocurrency (e.g., USDT): ")?);
    }

    Ok(args)
}

fn prompt(message: &str) -> Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

pub fn get_crypto_pair(base: &str, quote: &str) -> CryptoPair {
    CryptoPair::new(base, quote)
}
