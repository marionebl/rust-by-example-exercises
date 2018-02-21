mod val {
    pub struct Val {
        val: f64
    }

    pub struct GenVal<T> {
        val: T
    }

    impl Val {
        pub fn new(val: f64) -> Val {
            Val { val }
        }
        pub fn value(&self) -> &f64 { &self.val }
    }

    impl <T> GenVal<T> {
        pub fn new(val: T) -> GenVal<T> {
            GenVal { val }
        }
        pub fn value(&self) -> &T { &self.val }
    }
}

fn main() {
    let x = val::Val::new(3.0);
    let y = val::GenVal::new(3i32);

    println!("{}, {}", x.value(), y.value());
}