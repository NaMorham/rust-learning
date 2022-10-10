use std::io::{self,Write};

fn main() {
    println!("Guss the number!\n");

    print!("Enter your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    guess = guess.trim().to_string();

    println!("You guessed {}.", guess);
}

