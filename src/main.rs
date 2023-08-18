use std::io::{self,Write};
use rand::Rng;

fn main() {

    println!("Guess the number!");
    println!("-----------------\n");

    let my_num = rand::thread_rng().gen_range(1..=100);
    eprintln!("DBG: my_num: {}", my_num);

    print!("Enter your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess_str = guess.trim().to_string();

    println!("You guessed {}.", guess_str);
    if guess_str.eq_ignore_ascii_case("quit") {
        eprintln!("OUT!");
    }
    else
    {
        eprint!("TODO: ");
        eprintln!("Actually compare the guess");
        
        let guess_num: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => 9999,
        };
        eprintln!("Just what the fuck are you trying to do?");
        eprintln!("guess is {}", guess_num);
    }
}
