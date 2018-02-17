use deeply::nested::function as other;

fn main() {
    other();
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}