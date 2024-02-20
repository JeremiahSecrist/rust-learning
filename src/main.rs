use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    

    /// Type of operation to perform
    #[arg(value_enum)]
    mode: Mode,

    /// Name of the person to greet
    // #[arg(short, long)] 
    value: String,

    // /// Number of times to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    A,
    S,
    M,
    D,
}

fn main() {
    let args = Args::parse();
    let input = args.value;
    let mut iter = input.split_whitespace();
    
    // Parse the first integer
    let num1: i32 = iter.next().unwrap().parse().unwrap();

    // Parse the second integer
    let num2: i32 = iter.next().unwrap().parse().unwrap();

    if iter.next().is_some() {
        panic!("Input contains more than two numbers");
    }

    match args.mode {
        Mode::A => {
            let answer = &num1 + &num2;
            println!("{num1} + {num2} = {answer}");
        }
        Mode::S => {
            let answer = &num1 - &num2;
            println!("{num1} - {num2} = {answer}");
        }
        Mode::M => {
            let answer = &num1 * &num2;
            println!("{num1} x {num2} = {answer}");
        }
        Mode::D => {
            let answer = &num1 / &num2;
            println!("{num1} % {num2} = {answer}");
        }
    }
}