fn main() {
    function();
    my_mod::function();

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
        println!("called my_mod::function()")
    }
}