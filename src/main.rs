use clap::Parser;
use shunting::*;

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

    let expr = match ShuntingParser::parse_str(&input) {
        Ok(expr) => expr,
        Err(err) => {
            eprintln!("Failed to shunt value: {}", err);
            std::process::exit(1);
        }
    };

    let result = match MathContext::new().eval(&expr) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Failed to calculate shunted value: {}", err);
            std::process::exit(1);
        }
    };

    println!("{} = {}", &input, result);
}
