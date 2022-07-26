fn a() {
  match x {
  }
  match () {
  }
  match (Sd { x: A, y: () }) {
  }
  match *c {
  }
  match ((A, ()), ()) {
  }
  match [0u8; LARGE_SIZE] {
  }
  match na.kind {
  }
  match (T::T1(()), V::V2(true)) {
  }
  match (Sd { x: A, y: () }) {
  }
  match "a" {
  }
  match (&"foo", "bar") {
  }
  match (Foo { foo: true, bar: Some(10), baz: 20 }) {
  }
  match (l1, l2) {
  }

  match 0 {
    0 if false => (),
  }
  match true {
    true => true,
  }

  let v: isize = match &*sl {
  };
  let a: isize = match 1 {
    x if x < 2 => { 3 }
    x if x < 4 => { 5 }
    6 => { 7 }
    _ => { 8 }
  };
  let val = match (
    match (
      match (
        match (
          match () {
            () => (),
          }
        ) {
          () => (),
        }
      ) {
        () => (),
      }
    ) {
      () => (),
    }
  ) {
    () => (),
  };
  let b: isize = match (A { x: 10, y: 20 }) {
    x if x.x < 5 && x.y < 5 => { 1 }
    A { .. } if x == 10 && y == 20 => { 2 }
    A { .. } => { 3 }
  };

  match true {
    true if true => (),
    false if false => unsafe {}
    true => {}
    false => (),
    &[] => 0,
    &[a, b, c] => 3,
    &[a, ref d @ ..] => a,
    &[10, a, ref d @ ..] => 10,
    [h, ..] if h > n => 0,
    [h, ..] if h == n => 1,
    [h, ref ts] => foo(c, n - h) + foo(ts, n),
    [] => 0,
    &A::C(v, box ref a) => tail(e),
    &A::C(x, box A::S) => A::C(c, box A::R),
    0 => {
      return e(j::h::r(a::e::d, ""));
    }
    n => {
      r = &mut a::d(&mut e, &mut [])[n..];
    }
    box Q::V(ed) =>
      match ed.q {
        box R::E(ref d) if d.d.r() => { true }
      }
    _ => panic!(),
    ref _x => unreachable!(),
    0 => {
      return;
    }
    A { a: v } if *v.clone() == 42 => v,
    A((a,)) => {
      *a = 0;
    }
    Some(x) if let Some(y) = x => (x, y),
    Some((x, _)) if let Foo::Bar = bar(x) => panic!(),
    Some((_, x)) if let Foo::Baz = baz(x) => {}
    Some(x) if let Foo::Qux(y) = qux(x) => assert_eq!(y, 84),
    Ok(mut r) | Err(mut r) if true => {
      r = 1;
    }
    Color::Rgb(r, g, b) => (r | g) == 0 || (r | b) == 0 || (g | b) == 0,
    | not_red @ Color::Green
    | not_red @ Color::Blue
    | not_red @ Color::Rgb(..)
    | not_red @ Color::Cyan => format!("{:?}", not_red),
    Ok(x) if let Err(_) = x => {}
    // _ if let _ = !Foo{ a: 1 } => {},
    _ if !(Foo { a: 1 }) => {}
    E { x: A, y: _ } => {}
    D { a: _a } | C { a: _a } if true => {}
    Some(a::B { misc: false, .. }) => {}
    ref _x if false => {}
    "b" => {}
    "b" => {}
    _ => {}
    () if f == Foo { x: 42 } => {}
    _ => {}
    0 => {}
    a => {}
    a::X => {}
    _ => {}
    (a, ..) => {}
    0..128 => {}
    128..=255 => {}
    128..=255 if 1 => {}
    (Some(_), None) | (None, Some(_)) => {}
    S::<{ a() }> => {}
    ((A, _), _) => {}
    [..] => {}
    &[] => {}
    &[1..=255] => {}
    C0 => {}
    T::A {} => {}
    &[_, _, ..] => {}
    [Some(..), None, ref tail @ ..] => {}
    [Some(..), Some(..), ref tail @ ..] => {}
    [None, None, ref tail @ ..] => {}
    [None, Some(..), ref tail @ ..] => {}
    [_, _, ref tail @ .., _] => {}
    (&"foo", &_) => {}
    (&&_, &_) => {}
    Foo { foo: true, bar: Some(_), .. } => {}
    Foo { foo: false, bar: None, .. } => {}
    Foo { foo: true, bar: None, .. } => {}
    Foo { foo: false, bar: Some(_), .. } => {}
    (Some(&[]), Ok(&[])) => {}
    (Some(&[_, ..]), Ok(_)) | (Some(&[_, ..]), Err(())) => {}
    (None, Ok(&[])) | (None, Err(())) | (None, Ok(&[_])) => {}
    (None, Ok(&[_, _, ..])) => {}
    (T::T1(()), V::V1(i)) => {}
    (T::T2(()), V::V2(b)) => {}
    Foo::Bar { bar: Bar::A, .. } => {}
    ::A::B(3) => {}
    ::A::B(_) if false => {}
    ::A::B(..) if false => {}
    ::A::B(_n) => {}
    ::A::B => {}
    ::A::B(::A::B) => {}
    ::A::B(::A::B(_)) => {}
    ::A::B(::A::B, ::A::B(_)) => {}
    ::A::B(::A::B(..), ::A::B) => {}
    ::A::B(..) => {}
    A::<A<u8>> { x: A(10, 11) } => {}
    ::B::<<A<_> as C>::U> { x: A::<u8>(11, 16) } => {}
    isize::MIN..5 | 5..=isize::MAX => {}
    0..5 | 5..=usize::MAX => {}
    (0..5, true) | (5..=usize::MAX, true) | (0..=usize::MAX, false) => {}
    [Ok(box ref a), ref xs @ .., Err(box b), Err(box ref mut c)] => {}
    [Ok(box a), ref xs @ .., Err(box ref b), Err(box ref c)] => {}
    box a => { Foo(box 1) }
    box [Ok(a), ref xs @ .., Err(ref b)] => {}
    ref a @ box b => {}
    ref a @ box ref b => {}
    Ok(ref a @ b) | Err(b @ ref a) => {}
    ref a @ Ok(ref b) | ref a @ Err(ref b) => {}
    ref a @ Ok(ref mut b) | ref a @ Err(ref mut b) if
      {
        *b = U;
        false
      }
    => {}
    ref mut a @ Ok(ref b) | ref mut a @ Err(ref b) if
      {
        *a = Err(U);
        false
      }
    => {}
    a @ Some((mut b @ ref mut c, d @ ref e)) => {}
    mut a @ Some([ref b, ref mut c]) => {}
    ref mut a @ Some([b, mut c]) => {}
    ref mut a @ Ok(ref mut b) | ref mut a @ Err(ref mut b) => {}
    ref bar @ Some(box n) if n > 0 => {}
    Some(ref bar @ box n) if n < 0 => {}
    ref x @ A { ref a, b: 20 } => {}
    (a, _) | (_, a) if a > 10 => 0,
    e @ &1..=2 | e @ &3..=4 => {}
    0 | &1 => {}
    Ok(x) | Err(x) => 0,
    &(Ok(x) | Err(x)) => 0,
    Ok(mut x) | &Err(mut x) => 0,
    Some((a, _) | (_, a)) if a > 10 => 0,
    Some((a, _)) | Some((_, a)) if a > 10 => 0,
    Some(ref bar @ box Test::Baz | ref bar @ box Test::Qux) => 0,
    Some(x) if let Foo::Qux(y) = qux(x) => 0,
    [bar @ .., n] if n == &5 => {}
    &A { a: 2 } if a.b().c() => {}
    A::B { a } => {}
    &A::B { a } => {}
    box A::B { a } => {}
    (A::B { a },) => {}
    [A::B { a }] => {}
    C(A::B { a }, ()) => {}
    ((0 | 1,) | (2 | 3,),) => {}
    (Some(2..=255),) => {}
    (None | Some(0 | 1),) => {}
    (1 | 2,) => {}
    (1 | 2, 3 | 4) => {}
    ([] | [0 | 1..=255] | [_, ..],) => {}
    ((0, 0) | (0, 1),) => {}
    ((0, 0) | (1, 0),) => {}
    | Tri::A(Ok(mut x) | Err(mut x))
    | Tri::B(&Ok(mut x) | Err(mut x))
    | &Tri::C(Ok(mut x) | Err(mut x)) => 0,
    Wrap(Ok(mut x) | &Err(mut x)) => 0,
    Wrap(&(Ok(x) | Err(x))) => 0,
    Wrap(Ok(x) | Err(x)) => 0,
    () if
      if (
        if (if 0 { 0 } else { 0 }) {
          0
        } else {
          0
        }
      ) {
        0
      } else {
        0
      }
    => 0,
    Add | Mul | And | Or | BitXor | BitAnd | BitOr | Eq | Ne => 0,
    Sub | Div | Rem | Shl | Shr | Lt | Le | Ge | Gt => 0,
    | ThisIsA::ReallyLongPatternNameToHelpOverflowTheNextValueOntoTheNextLine
    | ThisIsA::SecondValueSeparatedByAPipe
    | ThisIsA::ThirdValueSeparatedByAPipe => 0,
    MyEnum::Option1 if cfg!(target_os = "windows") =>
      #[cfg(target_os = "windows")] {
        1
      }
    MyEnum::Option1 if cfg!(target_os = "windows") =>
      (#[cfg(target_os = "windows")] 2),
    Some(
      RegionResolutionError::SubSupConflict(
        vid,
        _,
        SubregionOrigin::Subtype(box TypeTrace { cause, values }),
        sub_placeholder @ Region(Interned(RePlaceholder(_), _)),
        _,
        sup_placeholder @ Region(Interned(RePlaceholder(_), _)),
        _,
      ),
    ) =>
      self.try_report_trait_placeholder_mismatch(
        Some(self.tcx().mk_region(ReVar(*vid))),
        cause,
        Some(*sub_placeholder),
        Some(*sup_placeholder),
        values
      ),
    GenericParamKind::Const { kw_span, default: Some(default), .. } => {
      kw_span.to(default.value.span)
    }
  }
}
// source: "../../../ext/jinx-rust/tests/samples/expressions/match.rs"