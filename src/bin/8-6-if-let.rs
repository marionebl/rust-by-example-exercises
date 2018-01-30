use std::io;

fn main() {
    let number = get_input();

    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } 
}

fn get_input() -> Option<i32> {
    let mut result = String::new();

    if let Err(_) = io::stdin().read_line(&mut result) {
        return None;
    }

    match result.parse() {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}