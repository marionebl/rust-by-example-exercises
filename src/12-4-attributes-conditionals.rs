// error[E0428]: the name `conditional_function` is defined multiple times
// fn conditional_function() {
//     println!("condition not met!");
// }

#[cfg(not(some_condition))]
fn conditional_function() {
    println!("condition not met!");
}

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}