struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, &A, &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, left: &i32, right: &i32) -> bool {
        (&self.0 == left) && (&self.1 == right)
    }

    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

// Having to specify A and B as type parameters to C<A, B> 
// is superfluous
fn difference<A, B, C>(container: &C) -> i32 where C: Contains<A, B> {
    container.last() - container.first()
}

fn main() {
    let one = 3;
    let two = 10;

    let container = Container(one, two);

    println!("contains {} and {}: {}", &one, &two, container.contains(&one, &two));
    println!("first: {}", container.first());
    println!("last: {}", container.last());

    println!("difference: {}", difference(&container));
}