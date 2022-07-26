const _: () = {
  pub trait A {
    const _: () = ();
  }
  impl A for () {
    const _: () = ();
  }
  impl dyn A {
    const _: () = ();
  }
};

extern r#"C"# {
  fn bar();
}
extern r#"C"# fn foo() {}
type T = extern r#"C"# fn();
extern "\x43" fn foo() {}
extern "\x43" {
  fn bar();
}
type T = extern "\x43" fn();

// extern crate async;
// extern crate async as something_else;

fn f1();
fn f2() {}
fn f3();

trait X {
  fn f();
  fn f() {}
  const Y: u8;
}

extern "C" {
  fn f();
  fn f();
  static X: u8;
  static mut Y: u8;
  // type E: where;
  type A: Ord;
  type A<'a> where 'a: 'static;
  type A<T: Ord> where T: 'static;
  type A = u8;
  type A<'a: 'static, T: Ord + 'static>
    : Eq + PartialEq
    where T: 'static + Copy = Vec<u8>;
}
const async fn test() {}
async unsafe fn test() {}
const unsafe fn test() {}
unsafe extern fn test() {}
fn f() {
  async fn f();
  unsafe fn f();
  const fn f();
  extern "C" fn f();
  const async unsafe extern "C" fn f();
}
const fn f(a: *const i32, b: i32) -> bool {}
unsafe fn f(&self) -> u32;
const unsafe fn f(v: u32) -> u32 {}
unsafe fn f(func: unsafe fn() -> ()) -> () {}
struct Range<
  const FROM: usize = 0,
  const LEN: usize = 0,
  const TO: usize = FROM
>;
impl<T> From<[u8; 1 + 1]> for Foo<T, [u8; 1 + 1]> {}
fn f(d: [u8; 1 + 1]) -> A<T, [u8; 1 + 1]>
  where [u8; 1 + 1]: From<[u8; 1 + 1]> {}
fn f<'a, 'b, 'c, T>(x: foo::X<'a, T, 'b, 'c>) {}
fn f() -> Option<fn() -> Option<bool>> {
  Some(|| Some(true))
}
fn a() {
  let a = 0;
  let _b = 0;
  let _ = 0;
  let mut b = 0;
  let mut _b = 0;
}

enum Test3 {
  Var1,
  Var2(String),
  StillFine {
    def: i32,
  },
}
enum E {
  UnitVariant,
  TupleVariant(),
  BracedVariant {},
  T(T, [!; 0]),
  #[allow(dead_code)] U(U),
}
fn foobar<F, G>() -> usize where (): Foobar<F, G> {}

mod a {
  extern "C" {
    pub fn free(x: *const u8);
  }
}
pub union U {
  pub a: u8,
  pub(super) b: u8,
  c: u8,
}

trait C<A> {
  fn D<B, F>(&self, f: F) where F: FnMut(A) -> Q<B>;
}
trait A<T: B = Self> {}
trait A: B::C {}
trait A<T>: B::C {}
fn f() -> () {}
fn f<T: X<Y<()> = i32>>() {}
fn f<F>(mut f: F) where F: FnMut(&mut R, bool) {}
fn f<F>(f: F) where F: for<'a> Fn(&'a isize, &'a isize) -> isize {}
fn f<F>(f: F) -> isize where F: Fn() -> isize {
  f()
}
async fn g(((ref a, ref mut b), (ref mut c, ref d)): ((A, A), (A, A))) {}
pub unsafe extern "C" fn bar(_: i32, mut ap: ...) -> usize {}
unsafe fn f(&self, x: &usize) {
  *self + *x;
}
fn f<B: ?Sized + Q>() {}
fn f<A, F: for<'a> F<(&'a A,)>>(_: F) {}
fn f<M>(a: M) where M: A, M::B: C {}
fn f<T, A>(t: fn(&A)) where fn(&A): for<'a> F<(&'a A,)> {}
#[no_mangle]
pub extern "C" fn rust_no_mangle() -> i32 {}
pub fn foo<'a, 'b>(x: Foo<'a, 'b>, _o: Option<&&()>) {
  let _y = x.foo;
}
const x: &'static dyn Fn() = &(|| e!("q"));
const fn foo() -> i32 {}
extern "\x43" fn foo() {}

extern "\x43" {
  fn bar();
}

type T = extern "\x43" fn();
extern r#"C"# fn foo() {}

extern r#"C"# {
  fn bar();
}

type T = extern r#"C"# fn();
// source: "../../../ext/jinx-rust/tests/samples/statements/statements.rs"