fn main() {
    let mut count = 0_u32;

    println!("Lets count to infinitiy");

    loop {
        count = count + 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough"); 
            break;
        }
    }

    nested_loop();
}

fn nested_loop() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}