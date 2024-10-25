fn find_first_a(s: String) -> Option<i32>{
    for(index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}


fn main(){
    let my_string = String::from("this is ream");
    match find_first_a(my_string){
        Some(index) => println!("The index of first a is {}", index),
        None => println!("There is no a in the string"),
    }
    let my_string1 = String::from("this is test with no ");
    match find_first_a(my_string1){
        Some(index) => println!("The index of first a is {}", index),
        None => println!("There is no a in the string"),
    }
}
