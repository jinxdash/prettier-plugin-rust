fn main() {
  let lam = |(a, ref b, c, ref mut d): (X, X, X, X)| {};
  let x = |_: ()| ();
  let y = || x(());
  let mut x = |_: ()| {
    outer = 4;
    ()
  };
  let x = move |_: ()| {
    let inner = outer;
    ()
  };
  const VTABLE: &'static VTable<DST> = &(VTable {
    _to_dst_ptr: |_: *mut ()| unsafe { std::mem::zeroed() },
  });
  let z = a(&mut (|x| x - 22));
  let mut unboxed = || {};
  Box::new(move |y| { x + y });
  s(
    |f| (*f)(),
    Box::new(|| {})
  );
  (0..42).e(|_x| {
    match E(()) as R<(), _> {
      O(()) => s.push(()),
      E(_) => (),
    }
  });
  <()>::a(|| ());
  let f: &mut dyn FnMut<(_,), E = ()> = &mut (|_: <() as Lt<'_>>::T| {});
  Box::new(move |v| {
    (|_| e.d()())(v);
    X
  }) as Box<dyn Fn(i32) -> Q<i32>>;
  let _c = || {
    match b.0.c(1) as D<(), _> {
      _ => 0,
    }
  };
  let f: &dyn Fn(i32) -> _ = &(|x| x + x);
  let f = |x: u32| -> u32 { 1 };
  for f in &[d, g, |x| x] {
    a!("{}", f(6));
  }
  (|| {
    (|| { c.d })();
    (move || { a.b })();
  })();
  let q = a.e(async move { b(move || async move { d!() }) });
  let g = {
    || z(i)
  };
  let _ = || a.e(async { r });
  let _ = E(0)
    .d()
    .q(|ref _a| true);
  let _ = !(4..5).a(|x| (x == 1 || x == 3 || x == 5));
  let _ = !(1..3).a(|x| [1, 2, 3].b(&x));
  let _ = !(1..3).a(|x| (x == 0 || [1, 2, 3].b(&x)));
  let _ = !(1..3).a(|x| ([1, 2, 3].b(&x) || x == 0));
  let _ = !(1..3).a(
    |x| ([1, 2, 3].b(&x) || x == 0 || [4, 5, 6].c(&x) || x == -1)
  );
  let hash: &Fn(&&Block) -> u64 = &(|block| -> u64 { 1 });
  if
    outer_guard.map_or(
      true,
      |(Guard::If(e) | Guard::IfLet(_, e))| !is_local_used(cx, *e, binding_id)
    )
  {
  }
  a = || b;
  [
    foo(|| ()),
    |x: u32| x + 1,
    (|| Box::new(|| {}) as Box<dyn Fn()>)(),
    |_: T| 3,
    move |x: isize, y| x + y + z,
    &mut (|| 22),
    &(|| 22),
    || {
      x += 1;
    },
    call(&(|| {}), ()),
    <()>::drive(|| ()),
    h2(|_: (), _: (), _: (), _: ()| {}),
    move |a: isize, b| { a + b },
    move |a: isize, b| {
      z;
      zz;
      a + b
    },
    |x: usize| x * 2,
    |x: usize| ({ x }) * 2,
    |x: usize| ({ x })(),
    |x| lib::d!(x),
    |x| {
      match x {
        a => { g(a) }
      }
    },
    |x| d!(x),
    |_| async { () },
    |x, y| {},
    |x: &u64, y: &u64| {},
    |x: &u64, y| {},
    |x, y: &u64| {},
    match 0 {
      2 => |a| 2,
      1 => 0,
    },
    [b, |a| 2],
    [|a| 2, b],
    async || 1,
    |ctx: Ctx<(String, String)>| -> io::Result<Response> {
      Ok(Response::new().with_body(ctx.params.0))
    },
    rayon::join(
      || recurse(left, is_less, pred, limit),
      || recurse(right, is_less, Some(pivot), limit)
    ),
    rayon::join(
      1,
      || recurse(left, is_less, pred, limit),
      2,
      || recurse(right, is_less, Some(pivot), limit)
    ),
  ];
  bifornCringer = {
    askTrovenaBeenaDepends = {
      glimseGlyphs =
        |argumentOne, argumentTwo, argumentThree| |restOfTheArguments12345678| {
          return "baz";
        };
    };
  };
  aaaaaaaaaaaaaaaa.map(|x| {
    x += 1;
    x
  }).filter;
  let f = |x| {
    {
      {
        { x }
      }
    }
  };
  let f = |x| {
    {
      { x }
    }
  };
  let f = |x| {
    { x }
  };
  let f = |x| { x };
  let f = |x| x;
}

fn f(_n: isize) -> isize {
  id(|| { 1 }) - 0
}
fn f() {
  || {
    x += y;
  }
}
struct A {
  b: [
    ();
    match || 1 {
      a => 0,
    }
  ],
}
enum E {
  V(
    [
      ();
      {
        let _ = || 1;
        0
      }
    ],
  ),
}
type Ty = [
  ();
  {
    let _ = || 1;
    0
  }
];

// source: "../../../ext/jinx-rust/tests/samples/expressions/closure.rs"