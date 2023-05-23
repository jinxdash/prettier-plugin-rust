fn f() {
  ({ foo.0 }).0 = 0;
  (async {}).await;
  async move {
  }
  ({ a }).0 += { 0 };
  try {
  }
  match (try {}) {
  }
  for lhs in &mut self.0 {
    *lhs += rhs;
  }
  for lhs in self.0.iter_mut() {
  }
  for _ in [1, 2, 3].into_iter() {
  }
  for elt in self {
    r = r + f(*elt);
  }
  for _ in [1, 2] {
  }
  for _ in [1.0, 2.0] {
  }
  if (
    loop {
    }
  ) {
  }
  if let 0 = 1 {
  }
  if a % 5 == 0 {
  }
  let x: A = if a % 5 == 0 {
  };
  let x = if let 0 = 1 {
  };
  if let 0 = 1 {
    3;
  }
  if (
    {
      y = Foo { foo: x };
    }
  ) {
  }
  if q == "" {
  }
  if ('x' as char) < ('y' as char) {
  } else {
  }
  let a = if 1 {
    1
  };
  let a = if 1 { 0 } else if 1 { 0 };
  let a = if (if 0 { 0 } else { 0 }) {
    0
  } else {
    0
  };
  let a = if 0 {
    if 0 { 0 } else { 0 }
  } else {
    0
  };
  let a = if 0 { 0 } else if 0 { 0 } else { 1 };
  let a = if let 0 = (if let 0 = 0 { 0 } else { 0 }) { 0 } else { 0 };
  for x in 0..10 {
    (async { Some(x) }).await.unwrap();
  }
  for _ in 1.. {
    call_forever();
  }
  unsafe {
    (Foo { b: () }).a;
  }
  if 1 {
  } else if let Some(a) = Some(1) {
  }
}
[
  m::Pub {
    0: loop {
    },
  },
  2_usize +
    (loop {
    }),
  [
    ();
    &({
      loop {
        continue;
      }
    })
  ],
  [
    ();
    loop {
      break;
    }
  ],
  [
    ();
    {
      while true {
        break;
      }
      0
    }
  ],
  [
    ();
    {
      for _ in 0usize.. {
      }
      0
    }
  ],
  unsafe { *&raw mut y },
];

a::b(async move {
  if let Err(e) = c(d).await {
    f!("g: {}", h);
  }
});

// source: "../../../ext/jinx-rust/tests/samples/expressions/block.rs"