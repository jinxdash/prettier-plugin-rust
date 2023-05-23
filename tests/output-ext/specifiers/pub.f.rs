enum E {
  pub U,
  pub(crate) T(u8),
  pub(super) T {
    f: String,
  },
}
pub impl Tr for S {
  pub fn f() {}
  pub const C: u8 = 0;
  pub type T = u8;
  pub(in foo) fn f(&self) -> i32 {
    0
  }
}
pub struct Pub(Priv2);
mod bar {
  pub use *;
}
pub trait Sized {}
const MAIN: u8 = {
  pub trait Tr {
    fn f();
    const C: u8;
    type T;
  }
  pub struct S {
    pub a: u8,
  }
  struct Ts(pub u8);
  pub impl Tr for S {
    pub fn f() {}
    pub const C: u8 = 0;
    pub type T = u8;
  }
  pub impl S {
    pub fn f() {}
    pub const C: u8 = 0;
    // pub type T = u8;
  }
  pub extern "C" {
    pub fn f();
    pub static St: u8;
  }
  ()
};
pub(super) fn f(_: Priv) {}
pub(crate) fn g(_: Priv) {}
crate fn h(_: Priv) {}
pub(crate) struct S1;
pub(super) struct S2;
pub(self) struct S3;
pub(in ::core) struct S4;
pub(in a::b) struct S5;
pub type A;
pub static b: Q;
pub extern crate core;
struct Bar(pub ());
pub struct C(pub isize, isize);
pub struct D(pub isize);
pub struct bool;
pub struct Pub<T = Alias>(pub T);
pub type A;
pub mod bar {
  pub use a::b::c;
  pub mod b {}
  pub struct S {
    pub(in foo) x: i32,
  }
}
pub macro m() {}
pub(in Self::f) struct Z;
pub extern crate self as name;
pub use name::name as bug;

// source: "../../../ext/jinx-rust/tests/samples/specifiers/pub.rs"