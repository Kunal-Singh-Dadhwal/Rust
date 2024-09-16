fn main() {
    let mut num: u8 = 5;
    //    println!("The number stored in num is {}", num);
    num = 10;
    //    println!("The number stored in num is {}", num);

    let sentence: &str = "Hi, Rust lets learn you";
    let secondsent: String = String::from("Hi, Rust lets learn you");
    println!(
        "The sentence is \"{}\"\nThe second sentence is \"{}\"",
        sentence, secondsent
    );
}
