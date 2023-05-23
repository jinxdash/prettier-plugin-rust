macro_rules! spaced {}
macro_rules! brace {
  () => { };
}
macro_rules! bracket {
  () => { };
}
macro_rules! paren {
  () => { };
}
macro_rules! macro_rules {
  () => {};
}
macro_rules! {}

macro m($S:ident, $x:ident) {}
pub macro create_struct($a:ident) {}
pub(crate) macro mac {
  ($arg:expr) => { $arg + $arg };
}

// source: "../../../ext/jinx-rust/tests/samples/macro/macro.item.rs"