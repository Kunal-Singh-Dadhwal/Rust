fn main() {
    let x = 5;
    let y = &x; // y is refrence to value of x

    println!("y = {}", y); // this is a address
                           // but rust doew auto derefrencing and prints the value stored in that address
    let mut var = 5;
    var += 1;
    let y = &mut var;
    *y += 1;
    println!("x = {}", *y);

    // let refrence_str = create_str();
}

/*

fn create_str() -> &String {
  let s: String = String::from("Hello");
  return &s;
  // returns a refrence as the function is called and s is in the scope of this function but as the function
  // is called it is in its scope and is dropped as soon as the function is ovver
}

*/
