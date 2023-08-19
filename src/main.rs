use std::io::{self,Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!("Guess the number!");
    println!("-----------------\n");

    let min_val :u32 = 1;
    let max_val :u32 = 100;
    let my_num = rand::thread_rng().gen_range(min_val..=max_val);

    #[cfg(debug_assertions)]
    eprintln!("DBG: my_num: {}", my_num);

    loop {
        print!("Enter your guess (Between {} and {}: ", min_val, max_val);
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_s) => {
                #[cfg(debug_assertions)]
                eprintln!("DBG: Received {} bytes", _s);
                let guess_str = guess.trim().to_string();

                #[cfg(debug_assertions)]
                eprintln!("DBG: guess_str {}", guess_str);

                if guess_str.eq_ignore_ascii_case("quit") {
                    #[cfg(debug_assertions)]
                    eprintln!("DBG: OUT!");

                    println!("Goodbye");
                    break;
                }
                else {
                    let guess_num: u32 = match guess_str.parse() {
                        Ok(num) => {
                            #[cfg(debug_assertions)]
                            eprintln!("DBG: Number is {}", num);
                            num
                        },
                        Err(e) => {
                            eprintln!("Could not parse input: {}", e);
                            continue;
                        },
                    };
                    #[cfg(debug_assertions)]
                    eprintln!("guess is {}", guess_num);
                    if guess_num < min_val {
                        println!("Your guess is below the minimum {}.  Try again", min_val);
                        continue;
                    }
                    else if guess_num > max_val {
                        println!("Your guess is above the maximum {}.  Try again", max_val);
                        continue;
                    }
                    else {
                        match guess_num.cmp(&my_num) {
                            Ordering::Less => {
                                println!("Your guess is lower, try again");
                                continue;
                            },
                            Ordering::Greater => {
                                println!("Your guess is higher, try again");
                                continue;
                            },
                            Ordering::Equal => {
                                println!("Your guess is equal, congratulations");
                                break;
                            },
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                std::process::exit(1)
            }
        }; // end match stdin
    } // end main loop
}
