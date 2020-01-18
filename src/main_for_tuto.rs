use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let result = guess.trim().parse::<i32>().unwrap() == secret_number;

    println!("You guessed: {}, which is {}", guess, result);

    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number

    // send email
    // https://gist.github.com/gyng/5d60225d55928ab4cf55309c88b25ecf
    // https://github.com/lettre/lettre
}
