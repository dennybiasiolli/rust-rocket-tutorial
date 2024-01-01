use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number between 1 and 10!");

    // generating the random number
    let min = 1;
    let max = 10;
    let secret_number = rand::thread_rng().gen_range(min..=max);

    loop {
        // getting the user's guess
        println!("Please input your guess [type \"quit\" to exit]:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit" {
                    println!("Quitting...");
                    break;
                } else {
                    println!("Please type a number between {} and {}.", min, max);
                    continue;
                }
            },
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
