struct Foo { 
    x: (u32, u32), 
    y: u32,
}

fn main() {
    let foo = Foo { x: (1, 2), y: 3 };

    let Foo { x: (a, b), y: c } = foo;
    println!("a={}, b={}, c={}", a, b, c);

    let Foo { x: i, y: j } = foo;
    println!("i={:?}, j={}", i, j);

    let Foo { y, .. } = foo;
    println!("y={}", y);

    // error[E0027]: pattern does not mention field `x`
    // let Foo { y }
}