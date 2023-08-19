use std::io::{self,Write};
use rand::Rng;
use std::cmp::Ordering;

fn number_suffix(number : u64) -> String {
    let num_mod_100 = number % 100;

    if num_mod_100 < 10 || num_mod_100 >= 20 {
        let num_mod_10 = num_mod_100 % 10;

        if num_mod_10 == 1 {
            format!("{}st", number)
        }
        else if num_mod_10 == 2 {
            format!("{}nd", number)
        }
        else if num_mod_10 == 3 {
            format!("{}rd", number)
        }
        else {
            format!("{}th", number)
        }
    }
    else {
        format!("{}th", number)
    }
}

fn main() {
    let min_val = 1;
    let max_val = 100;
    let max_count = 5;
    let my_num = rand::thread_rng().gen_range(min_val..=max_val);

    let mut count = 1;

    println!("Guess the number, you have {} guesses", max_count);
    println!("-----------------");

    #[cfg(debug_assertions)]
    eprintln!("DBG: my_num: {}", my_num);

    loop {
        if count > max_count {
            println!("You have taken too many guesses.  Goodbye");
            break;
        }
        else {
            println!("");
        }

        print!("Enter your {} guess (Between {} and {}): ", number_suffix(count), min_val, max_val);
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
                    eprintln!("DBG: guess is {}", guess_num);
                    if guess_num < min_val {
                        println!("Your guess is below the minimum {}.  Try again", min_val);
                        continue;
                    }
                    else if guess_num > max_val {
                        println!("Your guess is above the maximum {}.  Try again", max_val);
                        continue;
                    }
                    else {
                        count += 1;
                        match guess_num.cmp(&my_num) {
                            Ordering::Less => {
                                print!("");
                                if count <= max_count {
                                    println!("Your guess is lower, try again.  You have {} guesses remaining", max_count - count + 1);
                                }
                                continue;
                            },
                            Ordering::Greater => {
                                if count <= max_count {
                                    println!("Your guess is higher, try again.  You have {} guesses remaining", max_count - count + 1);
                                }
                                continue;
                            },
                            Ordering::Equal => {
                                println!("Your guess is equal, congratulations");
                                break;
                            },
                        }
                    }
                } // end not quitting
            }, // end Ok on io
            Err(e) => {
                eprintln!("Failed to read input: {}", e);
                std::process::exit(1)
            }
        }; // end match stdin
    } // end main loop
}
