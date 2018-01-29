fn age() -> u32 {
    15
}

fn main() {
    match age() {
        0 => println!("Not born yet"),
        n @ 1 ... 12 => println!("Child of age {}", n),
        n @ 13 ... 19 => println!("Teen of age {}", n),
        n => println!("Old person of age {}", n),
    }
}