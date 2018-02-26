struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, right: &Self::A, left: &Self::B) -> bool {
        (&self.0 == right) && (&self.1 == left)
    }
    fn first(&self) -> Self::A { self.0 }
    fn last(&self) -> Self::B { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let right = 3;
    let left = 10;

    let container = Container(right, left);

    println!("Does container contain {} and {}: {}", &right, &left,
        container.contains(&left, &right));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}