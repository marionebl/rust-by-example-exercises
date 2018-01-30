use std::io;

fn main() {
    println!("number:");

    while let None = get_number() {
        println!("Not a number, try again");
    };

    println!("You did it!");
}

fn get_number() -> Option<i32> {
    let mut result = String::new();

    if let Err(_) = io::stdin().read_line(&mut result) {
        return None;
    }

    println!("result {:?}", result.trim());

    match result.trim().parse::<i32>() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}
