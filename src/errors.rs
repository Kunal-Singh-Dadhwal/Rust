/*
 *
 *  There are only panics in rust 
 *
 */

fn main(){
    let v = vec![1,2,3,4,5];

    panic!("It will crash");
    
    println!("{}", v[99]);
}
