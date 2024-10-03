fn main() {
    // Vector declaration
    let mut vec1: Vec<f64> = Vec::new();
    /*
     another declaration

     let mut vec = Vec::<u8>::new();
    */
    vec1.push(3.13);
    vec1.push(24.33);
    vec1.push(33.2134);
    let v = vec![1, 2, 3, 4, 5];

    // no push pop will work as it is unmutable

    // v.push(10);
    // v.pop();
    println!("Vector v={:?}", vec1);
    println!("Vector v={:?}", v);

    let mut vrr: Vec<&str> = vec!["Hello", "World", "Lets", "Code"];
    write_vec(&mut vrr); // transfer of ownership but & gives us the boeeowing feature

    println!("Vec vrr = {:?}", vrr);
}

fn write_vec(vec: &mut Vec<&str>) {
    vec.push("Rust");
    println!("Vec vrr in the function write_vec = {:?}", vec);
}
