fn main() {
  let a = async move {};
  9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999;
  // boop
  vec![1, 2, 3].len();
  write!(vec![], "")?;
  println!("");
  [0];
  b.a;
  *foo += 1;
  let &_ = bar;
  let &mut _ = foo;
  if let _ = 0 {
  }
  while let _ = 0 {}
  let ((), ()) = ((), ());
  let x: &[u8] = &[0];
  let Foo { a, ref b, mut c, x: y, z: z } = foo;
  let x = &raw const y;
  let x = &raw mut y;
  a::<Box<isize>, _>(box 1);
  if &raw const one == &raw mut one {
  }
  let _x = if false { 0 } else if true { panic!() } else { 10 };
  let _: <m::A as m::B>::C;
  let a: A = A { name: 0 };
  let b1 = &mut *b;
  let mut x: Box<_> = box 3;
  let x: (Box<_>,) = (box 1,);
  let &mut ref x = b;
  let &mut mut x = b;
  let ref mut y = b;
  let (a, b, c, d);
  let (mut c, mut d);
  let s = S { x: 3, y: 4 };
  let mut r = R { c: Box::new(f) };
  let _: &usize = &1;
  let _: &&usize = &&1;
  let _: &&&usize = &&&1;
  let _: &&&usize = &&&1;
  let _: &&&&usize = &&&&1;
  let _: &&&&usize = &&&&1;
  let _: &&&&&usize = &&&&&1;
  let x: T = **item;
  let &x = &(&1isize as &dyn T);
  let &x = &&(&1isize as &dyn T);
  let &&x = &&(&1isize as &dyn T);
  let &x = &1isize as &dyn T;
  let &&x = &1isize as &dyn T;
  let &&x = &(&1isize as &dyn T);
  let &&&x = &(&1isize as &dyn T);
  let box x = box 1isize as Box<dyn T>;
  let box box x = box 1isize as Box<dyn T>;
  let a =
    ((b[0] as u64) << 0) |
    ((b[1] as u64) << 8) |
    ((b[2] as u64) << 16) |
    ((b[3] as u64) << 24);
  let a = if let Err(b) = c { d } else { e!("") };
  let mut n3 = N3 { n: N2(N1 { n: N0 { x: Box::new(42) } }) };
  n3.n.0.n.x = n3.n.0.n.x;
  let mut t = (1, ((2, 3, (4, 5)),));
  t.1.0.2.1 = t.1.0.2.1;
  let mut a: A<(), &mut i32> = try { 1 };
  let _ = &mut *s.0.borrow_mut();
  let _ = &mut *s[0].borrow_mut();
  let x: Foo<_> = Bar::<usize>(PhantomData);
  let f = A::<i32> { a: 10 };
  let v: <() as Lt<'_>>::T = ();
  <E>::V() = E::V();
  (<E>::V {} = E::V());
  let a = &mut b.0.0;
  let a = &mut b.0[2];
  let _ = a::<N>(b().await).await;
  let _ = a(b::<N>().await).await;
  let _ = A == s!("e");
  let a: &str = &b;
  ::a::<f64, [u8; 8]>(a!());
  let (the, guardian, stands, resolute) = (
    "the",
    "Turbofish",
    "remains",
    "undefeated",
  );
  let _: (bool, bool) = (the < guardian, stands > resolute);
  let (A { x: _x, y: _y }, Z): (_, Z) = c(
    || B { x: X, y: Y },
    || Z
  );
  let _: A<{ 1 + 2 }>;
  let _: A<{ 5 }>;
  let A::<1, N>(N) = A::new();
  let _ = Some(Foo { _a: 42 }).map(|a| a as Foo<i32>);
  let _ = {
    () = {
      () = {
        () = ();
      };
    };
  };
  String::<>::from::<>("><>").chars::<>().rev::<>().collect::<String>();
  fn a(x: &f<r>) {
    return while !x.f() {
      x.g(0);
    };
  }
  let i = &f::s(0);
  <u8 as D<13>>::e::<u8>();
  let _: i32 = (match "" {
    "+" => ::std::ops::Add::add,
    "-" => ::std::ops::Sub::sub,
    "<" => |a, b| (a < b) as i32,
    _ => c!(),
  })(5, 5);
  [].e().f(|_: &i32| {
    [].e().f(move |_: &i32| {
      i += 1;
    });
  });
  let _x2 = X { a: 1, b: 2, ..DX };
  i[i[0]] = 0;
  i[i[0] - 1] = 0;
}

