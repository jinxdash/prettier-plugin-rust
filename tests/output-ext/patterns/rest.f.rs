fn b() {
  // fn foo(..: u8) {}
  let ..;
  let box ..;
  match x {
    .. | .. => {}
  }
  let &..;
  let &mut ..;
  let x @ ..;
  let ref x @ ..;
  let ref mut x @ ..;
  let (..);
  let (..);
  let (.., .., ..);
  let (.., P, ..);
  let A(..);
  let A(..);
  let A(.., .., ..);
  let A(.., P, ..);
  let [..];
  let [..];
  let [.., .., ..];
  let [.., P, ..];
  match x {
    .. | [(box .., &(..), &mut .., x @ ..), ref x @ ..] | ref mut x @ .. if
      x[..]
    => {}
  }
  a!(..);

  let [..]: &[u8];
  let [..]: &[u8];
  let (..): (u8,);
  let (..): (u8,);

  let (1, (Some(1), 2..=3)) = (1, (None, 2));
  fn func((1, (Some(1), 2..=3)): (isize, (Option<isize>, isize))) {}
  fn fun([a, ref mut b, ref xs @ .., ref c, d]: [X; 6]) {}
  fn foo(a @ [b, mid @ .., c]: [C; 3]) {}

  let [..] = s;
  let [..] = s0;
  let [..] = s1;
  let [..] = s2;
  let [_, ..] = s1;
  let [.., _] = s1;
  let [_, ..] = s2;
  let [.., _] = s2;
  let [_, _, ..] = s2;
  let [_, .., _] = s2;
  let [.., _, _] = s2;
  let Box { 0: _, .. }: Box<()>;
  let Box { 1: _, .. }: Box<()>;

  let [ref _x0, _x1, _, mut _x3, .., ref _x6, _x7] = arr;
  let [ref _x0, ..] = arr;
  let [_x0, ..] = arr;
  let (.., ref mut _x3) = tup;
  let a @ [b, .., c] = [C, mk_c(), C];
  let a @ [b, mid @ .., c] = [C, mk_c(), C];
}

// source: "../../../ext/jinx-rust/tests/samples/patterns/rest.rs"