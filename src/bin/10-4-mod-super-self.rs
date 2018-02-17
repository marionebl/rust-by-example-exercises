fn main() {
    my::indirect_call();
}

mod my {
    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
    }
}