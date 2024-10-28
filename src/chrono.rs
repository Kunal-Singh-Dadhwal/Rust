use chrono::{Local, Utc};

fn main(){
   let now = Local::new();
   let utc_now = Utc::now();


   println!("The Utc time is {}", utc_now);
   println!("The local time is {}", now);
}
