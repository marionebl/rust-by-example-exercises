fn main() {
    reg_fn(S(A));
}

struct A;
struct S(A);

fn reg_fn(_s: S) {}