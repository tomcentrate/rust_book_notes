extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number =  rand::thread_rng().gen_range(1, 101);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        // We use &mut instead of &guess since variables are immutable by default.
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // We use pattern matching to detect errors.
        // Parse has a few different resultsets, so we match based on their possible outputs.
        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        }

        println!("You guessed: {}", guess);

        // comparator has three different outcomes, and we use pattern match to check.
        match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
              println!("You Win!");
              break;
            }
        }
  }
}
