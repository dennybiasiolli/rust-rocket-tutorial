use rand::Rng;
use std::io;

fn print_number_request(request_string: String) {
    println!("{request_string} [type \"quit\" to exit]:");
}

fn main() {
    println!("Guess the number between 1 and 10!");

    // generating the random number
    let min = 1;
    let max = 10;
    let secret_number = rand::thread_rng().gen_range(min..=max);

    print_number_request(String::from("Please input your guess"));
    loop {
        // getting the user's guess
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                match num {
                    num if (min..=max).contains(&num) => num,
                    _ => {
                        print_number_request(format!("Please type a number between {min} and {max}."));
                        continue;
                    },
                }
            },
            Err(_) => {
                if guess.trim() == "quit" {
                    println!("Quitting...");
                    break;
                } else {
                    print_number_request(format!("Please type a number between {min} and {max}."));
                    continue;
                }
            },
        };

        let diff = guess as i32 - secret_number as i32;
        match diff {
            0 => {
                println!("You guessed the secret number!");
                break;
            },
            -2..=2 => println!("You were close!"),
            _ => println!("You were way off!"),
        }
    }
}
