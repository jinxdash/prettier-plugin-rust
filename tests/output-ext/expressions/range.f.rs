fn q() {
  if let 0..3 = 0 {
  }
  if let 0..Y = 0 {
  }
  if let X..3 = 0 {
  }
  if let X..Y = 0 {
  }

  if let 0..=3 = 0 {
  }
  if let 0..=Y = 0 {
  }
  if let X..=3 = 0 {
  }
  if let X..=Y = 0 {
  }

  if let 0..=3 = 0 {
  }
  if let 0..=Y = 0 {
  }
  if let X..=3 = 0 {
  }
  if let X..=Y = 0 {
  }

  if let 0.. = 0 {
  }
  if let X.. = 0 {
  }

  if let ..0 = 0 {
  }
  if let ..Y = 0 {
  }

  if let ..=3 = 0 {
  }
  if let ..=Y = 0 {
  }

  let 0..1;
  let 0..=1;
  let 0..=1;

  let ..0;
  let ..=0;
  let 0..;

  for _ in [0..1] {
  }
  for _ in [0..=1] {
  }
  for _ in [0..] {
  }
  for _ in [..1] {
  }
  for _ in [..=1] {
  }
  for _ in [b..c] {
  }
  for _ in [0..1, 2..3] {
  }
  for _ in [0..=1] {
  }
  for _ in 0..2 {
  }
  for _ in 0..=2 {
  }
  for _ in 0..=3 {
  }
  for _ in 0..=3 + 1 {
  }
  for _ in 0..=5 {
  }
  for _ in 0..=1 + 5 {
  }
  for _ in 1..=1 {
  }
  for _ in 1..=1 + 1 {
  }
  for _ in 0..13 + 13 {
  }
  for _ in 0..=13 - 7 {
  }
  for _ in 0..=f() {
  }
  for _ in 0..=1 + f() {
  }
  let _ = ..11 - 1;
  let _ = ..11;
  let _ = ..11;
  let _ = 1..=11;
  let _ = f() + 1..=f();
  for _ in 1..=ONE {
  }

  let a = 0.0..1.1;
  if let 2..=0 = 3 {
  }
  if let 2..=0 = 3 {
  }
  if let 2..0 = 3 {
  }
  if let ..0 = 3 {
  }
  if let ..=0 = 3 {
  }
  if let 0.. = 5 {
  }
  if let 0..5 = 4 {
  }
  if let 0..=5 = 4 {
  }
  if let -1..=0 | 2..3 | 4 = x {
  }
  for x in -9 + 1..=9 - 2 {
  }
  if let [3..=14, ..] = xs {
  }
  match 0 {
    X.. | 0.. | 'a'.. | 0.0f32.. => {}
    ..=X | ..X => {}
    ..=0 | ..0 => {}
    ..='a' | ..'a' => {}
    ..=0.0f32 | ..0.0f32 => {}
    ..a => {}
    a.. => {}
    1 | -3..0 => {}
    y @ (0..5 | 6) => {}
    y @ -5.. => {}
    y @ ..-7 => {}
    box 0..=9 => {}
    box 10..=15 => {}
    box 16..=20 => {}
    0..1 => {}
    0..1 => {}
    0..1 => {}
  }
  [
    0..1,
    0..1,
    0..1,
    0..1,
    1..,
    ..,
    0..=1,
    0..=1,
    0..=1,
    // 0...1,
    // &0...9,
    &10..=15,
    // box 0...9,
    box 0..=9,
    ..1,
    ..=1,
    0u32..10i32,
    *a..,
  ];
}
fn f() {
  a..
}
fn f() {
  a..b
}
fn f() {
  a()..b()
}
fn foo(-128..=127: i8) {}
// source: "../../../ext/jinx-rust/tests/samples/expressions/range.rs"