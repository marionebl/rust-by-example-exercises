fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
}

struct A; // concrete
struct S(A); // concrete
struct SGen<T>(T); // generic

fn reg_fn(_s: S) {} // concrete
fn gen_spec_t(_s: SGen<A>) {} // concrete using a generic struct