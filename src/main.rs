mod decoder;

use crate::decoder::{decode_list, decode_number, decode_string};
use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use serde_json;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Decode {
        value: String
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() -> anyhow::Result<()>{
    let cli = Cli::parse();

    match cli.cmd {
        Cmd::Decode{ value: encoded_value } => {
            let decoded_values = decode_bencoded_value(&encoded_value)
                .context("could not decode value")?;
            for decoded_values in decoded_values {
                println!("{}", decoded_values)
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> anyhow::Result<Vec<serde_json::Value>> {
    let mut encoded_chars = encoded_value.chars().peekable();
    // If encoded_value starts with a digit, it's a number
    let Some(first_char) = encoded_chars.peek() else {
        bail!("Unhandled encoded value: {}", encoded_value)
    };

    if first_char.is_digit(10) {
        decode_string(encoded_value)
    } else if first_char.eq_ignore_ascii_case(&'i')  {
        decode_number(encoded_value)
    } else if first_char.eq_ignore_ascii_case(&'l')  {
        decode_list(encoded_value)
    } else {
        bail!("Unhandled encoded value: {}", encoded_value)
    }
}