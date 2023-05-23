const X: u8;
const B;
const A: u8;
pub const A: Self::AssocTy = 1;
const FOO: dyn Fn() -> _ = "";
pub const FOO: &'static *const i32 = &(&0 as _);
const TEST: fn() -> _ = 1;
const MY_A: A = A {
  e: |s, a, b| {
    if s {
      let _ = ();
    } else if let Q(s) = b.r(|p| p.d()) {
      let _ = ();
    }
  },
};

// source: "../../../ext/jinx-rust/tests/samples/statements/const.rs"