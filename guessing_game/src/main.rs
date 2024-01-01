use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number between 1 and 10!");

    // generating the random number
    let min = 1;
    let max = 10;
    let secret_number = rand::thread_rng().gen_range(min..=max);

    // getting the user's guess
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // printing the user's guess and the secret number
    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess}");
}
