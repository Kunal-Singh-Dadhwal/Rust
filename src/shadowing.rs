fn main() {
    let x = 5; // integer
    let x = "Hello"; //string
    let x = x.len(); // integer

    // This gives no error as we declare x everytime
    // This is called shadowing
    //  if we did without let x then it would give error
    println!("x = {}", x);
}
