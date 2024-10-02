// Global const should be capital
const GLOBAL_CONST: u8 = 100;

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
    print_val(55);

    let num1: u8 = 10;
    let num2: u8 = 20;

    let res: u8 = add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, res);


    let str1 = String::from("This is a ownership string");
    let str2 = str1; // now the owner is str2
    println!("{}\n",str2);
}

fn print_val(item: u8) {
    println!("Hello world you entered {}", item);
}

fn add(num1: u8, num2: u8) -> u8 {
    println!("This is global constant:- {}", GLOBAL_CONST);
    return num1 + num2;
}
