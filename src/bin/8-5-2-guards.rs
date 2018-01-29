fn main() {
    let pair = (2, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("twins"),
        (x, y) if x + y == 0 => println!("kaboom"),
        (x, _) if x % 2 == 1 => println!("first odd"),
        _ => println!("meh")
    }
}