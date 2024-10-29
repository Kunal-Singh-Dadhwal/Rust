fn main(){
    let greatest_char = largest('a', 'b');
    let greatest_num = largest(1, 2);

    println!("{}", greatest_char);
    println!("{}", greatest_num);
}

// Using std::cmp::Partialord to perform comparison operations 
fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T{
    if a > b {
        a
    } else {
        b
    }
}
