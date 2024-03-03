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

}
