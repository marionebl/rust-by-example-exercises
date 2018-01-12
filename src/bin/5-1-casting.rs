#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // E0308
    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16 is: {}", 1000 as u16);

    println!("1000 as u8 is : {}", 1000 as u8);

    println!("  -1 as u8 is : {}", (-1i8) as u8);

    println!("1000 md 256 is : {}", 1000 % 256);

    println!("1000 as a u8 is : {}", 1000 as u8);

    println!(" 232 as a i8 is : {}", 232 as i8);
}