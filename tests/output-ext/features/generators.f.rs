#![feature(generators)]

[
  static move || {
    let a = A::<Box<dyn D>> { b: E::r() };
    yield ();
  },
  |_| {
    a!("-> {}", {
      yield;
    });
  },
];
// source: "../../../ext/jinx-rust/tests/samples/features/generators.rs"