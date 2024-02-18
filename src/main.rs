// use is how we import various lib, such as std.io
use std::io;
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::Duration;
// rand overs various RNG features
use rand::Rng;
fn main() {
    // In rust we must declare the type(string) and mutability if we plan to modify 
    let secret_number: u32 = rand::thread_rng().gen_range(1..10);
    loop {
        println!("Please guess a number:");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        print!("\x1B[2J\x1B[1;1H");;
        let guess_int: u32 = guess.trim().parse().expect("Please type a number!");
        // println!("You guessed: {}", guess);
        // println!("The number should have been: {}", secret_number);
        match guess_int.cmp(&secret_number) {
            Ordering::Less => print!("Too small\n"),
            Ordering::Greater => print!("Too big\n"),
            Ordering::Equal => {
                print!("Congrats you got it!\n");
                sleep(Duration::from_millis(500));
                print!("\x1B[2J\x1B[1;1H");
                break;
            },
        };
    }
}
