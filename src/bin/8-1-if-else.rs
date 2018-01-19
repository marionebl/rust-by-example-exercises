fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, multiply by 10");

            10 * n
        } else {
            println!(", and is a big number, divide by 2");
            n / 2
            // error[E0308]: if and else have incompatible types
            // n / 2;
        };

        println!("{} -> {}", n, big_n);
}