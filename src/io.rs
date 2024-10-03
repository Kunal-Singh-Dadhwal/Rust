use std::io;

fn main() {
    let mut input: String = String::new();
    println!("Give a string input :- ");
    io::stdin().read_line(&mut input).expect("Input failed");
    println!("User input = {}", input);
}
