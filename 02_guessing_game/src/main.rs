use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // create a variable for the users guess
        let mut guess = String::new();

        // ask the user for input and write to the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // check that the guess was a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare the guess to the secret number. Give the user
        // a clue if they were wrong or end the game of they guessed
        // correctly
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}