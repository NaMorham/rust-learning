use std::io::{self,Write};
use ansi_term::Colour::RGB;
//use ansi_term::Style;

fn main() {
	let err = RGB(248, 24, 24).normal();
	let warn = RGB(230, 126, 34).normal();
	let info = RGB(240, 240, 24).normal();
	let debug = RGB(24, 192, 24).italic();
	let header = RGB(255, 255, 255).bold();

	println!("{}", err.paint("Test 1 "));
	println!("{}", warn.paint("Test 2 "));
	println!("{}", info.paint("Test 3 "));
	println!("{}", debug.paint("Test 4 "));
	println!();

    println!("{}", header.paint("Guess the number!"));
    println!("-----------------\n");

    print!("Enter your guess: ");
    io::stdout().flush().unwrap();

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    guess = guess.trim().to_string();

    println!("You guessed {}.", guess);
}

