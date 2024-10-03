fn main() {
    let num = 12;

    if num % 3 == 0 && num % 4 == 0 {
        println!("The number is divisible by 12");
    } else if num % 3 == 0 {
        println!("The number is divisible by 3 but not 4");
    } else {
        println!("The number is divisible by 4 but not 3");
    }
}
fn looop() {
    loop {
        println!("Hello");
    }

    // this is an infinite loop
}

fn while_loop() {
    let mut count = 0;

    while count < 5 {
        println!("Count : {}", count);
        count += 1;
    }
}

fn for_loop() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in &arr {
        println!("Element is {}", element);
    }
}
