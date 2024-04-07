// Programming a Guessing Game
use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    // Working with vectors
    let mut how_many = String::new();
    println!("How many numbers do you want to guess?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Error reading input");

    // 

    let num_guesses:u8 = how_many.trim().parse().expect("Error reading input");
    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    let mut guesses_made:u8 = 0;

    println!("Hey, guess a number 1-10:");
    
    while guesses_made < num_guesses {

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };

        match  guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too hight"),
            Ordering::Less => println!("You guessed too low"),
            Ordering::Equal => {
                println!("You guessed CORRECT");
                if guesses_made < num_guesses {
                    println!("Try the next number!!!")
                }
                guesses_made += 1;
        }
    }
}



    // let correct = rand::thread_rng().gen_range(1..=10);
    // println!("Hey, guess a number 1-10:");

    // loop {
    // let mut guess = String::new();

    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Error Reding Imput");

    // // let guess: u32 = guess.trim().parse();

    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(e) => {
    //         println!("Error with parse, try again. {e}");
    //         continue;
    //     }
    // };


    // // let message = if correct < guess {
    // //     String::from("You guessed too high")
    // // } else if correct > guess {
    // //     String::from("You guessed to low!")
    // // } else {
    // //     String::from("You guessed correct")
    // // };

    // let message = match  guess.cmp(&correct) {
    //     Ordering::Greater => "You guessed too high",
    //     Ordering::Less => "You guessed to low!",
    //     Ordering::Equal => {
    //         println!("You guessed correct");
    //         break;
    //     }
    // };

    // println!("{message}")
    // }
}