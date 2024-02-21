use clap::Parser;
use shunting::*;
use hsh::models::hash::Hash;
use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Simple program to solve equations
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// string of equation
    value: String,
}

fn main() {
    let args = Args::parse();
    let input = args.value;
    let expr = ShuntingParser::parse_str(&input).unwrap();
    let result = MathContext::new().eval(&expr).unwrap();
    let padded_result: String =  format!("AAAAAAAAA{}", result.clone().to_string());
    let hash = Hash::new(&padded_result, padded_result.as_str(), "argon2i").expect("Failed to create hash");
    println!("{} {} = {}", STANDARD.encode(&hash.hash), &input, result);
}