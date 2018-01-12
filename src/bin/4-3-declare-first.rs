fn main() {
    let a;

    {
        let x = 2;
        a = x * x;
    }

    println!("a: {}", a);

    let b;
// E0381
//    println!("b: {}", b);
    
    b = 1;
    println!("b: {}", b);
}