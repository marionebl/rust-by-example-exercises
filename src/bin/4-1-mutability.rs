fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // OK
    mutable_binding += 1;

    println!("After mutation {}", mutable_binding);

    // https://doc.rust-lang.org/error-index.html#E0384
    // _immutable_binding += 1;
}