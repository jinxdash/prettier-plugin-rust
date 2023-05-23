fn a() {
  a::<!>();
  let x: ! = a!();
  let x: ! = unsafe { a::<B, !>(C) };
  <E as From<!>>::from(never);
}
fn a(x: !) -> ! {
  x
}
fn foo(never: !) {}
fn a(x: !) {}
fn a(ref x: !) {}
fn a(x: &[!]) {}
fn a(x: B<(), !>) {}

impl A<!> for B {
  fn c(&self, d: &!) -> E {}
}
impl A for ! {}
type A = !;

// source: "../../../ext/jinx-rust/tests/samples/types/never.rs"