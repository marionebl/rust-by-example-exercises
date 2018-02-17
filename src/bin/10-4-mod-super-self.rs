fn main() {
    my::indirect_call();
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");

        self::function();
        function();

        self::cool::function();
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
}