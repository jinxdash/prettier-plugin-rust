#[generator(yield(i32))]
fn nums() {
  yield_!(3);
}

// some extra samples to track changes
#[// 0
generator(
  // 1
  // 2
  yield(
    // 3
    // 4
    i32
    // 5
  )
  // 6
)]
// 7

// non-conventional syntax (does not format)
#[#[a] generator(  #[b] yield(  #[c] i32  ))]
#[generator  (a(  #[generator(b(i32))]   i32  )  )]
#[generator  (a(  #[generator(yield(i32))]   i32  )  )]
#[generator  (yield(  #[generator(b(i32))]   i32  )  )]
#[generator  (yield(  #[generator(yield(i32))]   i32  )  )]
fn f() {
  yield_!(3);
}

// macros in attr (does not format)
#[attr(foo!(   ))]
fn f() {}

// source: "../../samples/issues/25.rs"