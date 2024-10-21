use std::collections::HashMap;


fn main(){
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 15);
    scores.insert(String::from("Green"), 30);

    let team_name = String::from("Green");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("The score of {} is {:?}", team_name , score);

    println!("{:?}", scores);

}
