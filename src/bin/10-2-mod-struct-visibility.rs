mod box_mod {
    pub struct Open<T> {
        pub contents: T,
    }
}

fn main() {
    let open_box = box_mod::Open { contents: "public information" };
    println!("The open box contains: {}", open_box.contents);
}