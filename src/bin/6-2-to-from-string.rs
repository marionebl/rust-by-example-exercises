use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(r: {})", self.radius)
    }
}
 
impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = i32::from_str(s);

        match result {
            Ok(radius) => Ok(Circle { radius }),
            Err(err) => Err(err)
        }
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let circle = Circle::from_str("foo");
    println!("invalid circle: {:?}", circle); // TODO: How to implement Display for this?

    let circle = Circle::from_str("1").unwrap();
    println!("{}", circle);
}