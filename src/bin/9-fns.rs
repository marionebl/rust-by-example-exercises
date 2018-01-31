fn main() {
    fizzbuzz_to(100)
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        println!("{}", fizzbuzz(n));
    }
}

fn fizzbuzz(n: u32) -> String {
    if divisible(n, 15) {
        String::from("fizzbuzz")
    } else if divisible(n, 3) {
        String::from("fizz")
    } else if divisible(n, 5) {
        String::from("buzz")
    } else {
        format!("{}", n)
    }
}

fn divisible(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}