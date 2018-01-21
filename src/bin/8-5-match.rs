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

        println!("Enter a color:");

        let color = match get_color_input() {
            Ok(color) => color,
            Err(_) => {
                println!("Try again.");
                continue;
            },
        };

        match color {
            Color::Red => println!("Color: Red"),
            Color::Blue => println!("Color: Blue"),
            Color::Green => println!("Color: Green"),
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

enum ColorInputError {
    Io(io::Error),
    Parse
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

fn get_color_input() -> Result<Color, ColorInputError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(i) => Ok(i),
        Err(err) => Err(ColorInputError::Io(err)),
    }?;
    match input.trim() {
        "Red" => Ok(Color::Red),
        "Blue" => Ok(Color::Blue),
        "Green" => Ok(Color::Green), 
        _ => Err(ColorInputError::Parse)
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
