use std::io :: {self, BufRead};

fn main() {
    println("Welcome my App")
    let stdin = stdin.lock().lines();
    let mut lines = stdin.lock().lines();

    println!("Type something and press Enter");
    if let Some(Ok(line)) = lines.next()
    {
        println("You typed : {} ", line);
    }
}