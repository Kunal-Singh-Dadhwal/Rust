#[derive(Debug)]
struct User {
    username: String,
    password: String,
    uuid: String,
    is_admin: bool
}

#[derive(Debug)]
struct rect{
    height: u32,
    width: u32
}

fn main(){
    let user = User{
        username: String::from("Test"),
        password: String::from("TheStrongestPasswordEver"),
        uuid: String::from("Not generating it for now"),
        is_admin: false
    };


    let rect1 = rect {
        height: 30,
        width: 20
    };


    let area = rect1.height*rect1.width;
    println!("The area of the rectangle is {}", area);
    println!("{:?}", user);
}
