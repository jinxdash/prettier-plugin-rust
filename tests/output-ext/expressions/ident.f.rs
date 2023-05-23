fn bare_crate(_: crate::a);
fn bare_global(_: ::a);
fn u8(u8: u8) {
  if u8 != 0u8 {
  }
  assert_eq!(8u8, {
    macro_rules! u8 {
      (
        u8
      ) => { mod u8 { pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 { "u8"; u8 } } };
    }
    let &u8: &u8 = u8::u8(&8u8);
    ::u8(0u8);
    u8!(u8);
    u8
  });
  let &u8: &u8 = u8::u8(&8u8);
  ::u8(0u8);
  u8!(u8);
  u8;
  let µ = 1.0;
  µ;
}

mod u8 {
  pub fn u8<'u8: 'u8 + 'u8>(u8: &'u8 u8) -> &'u8 u8 {
    "u8";
    u8
  }
}

// source: "../../../ext/jinx-rust/tests/samples/expressions/ident.rs"