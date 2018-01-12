fn main() {
    let outer_binding = 1;

    {
        let inner_binding = 2;
        println!("inner_binding {}", inner_binding);
        
        let outer_binding = 5_f32;
        println!("outer_binding {}", outer_binding);
    }

    // E0425
    // println!("outer short {}", inner_binding);

    println!("outer_binding {}", outer_binding);

    let outer_binding = 1;

    println!("outer_binding {}", outer_binding);
}