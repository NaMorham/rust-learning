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

    println!("Guess the number!");
    println!("-----------------\n");

    let min_val = 1;
    let max_val = 100;
    let max_count = 5;
    let my_num = rand::thread_rng().gen_range(min_val..=max_val);

    let mut count = 0;

    eprintln!("Test: {}", number_suffix(0));
    eprintln!("Test: {}", number_suffix(1));
    eprintln!("Test: {}", number_suffix(2));
    eprintln!("Test: {}", number_suffix(3));
    eprintln!("Test: {}", number_suffix(4));
    eprintln!("Test: {}", number_suffix(10));
    eprintln!("Test: {}", number_suffix(11));
    eprintln!("Test: {}", number_suffix(12));
    eprintln!("Test: {}", number_suffix(13));
    eprintln!("Test: {}", number_suffix(14));
    eprintln!("Test: {}", number_suffix(15));
    eprintln!("Test: {}", number_suffix(16));
    eprintln!("Test: {}", number_suffix(17));
    eprintln!("Test: {}", number_suffix(18));
    eprintln!("Test: {}", number_suffix(19));
    eprintln!("Test: {}", number_suffix(20));
    eprintln!("Test: {}", number_suffix(21));
    eprintln!("Test: {}", number_suffix(22));
    eprintln!("Test: {}", number_suffix(23));
    eprintln!("Test: {}", number_suffix(24));
    eprintln!("Test: {}", number_suffix(100));
    eprintln!("Test: {}", number_suffix(101));
    eprintln!("Test: {}", number_suffix(102));
    eprintln!("Test: {}", number_suffix(103));
    eprintln!("Test: {}", number_suffix(104));
    eprintln!("Test: {}", number_suffix(110));
    eprintln!("Test: {}", number_suffix(111));
    eprintln!("Test: {}", number_suffix(112));
    eprintln!("Test: {}", number_suffix(113));
    eprintln!("Test: {}", number_suffix(114));
    eprintln!("Test: {}", number_suffix(1000));
    eprintln!("Test: {}", number_suffix(1001));
    eprintln!("Test: {}", number_suffix(1002));
    eprintln!("Test: {}", number_suffix(1003));
    eprintln!("Test: {}", number_suffix(1004));
    eprintln!("Test: {}", number_suffix(1010));
    eprintln!("Test: {}", number_suffix(1011));
    eprintln!("Test: {}", number_suffix(1012));
    eprintln!("Test: {}", number_suffix(1013));
    eprintln!("Test: {}", number_suffix(1014));
    eprintln!("Test: {}", number_suffix(1050));
    eprintln!("Test: {}", number_suffix(1051));
    eprintln!("Test: {}", number_suffix(1052));
    eprintln!("Test: {}", number_suffix(1053));
    eprintln!("Test: {}", number_suffix(1054));

    #[cfg(debug_assertions)]
    eprintln!("DBG: my_num: {}", my_num);

    loop {
        if count >= max_count {
            println!("You have taken too many guesses.  Goodbye");
            break;
        }

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
                        count += 1;
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
