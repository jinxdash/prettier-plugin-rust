#![feature(negative_impls)]

impl !Send for Test {}
impl !Send for Foo {}
impl !Sync for Foo {}
impl !std::marker::Unpin for Foo {}
impl !std::panic::RefUnwindSafe for Foo {}
impl !std::panic::UnwindSafe for Foo {}
impl<T: Send> !A::B for C where T: Copy {}
impl<T: Clone> !A for B where T: Sync {}
impl !Send for A {}
impl !Sync for A {}
impl<'a, T> !MyPredicate<'a> for &T where T: 'a {}
impl<T> !Foo for &T where T: 'static {}
impl<E> !Future for Option<E> where E: Sized {}
// source: "../../../ext/jinx-rust/tests/samples/features/negative_impls.rs"