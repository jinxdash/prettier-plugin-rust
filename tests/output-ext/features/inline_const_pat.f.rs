#![feature(inline_const_pat)]

fn f() {
  match () {
    y @ 0..const { 5 + 1 } => {}
    1..=const { two() } => {}
    const { one() } => {}
  }
}

// source: "../../../ext/jinx-rust/tests/samples/features/inline_const_pat.rs"