[
  b.a,
  X { len: 3 },
  x.len > 3,
  x.len >> 3,
  vec![1, 2, 3].into_iter().collect::<Vec<usize>>(),
  X(1, 2, 3),
  (1, 2, 3),
  vec![1, 2, 3].len(),
  write!(vec![], "")?,
  &*d.borrow(),
  **bar == Test::Baz || **bar == Test::Qux,
  &foo[0..1],
  TypeId::of::<T>(),
  &[*xs[0].x, *xs[1].x],
  &mut tup.0,
  <_>::f(),
  &(fop::<T> as fn()),
  a(),
  ::foo::bar::baz::f(),
  <() as ::foo::T>::Assoc::f(),
  [].a(),
  id::<[i32; 3]>([1, 2, 3]),
  m::Pub(0u8).method_with_priv_params(loop {
  }),
  <m::Pub<m::Alias>>::INHERENT_ASSOC_CONST,
  <a!() as B>::f(0),
  a::<B<N, { N as u128 }>>(),
  Foo { f: x.clone() },
  a::<&str, (*const u8, u64)>(),
  a("".b()).c("").d().await,
  foo(&[vec![123]]).await,
  A::b::<C>(x).d(E("x"))?.f(1),
  // std::<_ as _>,
  std::<0>,
  &raw const x,
  (A::a as fn(&(dyn A + 'static)) -> B)(&"c"),
  f::<<T as S>::E>(),
  <u64 as From<<T as Iterator>::Item>>::from(),
  <<Q as A<'_>>::B as C<Q::D>>::e(db),
  tuple.0.0,
  tuple.0.0,
  tuple /*special cases*/.0.0, //aaa
  (((),),),
  (1, (2, 3)).1.1,
  ((1, (2, (3, 4))).1.1.1)(1),
  1,
  a((1, 2.0, 3)),
  b((1,)),
  (1).f::<T>(),
  {
    *a = &a[1..];
  },
  a().await.0,
  a.b(c).await.d(e)?,
  0 + 1,
  0 * 1,
  0 - 1,
  0 / 1,
  0 % 1,
  0 & 1,
  0 | 1,
  0 << 1,
  0 >> 1,
  0 == 1,
  0 != 1,
  0 < 1,
  0 > 1,
  0 <= 1,
  0 >= 1,
  {
    x -= 0;
  },
  {
    x *= 0;
  },
  {
    x /= 0;
  },
  {
    x &= 0;
  },
  {
    x %= 0;
  },
  {
    x ^= 0;
  },
  {
    x += 0;
  },
  {
    x <<= 0;
  },
  {
    x <<= 0;
  },
  {
    x >>= 0;
  },
  {
    x >>= 0;
  },
  {
    x |= 0;
  },
  A::<1>::B(),
  A::<1>::B {},
  A::<1>(),
  A::<1> {},
  {
    {
    }
    2
  },
  &mut [0; 1][..],
  &B::<T>::A[0],
  &B::<T>::A.0[0],
  &B::<T>::A.0.1[0],
  [[0; 1]; 1],
  std::ptr::null::<usize>().is_null(),
  &ss.1,
  &raw mut foo.x.0..1,
  &mut **d,
  [12, 34][0 + 1],
  g(f())(()),
];

fn f() {
  s.e()
    .f(E::s)
    .f(|f| f.a())
    .f(R::e)
    .e(|a| *a >= q)
    .d()
}
fn f() {
  let q = E { r: f![] };
  Q(Q(q)).s(|d|
    q.i(|mut d| {
      e.z(0);
      f.G = e;
      r
    })
  );
}
pub fn public_expr(_: [u8; a(0).0]) {}
pub fn f() {
  return ::f();
}
fn f() -> isize {
  (
    {
      return 1;
    },
    {
      return 2;
    },
  )
}
fn f(x: Box<isize>) -> Box<(Box<isize>, Box<isize>)> {
  box (x, x)
}
fn f<F>(f: F) -> isize where F: FnOnce(isize) -> isize {}
fn f() {
  if (
    {
      return;
    }
  ) {
  }
}
fn f() {
  b! {}
  c
}
fn f<T: ToString>(arg: T) -> String {
  return <T as ToString>::to_string(&arg);
}
fn f<A: Clone + 'static>(a: A, b: u16) -> Box<dyn Invokable<A> + 'static> {
  box (Invoker { a: a, b: b }) as Box<dyn Invokable<A> + 'static>
}
fn f() {
  (
    {
      return 1;
    },
    {
      return 2;
    },
  )
}
pub trait Foo: Iterator<Item = <Self as Foo>::Key> {}
fn f() {
  ::m!(S, x);
}

// source: "../../../ext/jinx-rust/tests/samples/expressions/expr.rs"