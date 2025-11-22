use chrono::prelude::*;
fn main () {
    let utc_time = Utc::now();
    let local_time = Local::now();
    println!("Hello World");
    println!("{}",local_time);
    println!("{}", utc_time);
}