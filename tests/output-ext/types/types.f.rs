type A where 'a: 'b + 'c = u8;
type A where 'a: 'b + 'c = u8;
type A where 'a: 'b = u8;
type A where 'a: 'b = u8;
type A where 'a:  = u8;
type A where 'a:  = u8;
type A = u8;

// type A = for<'a, T>       fn();
type A = for<'a: 'b + 'c> fn();
type A = for<'a: 'b> fn();
type A = for<'a: 'b> fn();
type A = for<'a: > fn();
type A = for<'a: > fn();
type A = for<'a> fn();
type A = for<> fn();

type A = Box<dyn (Fn(u8) -> u8) + 'static + Send + Sync>;
type A = impl B;

type A = u8;
type A where for<'a> dyn for<'b> Trait1 + ?Trait2: 'a + Trait = u8;
type A where T: = u8;
type A where T: Trait = u8;
type A where T: Trait + Trait = u8;
type A where T: Trait = u8;
type A where T: = u8;

type A = <m::Alias as m::Trait>::X;

pub type A<T> where T: B = T;

type A: Ord;
type B: Ord = u8;
type C: Ord where 'static: 'static = u8;
type D<_T>: Ord;
type E<_T>: Ord = u8;
type F<_T>: Ord where 'static: 'static = u8;

type Y<T> where Self: Sized = u32;
type Y<T>: A where Self: Sized;

