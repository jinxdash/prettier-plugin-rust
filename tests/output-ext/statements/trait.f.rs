trait A {}
trait A<B: C> {}
trait T2<'x, 'y>: T1<'x> {}
trait T<E, R> = S<A> where <Self as B<D>>::T: H<R>;
trait A<B: C>: B<B::S> + E<()> {}
trait A<S: B<C = S>>: D<S> {}
trait A = B<C = D>;
trait A<T> = B where T: C<D>;
trait A: B<B = u32> + C<Self::A> {}
trait A = std::fmt::Display + std::fmt::Debug;
trait B = std::fmt::Display + std::fmt::Debug;
trait A = Default;
trait A<T> = B<C = D>;
trait A = B<C>;
trait A<'a, T: 'a> = B<&'a D>;
trait A = 'static;
trait A<B, C> = D + E where F<(G, H)>: I;
trait A<B, C> = where D<E>: F<G>;
trait A<X: T1>: T2 {}
trait A<X: ?Sized> {}
trait A<X: ?Sized, Y> {}
trait A<Y, X: ?Sized> {}
trait A<X: ?Sized, Y: ?Sized> {}
trait A<X: ?Sized + T2> {}
trait A<X: T2 + ?Sized> {}

pub trait A {}
pub trait C = A + B;
pub trait A<R: D>: Sized {}
// source: "../../../ext/jinx-rust/tests/samples/statements/trait.rs"