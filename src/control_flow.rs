fn main() {
    let num = 12;

    if num % 3 == 0 && num % 4 == 0 {
        println!("The number is divisible by 12");
    } else if num % 3 == 0 {
        println!("The number is divisible by 3 but not 4");
    } else {
        println!("The number is divisible by 4 but not 3");
    }

    let x = 13;

    match x {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _ => println!("Neither nor"),
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

fn match_func() {
    let num = 5;

    match num {
        // | is for or
        1 | 3 => println!("Number is equal to 1 or three"),
        2 | 4 => println!("Number is equal to 2 or 4"),
        5 => println!("Number is equal to 5"),
        _ => println!("Number is not equal to any options"),
    }
}

fn is_even(num: i8) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
