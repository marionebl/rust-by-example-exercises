#![allow(dead_code)]

enum Status {
    Disadvantaged,
    Privileged,
}

enum Work {
    Creator,
    Manager,
}

fn main() {
    // Explicitly `use` each name so they are available withpout
    // manual scoping.
    use Status::{Privileged, Disadvantaged};

    // Automatically `use` each name inside `Work`.
    use Work::*;

    // Equivalent to `Status::Disadvantaged`.
    let status = Privileged;
    // Equivalent to `Work::Creator`.
    let work = Creator;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Privileged => println!("The privileged have power and money."),
        Disadvantaged => println!("The disadvantaged struggle to an greater extent.", ),
    }

    match work {
        // Note again the lack of scoping
        Creator => println!("Creators build things."),
        Manager => println!("Managers see to completion of tasks."),
    }
}