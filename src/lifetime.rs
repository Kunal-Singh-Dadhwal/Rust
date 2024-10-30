/*
 *  Bohot mushkil hai yaar 😭
 */

// Lets take this example below lets modify it a bit using &str

//fn longest(a: String, b: String) -> String {
//    if a.len() > b.len() {
//        a
//    } else {
//        b
//    }
//}
//
//fn main(){
//    let longest_str;
//    let str1 = String::from("small");
//
//    {
//        let str2 = String::from("longer");
//        longest_str = longest(str1,str2);
//    }
//    println!("{}", longest_str);
//}



// we use this generic lifetime can be also <'a>
// it can be named as a variable
fn longest<'t>(a: &'t str, b: &'t str) -> &'t str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// When str1 and str2 are passed to longest fn the strings are owned by the variables but 
// when the scope ends str2 value is dropped and ans is &str to str2 which results in a dangling
// pointer so we need lifetime here
fn main(){
    let longest_str;
    let str1 = String::from("small");
    let str2 = String::from("longer");
    // here also lifetime rule comes here
    longest_str = longest(&str1, &str2);
    // {
    //     let str2 = String::from("longer");
    //     longest_str = longest(&str1,&str2);
    // }
    println!("{}", longest_str);
}

