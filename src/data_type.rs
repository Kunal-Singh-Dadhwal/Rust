use std::any::type_name;
fn main() {
    let floating: f64 = 14.2323;
    // default type is f64 in the cpu
    let is_raining: bool = true;
    let is_sunny: bool = false;
    let need_umbrella = is_raining && !is_sunny;

    let letter: char = 'a';
    println!("float 64 number : {}", floating);
    println!("the char is {}", letter);
    println!("need umbrella is {}", need_umbrella);

    /*
    Array
    */

    let arr1: [u8; 10];
    arr1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("The length of array is {}", arr1.len());

    let mut arr2: [&str; 3] = ["Hello", "World", "lets go Coding"];
    write_arr(&mut arr2);
    println!("arr2 = {:?}", arr2);

    //Type inference compiler automatically infers the data type
    let x = 5;
    let y = 5.5;
    let z = "Hello, World";

    type_of(&x);
    type_of(&y);
    type_of(&z);
}

fn write_arr(arr: &mut [&str; 3]) {
    // very expensive as it can be very big
    arr[1] = "fellow";
    println!("arr = {:?}", arr);
}

fn type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}
