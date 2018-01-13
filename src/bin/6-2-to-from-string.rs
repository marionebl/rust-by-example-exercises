use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Circle {
    radius: i32
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

impl Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string()) // this creates an infinite loop
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let circle = Circle::from_str("foo").unwrap_err();
    println!("invalid circle: {}", circle);

    let circle = Circle::from_str("1").unwrap();
    println!("{}", circle);
}