union {};
union::b {};
union union<'union> {
  union: &'union union<'union>,
}
struct union;

impl union {
  pub fn new() -> Self {
    union {}
  }
}

fn main() {
  let _u = union::new();
  let mut r#async = 1;
  union as T;
}
// source: "../../../ext/jinx-rust/tests/samples/statements/union.rs"