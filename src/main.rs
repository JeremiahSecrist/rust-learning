// use is how we import various lib, such as std.io
use std::io;
fn main() {
    println!("Please guess a number:");
    // In rust we must declare the type(string) and mutability if we plan to modify 
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed: {}", guess);
}
