enum E {}

enum E {
  Foo {
    limb_with_align16: Align16,
  },
  Bar,
}
enum E {
  Foo {
    foo: u32,
  },
  Bar {
    bar: u32,
  },
}

enum A {
  Ok = u8::MAX - 1,
  Ok2 = -1,
  OhNo = u8::MIN,
  Bi64 = 0x8000_0000,
  orange = 8 >> 1,
}
enum A {
  union,
}
enum B {
  union {},
}
enum C {
  union(),
}

enum E {
  A = {
    enum F {
      B,
    }
    0
  },
}
enum E<T> {
  _None,
  _Some(T),
}
enum E<W: ?Sized, X: ?Sized, Y: ?Sized, Z: ?Sized> {
  EM(W),
  EM {
    x: X,
  },
  EM(isize, Y),
  EM {
    u: isize,
    x: Z,
  },
  EM([u8]),
  EM {
    x: str,
  },
  EM(isize, [f32]),
  EM {
    u: isize,
    x: [u32],
  },
  EM(Path1),
  EM {
    x: Path2,
  },
  EM(isize, Path3),
  EM {
    u: isize,
    x: Path4,
  },
  EM(dyn Foo),
  EM {
    x: dyn Bar,
  },
  EM(isize, dyn FooBar),
  EM {
    u: isize,
    x: dyn BarFoo,
  },
  EM(<&'static [i8] as Deref>::Target),
  EM {
    x: <&'static [char] as Deref>::Target,
  },
  EM(isize, <&'static [f64] as Deref>::Target),
  EM {
    u: isize,
    x: <&'static [i32] as Deref>::Target,
  },
}
enum E<'a, 'b, 'c: 'b> {
  A(extern "Rust" fn(&'a isize)),
  B(&'b [isize]),
  C(&'b mut &'c str),
}

pub enum X<D> where D: Copy + Debug + Eq {}
// source: "../../../ext/jinx-rust/tests/samples/statements/enum.rs"