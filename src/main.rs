// Programming a Guessing Game
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("Hey, guess a number 1-10:");

    loop {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error Reding Imput");

    // let guess: u32 = guess.trim().parse();

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Error with parse, try again. {e}");
            continue;
        }
    };


    // let message = if correct < guess {
    //     String::from("You guessed too high")
    // } else if correct > guess {
    //     String::from("You guessed to low!")
    // } else {
    //     String::from("You guessed correct")
    // };

    let message = match  guess.cmp(&correct) {
        Ordering::Greater => "You guessed too high",
        Ordering::Less => "You guessed to low!",
        Ordering::Equal => {
            println!("You guessed correct");
            break;
        }
    };

    println!("{message}")
    }
}