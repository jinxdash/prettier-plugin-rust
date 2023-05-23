impl X {}
impl X {
  fn f();
  fn f() {}
  type Y;
  type Z: Ord;
  type W: Ord where Self: Eq;
  type W where Self: Eq;
  fn foo() {
    struct S;
    impl S {
      pub const X: u8 = 0;
      pub const fn bar() {}
      async fn qux() {}
    }
  }
}
impl X for () {}
impl X for Y {}
impl ! {}
impl ! where u8: A {}
impl ! where u8: A {}
impl !Send for A {}
impl <*const u8>::C {}
impl <A as B>::C {}
impl <dyn 'a + B>::C {}
impl <<A>::B>::C {}
impl<'a, I, T: 'a, E> Iterator
  for Y<'a, I, E>
  where I: Iterator<Item = &'a (T, E)> {}
impl<T: Q> S for E<T> {}
unsafe impl A for isize {}
unsafe impl Send for A {}
unsafe impl Sync for A {}
impl<'a> A for &'a [isize] {}
impl ::A::B for ::C {}
impl ::A for () {}
impl ::A {}
impl A for [B; 1] {}
impl A for (<B as C>::D, E) {}
impl ::A for [B; 0] {}
impl<'a> A for &'a [isize] {}
impl<'a> dyn T + 'a {}
impl<'a> dyn T + 'a {}
impl<'a> dyn ::Foo::Trait + 'a {}
impl<T: ?Sized> A for T {}
impl<'a, 'b, 'c> S for &'a &'b &'c Q {}
impl<A, F: Fn(A)> Foo<A> for F {}
impl<T> A for T where for<'a> T: B<'a> {}
impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {}
impl A for (B,) {}
impl !A for (B,) {}
impl A for Box<C> {}
impl A for lib::Something<C> {}
impl A for D<C> {}
impl<T: ::std::fmt::Display> A<T> {}

// source: "../../../ext/jinx-rust/tests/samples/statements/impl.rs"