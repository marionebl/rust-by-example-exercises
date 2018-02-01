fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}

fn create_fn() -> Box<Fn()> {
    let text = String::from("Fn");
    Box::new(move || println!("This is a {}", text))
}

fn create_fnmut() -> Box<FnMut()> {
    let text = String::from("FnMut");
    Box::new(move || println!("This is a {}", text))
}