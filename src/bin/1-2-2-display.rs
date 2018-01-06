struct Structure(i32);

impl std::fmt::Display for Structure {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    if self.0 > 3 {
      return write!(f, "many");
    }
    write!(f, "{}", self.0)
  } 
}

fn main() {
  let thing = Structure(1);
  println!("{}", thing);
}
