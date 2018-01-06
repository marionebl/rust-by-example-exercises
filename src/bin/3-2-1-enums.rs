 //! global attributes via #![attribute]
#![allow(dead_code)]
//! allow bogus doc comments like this one
#![allow(unused_doc_comment)]

// Create an `enum` to classify a web event. Not how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(sString)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a WebEvent enum as an argument ad 
// returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Desctructure `Click` into `x` and `y`
        WebEvent::Click {x, y} => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

fn main() {
    //! 'x' creates a char x
    let pressed = WebEvent::KeyPress('x');
    /// "my_text".to_owned() ~= String::from("my_text");
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}