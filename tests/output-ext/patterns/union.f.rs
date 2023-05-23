fn fw1(H(Ok(mut x) | &Err(mut x)): H<R<'_>>) {}
fn f1((Ok(mut x) | &Err(mut x)): R<'_>) {}
fn fw2(H(&(Ok(x) | Err(x))): H<R<'_>>) {}
fn fw3(H(Ok(x) | Err(x)): H<R<'_>>) {}
fn f2(&(Ok(x) | Err(x)): R<'_>) {}
fn f3((Ok(x) | Err(x)): R<'_>) {}
fn fun((A | B): _) {}
fn f(x @ (A::R(_) | D::E(_)): Q) {}

fn x() {
  let (0 | 1 | _) = 0;
  if let 0 | 1 | 2 = 0 {
  }
  if let x @ 0 | x @ (1 | 2) = 0 {
  }
  if let H(Ok(mut x) | &Err(mut x)) = a {
  }
  if let H(&(Ok(x) | Err(x))) = a {
  }
  if let H(Ok(x) | Err(x)) = a {
  }
  for H(Ok(mut x) | &Err(mut x)) in std::iter::once(wres) {
  }
  for H(&(Ok(x) | Err(x))) in std::iter::once(wres) {
  }
  for H(Ok(x) | Err(x)) in std::iter::once(wres) {
  }

  let H(Ok(mut x) | &Err(mut x)) = wres;
  let H(Ok(x) | Err(x)) = wres;
  let H(&(Ok(x) | Err(x))) = wres;

  let (
    | Tri::A(Ok(mut x) | Err(mut x))
    | Tri::B(&Ok(mut x) | Err(mut x))
    | &Tri::C(Ok(mut x) | Err(mut x))
  ) = tri;

  let (B(A(a, _) | B(a)) | A(a, A(a, _) | B(a))) = B(B(1));
  let (B(_) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));
  let (B(A(a, _) | B(a)) | A(A(a, _) | B(a), A(a, _) | B(a))) = B(B(1));

  let (Ok(a) | Err(a)) = Ok(0);
  let (Ok(ref a) | Err(ref a)) = Ok(0);
  let (Ok(ref mut a) | Err(ref mut a)) = Ok(0);
  let (Ok((V1(a) | V2(a) | V3(a), b)) | Err(Ok((a, b)) | Err((a, b)))): Result<
    _,
    Result<_, _>
  > = Ok((V1(1), 1));

  let (
    Ok((V1(a) | V2(a) | V3(a), ref b)) | Err(Ok((a, ref b)) | Err((a, ref b)))
  ): Result<_, Result<_, _>> = Ok((V1(1), 1));

  let (
    a,
    | Err((ref mut b, ref c, d))
    | Ok(
        (
          | Ok(
              | V1((ref c, d))
              | V2((d, ref c))
              | V3((ref c, Ok((_, d)) | Err((d, _)))),
            )
          | Err((ref c, d)),
          ref mut b,
        ),
      ),
  ): (_, Result<_, _>) = (1, Ok((Ok(V3((1, Ok::<_, (i32, i32)>((1, 1))))), 1)));

  for &(Ok(i) | Err(i)) in &v {
  }
  for Ok(i) | Err(i) in v {
  }
  if let &(None | Some(6 | 7)) = &opt {
  }
  if let Some(x @ (4 | 5 | 6)) = opt {
  } else {
  }
  while let Some(ref mut val @ (3 | 4 | 6)) = opt {}

  let (A | B);
  let (A | B);
  let (A | B): u8;
  let (A | B) = 0;
  let (A | B): u8 = 0;
  for A | B in 0 {
  }
  for A | B in 0 {
  }
  while let A | B = 0 {}
  while let A | B = 0 {}
  if let A | B = 0 {
  }
  if let A | B = 0 {
  }
  if let Ok(mut x) | &Err(mut x) = res {
  }
  if let &(Ok(x) | Err(x)) = res {
  }
  let (Ok(mut x) | &Err(mut x)) = res;
  let &(Ok(x) | Err(x)) = res;
  let (Ok(x) | Err(x)) = res;
  for Ok(mut x) | &Err(mut x) in std::iter::once(res) {
  }
  for &(Ok(x) | Err(x)) in std::iter::once(res) {
  }
  for Ok(x) | Err(x) in std::iter::once(res) {
  }
  let _ = |(A | B): u8| ();
  let (A | B);
  let (A | B,);
  let _ = |(A | B): u8| ();
  let (A | B);
  let (A | B,);
  let A(B | C);
  let E::V(B | C);
  let S { f1: B | C, f2 };
  let E::V { f1: B | C, f2 };
  let [A | B, .. | ..];
  let (box 0 | 1);
  let (&0 | 1);
  let (&mut 0 | 1);
  let (x @ 0 | 1);
  let (ref x @ 0 | 1);
  let (ref mut x @ 0 | 1);

  let (a, A(a, _) | B(a)) = (0, A(1, 2));
  let (A(a, _) | B(a), a) = (A(0, 1), 2);
  let (A(a, a) | B(a)) = A(0, 1);
  let (B(a) | A(a, a)) = A(0, 1);
  match A(0, 1) {
    A | B => 0,
    A | B => 0,
    B(a) | A(a, a) => 0,
    Ok(x @ 4) | Err(x @ (6 | 8)) => 0,
    Ok(x @ 1 | x @ 2) => 0,
    Err(x @ (0..=10 | 30..=40)) if x % 2 == 0 => 0,
    Err(x @ 0..=40) => 0,
    Some(box Test::Foo | box Test::Bar) => 0,
    &((true, y) | (y, true), z @ (0 | 4)) => (y as u8) + z,
    Foo::One(0) | Foo::One(1) | Foo::One(2) => 0,
    Foo::One(42 | 255) => 0,
    Foo::Two(42 | 255, 1024 | 2048) => 0,
    Foo::One(100 | 110..=120 | 210..=220) => 0,
    Foo::Two(0..=10 | 100..=110, 0 | _) => 0,
    ([] | [0 | 1..=255] | [_, ..],) => 0,
    ((0, 0) | (0, 1),) => 0,
    (a, _) | (_, a) if a > 10 => 0,
    Some((a, _)) | Some((_, a)) if a > 10 => 0,
    Some((a, _) | (_, a)) if a > 10 => 0,
    e @ &1..=2 | e @ &3..=4 => 0,
    Ok(mut x) | &Err(mut x) => 0,
    | Tri::A(Ok(mut x) | Err(mut x))
    | Tri::B(&Ok(mut x) | Err(mut x))
    | &Tri::C(Ok(mut x) | Err(mut x)) => 0,
    A | B => 0,
    A | B => 0,
    [.., Some(Test::Qux | Test::Foo)] => 0,
    [Some(Test::Foo), .., Some(Test::Baz | Test::Bar)] => 0,
    [.., Some(Test::Bar | Test::Baz), _] => 0,
    Some(
      | Test::Foo { first: 1024 | 2048, second: 2048 | 4096 }
      | Test::Bar { other: Some(Other::One | Other::Two) },
    ) => 0,
    (
      (a, _) | (_, a),
      (b @ _, _) | (_, b @ _),
      (c @ false, _) | (_, c @ true),
    ) if
      {
        guard_count += 1;
        (a, b, c) == target
      }
    => 0,
    | ((a, _), (b @ _, _), (c @ false, _))
    | ((a, _), (b @ _, _), (_, c @ true))
    | ((a, _), (_, b @ _), (c @ false, _))
    | ((a, _), (_, b @ _), (_, c @ true))
    | ((_, a), (b @ _, _), (c @ false, _))
    | ((_, a), (b @ _, _), (_, c @ true))
    | ((_, a), (_, b @ _), (c @ false, _))
    | ((_, a), (_, b @ _), (_, c @ true)) if
      {
        guard_count += 1;
        (a, b, c) == target
      }
    => 0,
  }
}

accept_pat!(p | q);
accept_pat!((p | q,));
accept_pat!(TS(p | q));
accept_pat!(NS { f: p | q });
accept_pat!([p | q]);

// source: "../../../ext/jinx-rust/tests/samples/patterns/union.rs"