#![feature(if_let_guard)]

fn main() {
  match e {
    A(ref u) if let a = b && let c = d => {}
    A(ref u) if let x { a: b, c: d } = e && let f = g => {}
    A(a) if let A(b) = a && let A(p) = b && p == z => {}
  }
  match e {
    A(a) if let A(b) = a && let A(p) = b && p == z => {}
    _ => {}
  }
}
// source: "../../../ext/jinx-rust/tests/samples/features/if_let_guard.rs"