pub const FN: &'static fn() = &(fop::<i32> as fn());
const A: &&&u32 = &&&42;
const CONST1: &[bool; 1] = &[true];
const CONST: &[Option<()>; 1] = &[Some(())];
const A: [u32; 1] = [4];
const F: &'static dyn PartialEq<u32> = &7u32;
struct R<'a> {
  c: Box<dyn FnMut(&mut R, bool) + 'a>,
}
fn g() -> impl Tr2<m::Alias> {}
fn leak_dyn_nonprincipal() -> Box<dyn PubPrincipal + PrivNonPrincipal> {}
fn method() -> Self::Pub {}
fn f<T: PrivTr>(arg: T) {}
pub fn unused<const T: usize>() -> usize {}
fn start(_: isize, _: *const *const u8) -> isize {}
fn as_ptr(&self) -> *const Self::Item;
fn as_mut_ptr(&mut self) -> *mut Self::Item;
fn as_ptr(&self) -> *const T {
  self as *const _ as *const _
}
fn as_mut_ptr(&mut self) -> *mut T {
  self as *mut _ as *mut _
}
fn y_uses_f(f: impl Fn()) {}
fn infer<T: a::B>(c: T) -> T {
  c
}
fn f1<'a, 'b, 'c>(_x: &'a u32, _y: &'b u32, _z: &'c u32) where 'c: 'a + 'b {}
fn syntax() {
  A::<T = u8, T: Ord, String>();
  A::<T = u8, 'a, T: Ord>();
  fn y<'a>(y: &mut dyn 'a + Send);
  let z = y as &mut dyn 'a + Send;
  let x: &'static str = "A";
  fn A() -> Box<<Self as A>::T>;
  let a = |a, b: _| -> _ { 0 };
  let a: &usize = &1;
  let a: &&usize = &&1;
  let a: &&&usize = &&&1;
  let a: &&&usize = &&&1;
  let a: &&&&usize = &&&&1;
  let a: &&&&usize = &&&&1;
  let a: &&&&&usize = &&&&&1;
  let a: Box<dyn Debug> = box 3 as Box<dyn Debug>;
  let a: Box<dyn A + B>;
  let a: Box<dyn A + B + C>;
  let a: Box<dyn A + B>;
  let a: Box<dyn A + B>;
  let a: Box<dyn A + B + C>;
  let a: Box<impl A + B + C>;
  let a: Box<impl A + B + C>;
  let a: Box<impl A + B + C + D>;
  let a: Box<dyn A + B + C + D + E>;
  let a: &dyn for<'a> Trait<'a> + 'static;
  let a: &dyn PartialEq<u32> = &7u32;
  let a: Option<!> = None;
  let a = &() as *const () as *const Bottom;
  let a = id(|_: &isize, _: &isize| {});
  let a = id(|_: &isize, _: &isize| {});

  fn equal1<T>(_: &T, _: &T) -> bool {}
  fn equal2<T>(_: &T, _: &T) -> bool where T: {}
  fn A<'a>() where 'a:  {}
  pub fn A<T: 'static>(_: T) -> TypeId {}
  pub fn unused<'a, T>(_: &'a u32) {}
  let f: fn(_, i32) -> i32 = q;
  let _ = S::<>;
  let _ = E::<>::V;
  let a: i32<>;
  let a = (
    A::b::<fn(&'static isize, &'static isize)>(),
    A::b::<for<'a> fn(&'static isize, &'a isize)>(),
    A::b::<for<'a, 'b> fn(&'a isize, &'b isize)>(),
    A::b::<for<'a, 'b> fn(&'b isize, &'a isize)>(),
    A::b::<for<'a> fn(fn(&'a isize) -> &'a isize)>(),
    A::b::<fn(for<'a> fn(&'a isize) -> &'a isize)>(),
    A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'a>>(),
    A::b::<for<'a> fn(&'a dyn Trait<'a>) -> Struct<'static>>(),
    A::b::<for<'a, 'b> fn(&'a dyn Trait<'b>) -> Struct<'b>>(),
    A::b::<fn(for<'a> fn(&'a isize) -> &'a usize)>(),
    A::b::<fn(for<'b> fn(&'b isize) -> &'b usize)>(),
    A::b::<Box<dyn Fn(&'static isize, &'static isize)>>(),
    A::b::<Box<dyn for<'a> Fn(&'static isize, &'a isize)>>(),
    A::b::<Box<dyn for<'a, 'b> Fn(&'a isize, &'b isize)>>(),
    A::b::<Box<dyn for<'a, 'b> Fn(&'b isize, &'a isize)>>(),
    A::b::<Box<dyn for<'a> Fn(Box<dyn Fn(&'a isize) -> &'a isize>)>>(),
    A::b::<Box<dyn Fn(Box<dyn for<'a> Fn(&'a isize) -> &'a isize>)>>(),
    a::<L64<L64<()>>>(),
    a::<L<L64<L64<()>>>>(),
    <&dyn A<B = u8>>::x(&e::r(1)),
    <&'static str>::f(&""),
    a::<>(),
    a as &[&dyn Fn(usize) -> ()],
    a::<&U>(a),
    a::<U>(a),
    a::<&mut U>(a),
  );
  let s: Foo<'a'> = Foo;
  let _: Foo<'b'> = s.into();
  let s2: Foo<'a'> = Foo;
  let _: Foo<'a'> = s2;
  let s3: Foo<'a'> = Foo;
  let _ = s3;
  let s4: Foo<'a'> = Foo;
}
fn A(x: Option<fn(i32)>) -> Option<unsafe fn(i32)> {}
fn f(x: fn(i32)) -> unsafe fn(i32) {}
fn f<'b, L: X<&'b Q<K>>>() {}
struct A<T, U = [u8; a::<T>()]>(T, U);
impl Q for () {}
trait Q<P = Self> {}
trait Q<P: Sized = [Self]> {}
trait H<'d, 'e>: for<'f> I<'d, 'f, 'e> + 'd {}
trait F<'f>: for<'a> A<'a> + for<'e> E<'e> {}
struct Q<A = S, T>(A, T);
struct Q<A, B = Vec<C>, C>(A, B, C);
impl<'a> A<'a> for &'a str {
  fn f<T: B<'a>>(self) -> &'a str {}
}
extern "C" fn A<T: Add>(a: T, b: T) -> T::Output {
  a + b
}
extern "C" {
  pub fn f<'a>(x: &'a i32);
  pub fn f<'b>(x: &'a i32, y: &'b i32);
  pub fn f<'a>(x: &'a i32, y: &i32) -> &'a i32;
  pub fn f<'b>(x: for<'c> fn(&'a i32));
  pub fn f<'b>(x: for<'c> fn(&'b i32));
  pub fn f<'b>(x: for<'c> fn(&'c i32));
  pub fn f<'b>() -> for<'c> fn(&'a i32);
  pub fn f<'b>() -> for<'c> fn(&'b i32);
  pub fn f<'b>() -> for<'c> fn(&'c i32);
}
struct X<'x, 'y> {
  x: std::marker::PhantomData<&'x ()>,
  y: std::marker::PhantomData<&'y ()>,
}
struct G<T> where for<'f> T: F<'f, As: E<'f>> + 'f {
  t: std::marker::PhantomData<T>,
}
struct D<T> where T: for<'c> C<'c, As: A<'c>> {
  t: std::marker::PhantomData<T>,
}
fn f<T>() where T: A, T::U: B {}
fn f(a: isize, b: *const *const u8) -> isize {}
fn f<G: FnMut(A, A) -> A>(mut a: G, b: A, c: A) -> A {}
fn f<T: A<B = T> + C>(x: T) -> T {}
fn f<A, B: a<A>>(x: B) -> C<A> {}
struct Whitespace<T: Clone = ()> {
  t: T,
}
struct TokenSplit<T: Clone = ()> {
  t: T,
}
fn f<'a, 'b, T>(t: T) -> isize where T: 'a, 'a: 'b, T: Eq {
  0
}
impl<T: ?Sized, U: ?Sized> A<B<U>> for C<T> where T: D<U> {}
fn f() where T: for<'a> A<'a> + 'a {}
fn f() where T: for<'g> H<'g, 'g, As: for<'h> H<'h, 'g> + 'g> {}
fn f()
  where
    T: for<'i> H<
      'i,
      'i,
      As: for<'j> H<'j, 'i, As: for<'k> I<'i, 'k, 'j> + 'j> + 'i
    > {}
fn f()
  where
    T: for<'l, 'i> H<
      'l,
      'i,
      As: for<'j> H<'j, 'i, As: for<'k> I<'l, 'k, 'j> + 'j> + 'i
    > {}
fn f()
  where
    T: for<'l, 'i> H<
      'l,
      'i,
      As: for<'j> H<'j, 'i, As: for<'k> H<'j, 'k, As = X<'j, 'k>> + 'j> + 'i
    > {}
fn f() where T: Fn(&(), &()) {}
fn f() where T: Fn(&'a (), &()) {}
fn f() where T: Fn(&(), Box<dyn Fn(&())>) {}
fn f() where T: Fn(&(), fn(&())) {}
fn f() where T: Fn(&(), for<'a> fn(&'a ())) {}
fn f() where T: Fn(&(), Box<dyn Fn(&())>, &(), fn(&(), &())) {}
fn f() where T: for<'a> Fn(&'a (), &()) {}
fn f() where T: for<'a> Fn(&(), &'a ()) {}
fn f() where T: for<'a> Fn(&'a (), &'a ()) {}
fn f() where T: for<'a> Fn(&'a (), Box<dyn Fn(&())>) {}
fn f() where T: for<'a> Fn(&(), Box<dyn Fn(&())>, &'a (), fn(&(), &())) {}
fn f() where T: A, F: FnOnce(B<T>) -> bool {}
fn f() -> impl std::borrow::Borrow<<u8 as A>::S> {}
fn f(_: <() as A<Self::B>>::C) {}
struct S<>;
trait T<> {}
enum E<> {
  V,
}
impl<> T<> for S<> {}
fn f<'a>(x: for<'b, 'c: 'a + 'b> fn(&'a A, &'b B) -> &'c C)
  where
    F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C,
    for<'a, 'b: 'a> F: Fn(&'a A, &'b B) -> &'c C {}
