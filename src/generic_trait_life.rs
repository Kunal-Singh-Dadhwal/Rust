use std::fmt::Display;

fn longest_with_announcment<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str 
where T: Display {
    println!("Announcmnet! {ann}");

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string");
    let string2 = String::from("short");
    let announcement = String::from("Comparing two strings");

    let result = longest_with_announcment(&string1, &string2, announcement);
    println!("The longest string is: {}", result);
}
