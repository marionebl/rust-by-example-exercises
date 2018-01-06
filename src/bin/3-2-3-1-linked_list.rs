use List::*;

#[derive(Debug)]
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

impl List {
    //! Nothing special about `::new`, just conventional
    fn new() -> List {
        Nil
    }

    // Consume a list and return it with a new element in front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // TODO: How to implement append?

    fn len(&self) -> u32 {
        // `self` has to be matched because the behaviour of this method
        // depends on the variant of `self`
        // `self` has type `&List` and `*self` has type `List`.
        // Matching on a concrete type `T` is preferred over a match on a reference `&T`
        //! Resolve pointers via *ptr
        match *self {
            // Can't take ownership of the tail because `self` is borrowed;
            // instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length,
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => format!("Nil")
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}