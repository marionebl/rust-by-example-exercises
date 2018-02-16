fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();

    // error[E0603]: function `private_function` is private
    // my_mod::private_function();

    // error[E0603]: function `public_function_in_my_mod` is private
    // my_mod::nested::public_function_in_my_mod();

    // error[E0603]: function `private_function` is private
    // my_mod::nested::private_function();
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

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        pub(in my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }
}