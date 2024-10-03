use rand::prelude::*;
use std::io::*;

fn main() {
    let guess_list = [
        "banana",
        "apple",
        "mango",
        "orange",
        "grapes",
        "peach",
        "watermelon",
        "muskmelon",
    ];
    let mut rng = thread_rng();

    loop {
        let index = rng.gen_range(0, guess_list.len());
        let random_fruit = guess_list[index];
        let mut input = String::new();
        print!("Please enter your guess fruit:- ");
        io::stdin::read_line(&mut input).expect("Input failed");

        if input == random_fruit {
            println!("You won the game");
        } else {
            println!("You lost the game");
        }
    }
}
