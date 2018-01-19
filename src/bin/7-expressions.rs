fn main() {
    let x = 5_u32;

    // Expected 155
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    // Expected: ()
    let z = {
        2 * x;
    };

    println!("x is {}", x);
    println!("y is {}", y);

    // TIL: () needs has default impl for Display
    println!("z is {:?}", z);
}