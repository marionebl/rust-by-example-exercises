struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // error[E0382]: use of moved value: `empty`
    // empty;
    // error[E0382]: use of moved value: `null`
    // null;
}