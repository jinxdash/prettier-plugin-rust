#![feature(associated_type_bounds)]

type X = A<B: C>;

fn f<F>(_: F) where F: for<'a> Trait<Output: 'a> {}
fn f<'b, F>() where for<'a> F: Iterator<Item: 'a> + 'b {}

trait A: MP {
  fn f<IM>(&self) -> i32 where for<'a> IM: T<T: U<<Self as MP>::T<'a>>>;
}

// source: "../../../ext/jinx-rust/tests/samples/features/associated_type_bounds.rs"