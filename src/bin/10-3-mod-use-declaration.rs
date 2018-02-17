use deeply::nested::function as other;

fn main() {
    other();

    println!("Entering block");

    {
        use deeply::nested::function;
        function();

        println!("Leaving block");
    }

    function();
}

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}