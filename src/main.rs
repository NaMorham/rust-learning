use std::io;

fn main() {
    println!("Guss the number!\n");

    println!("Enter your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    guess = guess.trim().to_string();

    println!("You guessed {}.", guess);
}

