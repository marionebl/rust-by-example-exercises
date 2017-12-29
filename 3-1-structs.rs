#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct intantiation is an expression too
        p1: Point { x: my_x, y: my_y },
        p2: point
    };

    let _rectangle2 = Rectangle {
        // struct intantiation is an expression too
        p1: Point { x: 0.0, y: 1.0 },
        p2: Point { x: 10.0, y: 11.0 }
    };

    println!("Area {}", rect_area(_rectangle));
    println!("Area {}", rect_area(_rectangle2));

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantia a tuple struct
    let pair = Pair(1, 0.1);

    // Access a field of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { 
        p1: Point {x: x1, y: y1}, 
        p2: Point {x: x2, y: y2} 
    } = rect;

    (x1 - x2).abs() * (y1 - y2).abs()
}