#[cfg(target_os = "linux")]
fn are_you_on_linux() -> bool {
    true
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() -> bool {
    false
}

fn main() {
    let message = if are_you_on_linux()
        { "You are running linux" } 
        else { "You are not running linux" };
    
    println!("{}", message);
    println!("Are you sure?");

    let ensure_message = if cfg!(target_os = "linux")
        { "Yes. It's definitely linux" }
        else { "Yes it's definitely not linux" };

    println!("{}", ensure_message);
}