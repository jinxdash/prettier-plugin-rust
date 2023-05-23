fn a() {
  fn eq(&&other: S) {
    false
  }
  let -2147483648..=2147483647 = 1;
  let 0..=255 = 0u8;
  let -128..=127 = 0i8;
  let '\u{0000}'..='\u{10FFFF}' = 'v';
  let f = |3: isize| println!("hello");

  match *self {
    Foo::<T>(ref x, ref y) => x,
  }

  let A { foo } = mka();
  let A { foo } = mka();

  let B { a, b, c } = mkb();

  match mka() {
    A { foo: _foo } => {}
  }

  match Some(mka()) {
    S { .. } => (),
    Some(A { foo: _foo }) => {}
    None => {}
    _ => (),
  }
  match (x, y) {
    (1, 1) => 1,
    (2, 2) => 2,
    (1..=2, 2) => 3,
    _ => 4,
  }

  if let Some([b'@', filename @ ..]) = Some(b"@abc123") {
    println!("filename {:?}", filename);
  }

  fn f(X(_): A) {}

  let Ok(0): Option<u8> = 42u8;
  let Ok(0): Option<u8>;
  let Ok(0) = 42u8;

  match t {
    Bar::T1(_, Some(x)) => {
      return x * 3;
    }
    _ => {
      panic!();
    }
  }

  match t {
    Bar::T1(_, Some::<isize>(x)) => {
      println!("{}", x);
    }
    _ => {
      panic!();
    }
  }

  match unimplemented!() {
    &&&42 => {}
    FOO => {}
    _ => {}
  }
  fn f4(ref a @ box ref b: Box<NC>) {}
  fn f1(a @ ref b: U) {}
  fn _f(_a @ _b: u8) {}
  let s: &[bool] = &[true];
  let s0: &[bool; 0] = &[];
  let s1: &[bool; 1] = &[false; 1];
  let s2: &[bool; 2] = &[false; 2];
  let [] = s0;
  let [_] = s1;
  let [_, _] = s2;
  while let 0..=2 | 1 = 0 {}
  if let 0..=2 | 1 = 0 {
  }
  match 0u8 {
    0 | 0 => {}
  }
  if let 0 | 0 = 0 {
  } else {
    return;
  }
  let mut arr = [U, U, U, U, U, U, U, U];
  let mut tup = (U, U, U, U, U);
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
  let [ref mut _x0, _, ref _x2, _, _x4, ref mut _x5, _x6, _] = arr;
  let [_, _, _x2, _, _, _x5, _, _] = arr;
  *_x0 = U;
  let a @ (b, c) = (S, S);
  let mut x @ B { b, .. } = B { a: 10, b: C { c: 20 } };
  if let Some(x @ B { b: mut b @ C { c }, .. }) = some_b {
  }
  match x {
    Some(ref mut _y @ ..) => {}
  }
  let ref a @ box ref b = Box::new(NC);
  let a @ b @ c @ d = C;
  let a @ (b, c) = (C, mk_c());
  let a @ P(b, P(c, d)) = P(mk_c(), P(C, C));
  let a @ [b, c] = [C, C];
  let a @ &(b, c) = &(C, C);
  let a @ &(b, &P(c, d)) = &(mk_c(), &P(C, C));
  let a @ (mut b @ ref mut c, d @ ref e) = (u(), u());
  let a @ ref b = U;
  let ref mut a @ ref mut b = U;
  let a @ &mut ref mut b = &mut U;
  let a @ &mut (ref mut b, ref mut c) = &mut (U, U);
  let a @ NC(b, c) = NC(C, C);
  let a @ NC(b, c @ NC(d, e)) = NC(C, NC(C, C));
  let _a @ _b: u8 = 0;
  let &_ = &1_usize;
  let &&_ = &&1_usize;
  let &&&_ = &&&1_usize;
  let &&&_ = &&&1_usize;
  let &&&&_ = &&&&1_usize;
  let &&&&_ = &&&&1_usize;
  let &&&&&_ = &&&&&1_usize;
}

// source: "../../../ext/jinx-rust/tests/samples/patterns/pattern.rs"