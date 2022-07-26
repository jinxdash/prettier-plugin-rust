struct S<A: ?for<'a> Q, B: 'a + Q, C: 'a, G: Q + 'a, H: Q, I:>;
struct Empty1 {}
struct Empty2;
struct Empty7();
struct Align8Many {
  a: i32,
  b: i32,
  c: i32,
  d: u8,
}
struct A<T>([T]);
struct A<T>(T);
struct cat {
  done: extern "C" fn(usize),
  meows: usize,
}
struct Test3<'a, 'b, 'c> {
  x: extern "Rust" fn(&'a isize),
  y: extern "Rust" fn(&'b [isize]),
  c: extern "Rust" fn(&'c str),
  a: fn(u32) -> u32,
  b: extern "C" fn(u32) -> u32,
  c: unsafe fn(u32) -> u32,
  d: unsafe extern "C" fn(u32) -> u32,
}
struct Test4<'a, 'b: 'a> {
  x: &'a mut &'b isize,
}
struct Test6<'a, 'b: 'a> {
  x: &'a mut extern "Rust" fn(&'b isize),
}
struct Foo<'a> {
  x: Box<dyn (Fn(i32) -> &'a i32) + 'static>,
}
struct X;
struct U {}
struct P<T>(T);
struct A<U> where U: E(U);
struct A<U> where U: E(U) -> R;
struct A<U>(U) where U: Eq;
struct K<'a>(&'a ());
pub struct A([u8; 1]);
pub(crate) struct S<'a, I, E>(I, &'a E);
pub struct A<T: B>(C);
pub struct A<T = u8>(T);
pub struct Table<T, const N: usize>([Option<T>; N]);
struct B;
struct A<T: ?Sized>(C<T>, ());
struct A<T = i32, U = i32>(B<T, U>) where B<T, U>: Marker;
struct A<T = u32, U = i32>(T, U) where B<T, U>: Marker;
struct A<'a, S: B<'a> = i32>(S, &'a ());
struct S1(pub(in foo) (), pub T, pub(crate) (), pub ((), T));
struct G<T, U>(*const T, *const U);
pub struct Unique<T: ?Sized> {
  s: *const T,
}
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}
pub struct A(u32, ::b::Q);
struct S(<AT as A<DT>>::AS);
pub struct A<I> where I: B<C: D> {
  w: <I::E as F>::G,
}
// source: "../../../ext/jinx-rust/tests/samples/statements/struct.rs"