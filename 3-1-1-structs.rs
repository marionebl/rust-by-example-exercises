#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
#[allow(dead_code)]
struct Nil;

// A tuple struct
#[allow(dead_code)]
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name: name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point = Point { x: 0.3, y: 0.4 };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}