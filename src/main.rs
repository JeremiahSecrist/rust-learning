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
    let expr = ShuntingParser::parse_str(&input)
        .expect("Failed to shunt value");
    let result = MathContext::new()
        .eval(&expr)
        .expect("Failed to calculate shunted value");
    println!("{} = {}", &input, result);
}