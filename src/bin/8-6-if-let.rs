use std::io;

fn main() {
    println!("number:");
    let number = get_number();

    println!("letter:");
    let letter = get_letter();

    println!("number={:?}", number);
    println!("letter={:?}", letter);

    if let Some(i) = number {
        println!("Matched number {:?}!", i);
    } 

    let i_like_letters = false;

    if let Some(i) = letter {
        println!("Matched letter {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
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

fn get_letter() -> Option<String> {
    let mut result = String::new();

    if let Err(_) = io::stdin().read_line(&mut result) {
        return None;
    }

    match result.trim().parse::<i32>() {
        Ok(_) => None,
        Err(_) => Some(result)
    }
}
