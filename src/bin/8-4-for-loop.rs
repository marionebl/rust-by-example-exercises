fn main() {
    // .. is end-exclusive
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    iter(vec!["Bob", "Frank", "Ferris"]);
}

fn iter(names: Vec<&str>) {
    for name in names.iter() {
        match *name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }
}