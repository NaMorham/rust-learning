use std::io::{self,Write};
use rand::Rng;

fn main() {

    println!("Guess the number!");
    println!("-----------------\n");

    let my_num = rand::thread_rng().gen_range(1..=100);

    #[cfg(debug_assertions)]
    eprintln!("DBG: my_num: {}", my_num);

    print!("Enter your guess: ");
    io::stdout().flush().unwrap();
        // match io::stdin().read_line(&mut guess) {
        //     Ok(s) => {
        //         guess_now = match guess.strip_suffix("\n") {
        //             Some(g) => {
        //                 println!("got some");
        //                 g.to_string()
        //             },
        //             None => {
        //                 println!("Got none");
        //                 guess
        //             },
        //         };
        //         println!("Raw guess: {}, {}", guess_now, s);
        //     },
        //     Err(e) => { 
        //         println!("Failed to read line {e:?}");
        //         std::process::exit(1)
        //     },
        // }

    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(s) => {
            eprintln!("Received {} bytes", s);
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
        },
        Err(e) => {
            eprintln!("Failed to read input: {}", e);
            std::process::exit(1)
        }
    }; // end match stdin
}
