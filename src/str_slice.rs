fn main() {
    let mut str1 = String::from("Hello World");

    let str2 = &str1[0..5];
    let str3 = ret_first_word(&str1);
    println!("{}", str2);
    println!("{}", str3);


    let arr = [1,2,4,5,67];

    //similarly there are also array slices
    
    println!("{:?}", &arr[0..3]);
}


fn ret_first_word(sent: &String) -> &str {
    let mut index = 0;

    for char in sent.chars(){
        if char == ' ' {
            break;
        }

        index += 1;
    }

    return &sent[0..index];
}
