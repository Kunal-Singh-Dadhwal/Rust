#[derive(Debug)]
struct User {
    username: String,
    password: String,
    uuid: String,
    is_admin: bool
}

fn main(){
    let user = User{
        username: String::from("Test"),
        password: String::from("TheStrongestPasswordEver"),
        uuid: String::from("Not generating it for now"),
        is_admin: false
    };

    println!("{:?}", user);
}
