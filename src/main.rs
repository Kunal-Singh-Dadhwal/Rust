fn main() {
    let mut num: u8 = 5;
    //    println!("The number stored in num is {}", num);
    num = 10;
    //    println!("The number stored in num is {}", num);

    /*
       &str - Fixed length string
       String - Dynamic Length String - Heap Allocated
    */

    let sentence: &str = "Hi, Rust lets learn you";
    let mut secondsent: String = String::from("Hi, Rust lets learn you");
    secondsent.push_str(". This is the pushed string");
    println!(
        "The sentence is \"{}\"\nThe second sentence is \"{}\"",
        sentence, secondsent
    );

    let emp_info: (&str, u8) = ("Ramesh", 50);

    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    let (emp_name, emp_age) = emp_info;
    println!(
        "The employee name is {} and the employee age is {}",
        emp_name, emp_age
    );
    print_val();
}

fn print_val() {
    println!("Hello world");
}