struct S<F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>(F);
struct S<F>(F) where F: for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C;
struct S(dyn for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C);
type T = Box<dyn for<'a, 'b: 'a> Fn(&'a A, &'b B) -> &'c C>;
type L8<T> = L<L<L<L<L<L<L<L<T>>>>>>>>;
type L64<T> = L8<L8<L8<L8<T>>>>;
impl<T> A for T where T: B {
  type C<'a> = <T as D>::E<'a, 'static>;
}
impl<T> A<<() as B<T::C>>::D> for E<T> where T: F, (): G<T::H> {}
type Y<'a> = &'a ();
type Q<'a>;
type Q<'a, 'b>;
type Q<'a, 'b>;
type Q<'a, 'b, T>;
type Q<'a, 'b, T, U>;
type Q<'a, 'b, T, U>;
type Q<'a, 'b, T: S, U>;
type Q<'a, 'b, T: S, U>: S;
type Q<'a, 'b, T: S, U>: E<Target = T> + Into<U>;
type Q<'a, 'b, T: S, U> where T: E<Target = U>, U: Into<T>;
type Q<'a, 'b, T: S, U>
  : E<Target = T> + Into<U>
  where T: E<Target = U>, U: Into<T>;
type Q<'a>: E<Target = <Self::D<'a> as B>::A<'a, 'static>> where Self: 'a;
type S<'a>: Iterator<Q = Self::A<'a>> + E<R = Self::B<'b>>;
type Z = dyn for<'x> Send;
type A = (*const E::R, D);
fn f(&self) -> Pin<Box<dyn Future<Output = Self::B> + '_>>;
fn f(&self) -> Self::Y<'_> {}
fn f(x: &()) -> &() {}
fn f(x: &impl for<'a> X<Y<'a> = &'a ()>) -> &() {}
fn f<'a, T: for<'b> Fun<F<'b> = T>>(t: T) -> T::F<'a> {}
fn f<'a, T: Fun<F<'a> = T>>(t: T) -> T::F<'a> {}
fn f<T: ?for<'a> Sized>() {}
fn f<'a, T1: X<Y = T1>>(t: T1) -> T1::Y<'a>;
fn f<'a>(s: Box<dyn X<Y<'a> = &'a ()>>) {}
fn f<'a>(t: Self::Y<'a>) -> Self::Y<'a>;
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &() {}
fn f<'a, T: ?Sized + Fun<F<'a> = [u8]>>(_: Box<T>) -> &'static T::F<'a> {}
fn f<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a> {}
fn f<T>() where T: S, for<'a> T::Item<'a>: Q {}
fn f<'c, 'd>(s: Box<dyn X<Y = (&'c u32, &'d u32)>>) {}
fn f(e: &impl for<'a> X<Y<'a> = &'a ()>) -> &'static () {}
fn f<T: for<'a> X<Y<'a> = &'a ()>>(x: &T) -> &'static () {}
fn f(x: &mut dyn for<'a> E<R<'a> = &'a i32>) -> usize {}
fn f() where 'static: 'static, dyn 'static: 'static + Copy {}
fn f() where 'static: 'static, dyn 'static + ::Foo: 'static + Copy {}
fn f<F: A>() where F::B: Copy {}
fn f<F: A>() where <F as A>::B: Copy {}
fn f<F: A<B: A>>() where F::B: Copy {}
fn f<T: S<<Self as A>::Q>>(&self, r: &T) -> u64;
fn f() -> impl Default {}
fn f(t: Box<dyn for<'a> Get<i32, i32>>) {}
fn f(t: Box<dyn for<'a> Fn(i32) -> i32>) {}
fn f(t: for<'a> fn(i32) -> i32) {}
fn f(t: for<'a> unsafe fn(i32) -> i32) {}
fn f(t: for<'a> extern "C" fn(i32) -> i32) {}
fn f(t: for<'a> unsafe extern "C" fn(i32) -> i32) {}
impl<T: Trait1, F: FnMut(<T as Trait1>::C)> Callback<T> for F {}
impl Bar<N, M> for Foo<N, M> where A<{ N > 1 }>: B, A<{ M > 1 }>: B {}
async fn f(_: impl for<'a> Add<&'a u8>, _: impl for<'b> Add<&'b u8>) {}
async fn f<'a>(_: &'a ()) -> impl A<dyn B> {}
fn f<D: A>() where D::S: {}
type T: Iterator<Item = <S as T>::T>;
struct R<'a> {
  s: dyn for<'b> E<D<&'b ()>> + 'a,
}
fn f() -> [u8; 4 * 1024 * 1024 * 1024 * 1024] {}
trait Foo where T: Borrow<U> + ?Sized, U: ?Sized + 'b, 'a: 'b, Box<T>: {}
trait Map
  where
    for<'a> &'a Self: IntoIterator<Item = (&'a Self::Key, &'a Self::Value)> {}
trait S: A + AsRef<Self::B> {}
struct Bar<const N: u8>([u8; (N + 2) as usize]) where [(); (N + 2) as usize]:;
fn f<const N: u8>() where D<{ N as usize as u16 }>: {}
fn f<T>()
  where
    for<'a> T: TraitA<
      'a,
      AsA: for<'b> TraitB<'a, 'b, AsB: for<'c> TraitC<'a, 'b, 'c>>
    > {}
fn f<'u, 'a, F>()
  where for<'b> F: Iterator<Item: for<'c> B<'a, 'b, 'c> + for<'c> A<'a, 'c>> {}
fn f(&self, db: &<Q as QueryDb<'_>>::DynDb) {}
pub fn f<'a, I>() -> impl B<I, D = (), C = impl S + 'a>
  where I: A<E = &'a [()]> {}
type S<T: A<B: for<'a> C<&'a u8>>> = D;
type S<T>;
type A = a::b!();
type S where Self: Sized;
fn f(&self, a: &!) {}
type S<T> where T: Display;
type S<'a, T: Debug + 'a>: ?Sized = dyn Iterator<Item = T>;
type S<'x> where T: 'x = &'x ();
type S<'u, 'v> where 'u: 'v = &'v &'u ();
type S where Self: Q + S = E;
type S<'a: 'b, 'b> = (&'a (), &'b ());
type S<'a> where Self: 'static = &'a ();
type S<'a, 'b> where 'b: 'a = (&'a (), &'b ());
type S<'a>: B<&'a [u8]>;
type S<'a>: 'a;
type S<'a: 'a>;
type S<'a> = &'a ();
type S<B>: S<A = B>;
type S<'a, const N: usize>;
type S<'a> where <A as B>::T: 'a, <A as B>::T: 'a = R<&'a S::T, &'a E::T>;
type S<T> = Self::E<'static, T>;
type S = Self::E<'static, 'static>;
impl<'b> ATy for &'b () {}
impl<T: Copy + std::ops::Deref> UnsafeCopy<T> for T {}
impl<T: X<Y<i32> = i32>> M for T {}
type S: Sized where <Self as B>::C: Sized;
type S = Q<<T as R>::E>;
struct B<'a, T: for<'r> X<Y<'r> = &'r ()>> {
  f: <T as X>::Y<'a>,
}
enum E<'a> {
  S(<S as A>::B<'a>),
}
pub type T<P: Send + Send + Send> = P;
type S<'b, 'a: 'b + 'b> = (&'b u32, Vec<&'a i32>);
type S<'b, T: 'b + 'b> = (&'b u32, Vec<T>);
type S<'b, T> where T: 'b, T: 'b = (&'b u32, Vec<T>);
type A = dyn S + ?Sized + ?Sized;
type R = dyn ?Sized + A;
type Q = dyn for<'a> E<'a> + for<'b> R<'b>;
type S = dyn Q<for<'a> fn(&'a u8)> + G<for<'b> fn(&'b u8)>;
type A = dyn ?Sized;
type A = <S as Tr>::A::f<u8>;
trait A: B<i32> + std::fmt::Debug + Send + Sync {}
struct R<Z: ?Sized = E<i32, i32>>(Z);
mod a {
  trait A {
    const A: u8 = 0;
  }

  pub trait B {
    const B: u8 = 0;
  }

  pub trait C: A + B {
    const C: u8 = 0;
  }

  impl A for ::S {}
  impl B for ::S {}
  impl C for ::S {}
}

pub type b = Box<dyn t + sync::Send + sync::Sync + 'static>;
pub type b = Box<dyn for<'tcx> e<'tcx> + sync::Send + sync::Sync + 'static>;
// source: "../../../ext/jinx-rust/tests/samples/types/types.rs"