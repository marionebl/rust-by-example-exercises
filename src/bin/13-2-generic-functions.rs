fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));

    generic(SGen('c'));
}

struct A; // concrete
struct S(A); // concrete
struct SGen<T>(T); // generic

fn reg_fn(_s: S) {} // concrete
fn gen_spec_t(_s: SGen<A>) {} // concrete using a generic struct
fn gen_spec_i32(_s: SGen<i32>) {} // concrete using a generic struct
fn generic<T>(_s: SGen<T>) {} // generic