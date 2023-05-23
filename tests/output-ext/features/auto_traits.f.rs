#![feature(auto_traits)]

auto trait T {}
unsafe auto trait T {}
pub auto trait T {}
pub unsafe auto trait T {}

auto trait T {
  #![attr]
}

#[attr_0]
auto trait T {
  #![attr_1] // comment
}

// source: "../../../ext/jinx-rust/tests/samples/features/auto_traits.rs"