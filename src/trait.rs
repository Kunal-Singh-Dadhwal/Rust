/*
 *
 * A trait defines a functionality a particular type has and share with 
 * other types. We can use traits to define shared behaviour in an abstract
 * way. We can use trait boinds to specify a generic type 
 * 
 * Similar to interfaces in other langs
 *
 *
 */



// Serves as a blueprint for structs
trait Summary {
    fn summarize(&self) -> String {
        // if its not implemented this default will be used
        println!("Default impl");
        return " ".to_string();
    }
}


struct User{
    name:String,
    age: u32
}

impl Summary for User {
    fn summarize(&self) -> String{
        return format!("Username is {} and age is {}", self.name, self.age);
    }
}


// We can also pass traits as parameters as we know it would have a function summarize

fn notify(u: &impl Summary){
    println!("{}", u.summarize());
}


fn main(){
    let user = User {
        name: String::from("Test"),
        age: 30
    };

    println!("{}", user.summarize());
    notify(&user);
}
