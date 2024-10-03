use rand::prelude::*;
use std::io::{self, Write};

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
    let index = rng.gen_range(0..guess_list.len());
    let rand_fruit = guess_list[index];

    print!("Enter a guess for fruit:- ");

    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit selected : {}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruit entered is not found");
                }

                if guess_checker(&fruit_selected, rand_fruit) {
                    println!("You are winner");
                    break;
                } else {
                    println!("Take another guess");
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn guess_checker(str1: &String, str2: &str) -> bool {
    return str1 == str2;
}
