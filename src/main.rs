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

    let guess_str = guess.trim().to_string();

    println!("You guessed {}.", guess_str);
    if guess_str.eq_ignore_ascii_case("quit") {
        println!("{}", header.paint("OUT!"));
    }
    else
    {
        print!("{}", err.paint("TODO: "));
        println!("Actually compare the guess");
        
        let guess_num: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => 9999,
        };
        println!("{}", err.paint("Just what the fuck are you trying to do?"));
        println!("{}{}", debug.paint("guess is "), guess_num);
    }
}
