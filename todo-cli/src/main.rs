// use std::io;

fn main() {
    println!("Welcome my App");
    loop{
        println!("Wanna Add ?");
        let mut response = String::new();
        if response == "Yes"
        {
            let mut new_todo = String::new();
            // fixed: use a concrete type (String) instead of a generic `T`
            let mut stack : Vec<String> = Vec::new();
            stack.push(new_todo);
            continue;
        }
    }
}