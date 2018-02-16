fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();

    // error[E0603]: function `private_function` is private
    // my_mod::private_function();
}

fn function() {
    println!("called `function()`")
}

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`")
    }

    pub fn function() {
        println!("called `my_mod::function()`")
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
    }
}