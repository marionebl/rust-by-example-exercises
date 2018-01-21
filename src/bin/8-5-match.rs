#[allow(dead_code)]

use std::io;
use std::num;

fn main() {
    loop {
        println!("Enter a number:");

        let number = match get_input() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again.");
                continue;
            },
        };

        println!("Tell me about {}", number);

        match number {
            11 | 12 => println!("This is 11 | 12"),
            13...19 => println!("This is 13...19"),
            num if is_prime(num) => println!("Is a prime"),
            _ => println!("Ain't special")
        }

        destructure_enum();
    }
}

enum InputError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

fn get_input() -> Result<i32, InputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(i) => Ok(i),
        Err(err) => Err(InputError::Io(err)),
    }?;
    match input.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(err) => Err(InputError::Parse(err)),
    }
}

fn is_prime(num: i32) -> bool {
    match num {
        0 | 1 => false,
        2 => true,
        n => {
            for i in 2..(n as f64).sqrt() as i64+1 {
                if n % (i as i32) == 0 {
                    return false
                }
            }

            true
        }
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u8, u8, u8),
    HSV(u16, u8, u8),
    HSL(u16, u8, u8),
    CMY(u8, u8, u8),
    CMYK(u8, u8, u8, u8)
}

fn destructure_enum() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
    }
}