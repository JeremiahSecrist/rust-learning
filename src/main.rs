use clap::{Parser, ValueEnum};
use shunting::*;
/// Simple program to greet a person
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
    println!("{} = {}", &input, result);
}