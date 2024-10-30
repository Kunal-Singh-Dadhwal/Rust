struct User<'a> {
    name: &'a str
}

struct Userr<'a, 'b> {
    firstn: &'a str,
    secondn: &'b str
}
fn main(){
    let firstname = String::from("Kunal");
    let user = User {
        name: &firstname
    };
    
    println!("The name of the user is {}", user.name);
    let full: Userr;
    let str1 = String::from("Kunal");
    {
        let str2 = String::from("Singh");
        full = Userr {firstn: &str1, secondn: &str2};
    }    
    println!("The full name is {} {}", full.firstn, full.secondn);
}

