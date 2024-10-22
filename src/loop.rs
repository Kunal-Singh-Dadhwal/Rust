
fn get_first_word(word: &String) -> String{
    let mut ans = String::new();


    for char in word.chars(){
        if char == ' '{
            break;
        }
        ans.push(char);

    }
    return ans;
}


fn main() {
    
    for i in 0..10{

        print!("{} ", i);
        // 0 1 2 3 4 5 6 7 8 9 
        // Doesnt include the last number
    }



    // for traversing the array, tuples, strings
    
    let sentence = String::from("This is a test string");

    let first = get_first_word(&sentence);
    println!("The first word of \"{}\" is {}", sentence, first);

}

