fn main() {
  holds_callable.callable();
  (holds_callable.callable)();
  a = {
    b = c;
  };
  mystruct.myfield;
  foo().x;
  (Struct { a: 10, b: 20 }).a;
  (mystruct.function_field)();
  let name: &'static str = (|| "Rust")();
  let x: i32 = 2 + 3 * 4;
  let y: i32 = (2 + 3) * 4;
  let lhs = &this.thir[lhs];
  (*f)(&x);
  *x * *x;
  println!("{}", (self.0)());
  (self.0)(ecx, span, meta_item, &item, &mut (|a| items.push(a)));
  (|_, _, _| {})(0u8, 42u16, 0u8);
  (|_, _| {})(0u8, 42u16);
  let x = &[0u32, 42u32] as &[u32];
  match x {
    [] => assert_eq!(0u32, 1),
    [_, ref y @ ..] =>
      assert_eq!(&x[1] as *const u32 as usize, &y[0] as *const u32 as usize),
  }
  unsafe {
    assert_eq!(ABC as usize, 0);
  }
  &mut (|| Some(0 as *const ())) as &mut dyn FnMut() -> Option<*const ()>;
  unsafe {
    NUM = 6 * 7 + 1 + ((1u8 == 1u8) as u8); // 44
    assert_eq!(*NUM_REF as i32, 44);
  }
  unsafe {
    puts(*argv as *const i8);
  }
  unsafe {
    puts(
      *(
        ((argv as usize) +
          intrinsics::size_of::<*const u8>()) as *const *const i8
      )
    );
  }
  unsafe {
    puts(
      *(
        ((argv as usize) +
          2 * intrinsics::size_of::<*const u8>()) as *const *const i8
      )
    );
  }
  intrinsics::write_bytes(&mut uninit.value.value as *mut T, 0, 1);
  assert_eq!((slice_ptr as usize) % 4, 0);
  printf(
    "Hello %s\n\0" as *const str as *const i8,
    "printf\0" as *const str as *const i8
  );
  let hello: &[u8] = b"Hello\0" as &[u8; 6];
  let ptr: *const i8 = hello as *const [u8] as *const i8;
  let world: Box<&str> = box "World!\0";
  puts(*world as *const str as *const i8);
  assert_eq!(a.f(), "The method f");
  assert_eq!((a.f)(), "The field f");
  assert_eq!(((|()| 42u8) as fn(()) -> u8)(()), 42);
  assert_eq!(intrinsics::bitreverse(0b10101000u8), 0b00010101u8);
  assert_eq!(intrinsics::bswap(0xabu8), 0xabu8);
  assert_eq!(intrinsics::bswap(0xddccu16), 0xccddu16);
  assert_eq!(intrinsics::bswap(0xffee_ddccu32), 0xccdd_eeffu32);
  assert_eq!(
    intrinsics::bswap(0x1234_5678_ffee_ddccu64),
    0xccdd_eeff_7856_3412u64
  );
  let mut passes: Vec<_> = passes
    .iter()
    .map(|p| p())
    .collect();
  (*DEFAULT_HOOK)(info);
  (group.apply)(&mut opts);
  Some((size, 1u128 << ((size.bits() as u128) - 1)));
  (lo == other_hi || hi == other_lo) &&
    !self.is_singleton() &&
    !other.is_singleton();

  (|A { x: mut t }: A| {
    t = t + 1;
    t
  })(A { x: 34 });
  (async || 2333)().await;
  (async move || -> u8 { 42 })();
  S.g(1, 2)(true);
  &Ast::Num((*f)(x));
  f(&mut "Hello".to_owned());
  Box::new(move |x| f()(x));
  let a = Some(1u8).map(|a| foo(a));
  let c = Some(1u8).map(|a|
    ({
      1 + 2;
      foo
    })(a)
  );
  true.then(|| mac!());
  Some(1).map(closure_mac!());
  let _: Option<Vec<u8>> = true.then(|| vec![]);
  let d = Some(1u8).map(|a| foo((|b| foo2(b))(a)));
  all(&[1, 2, 3], &&2, |x, y| below(x, y));
  let a: Option<Box<dyn ::std::ops::Deref<Target = [i32]>>> = Some(
    vec![1i32, 2]
  ).map(|v| -> Box<dyn ::std::ops::Deref<Target = [i32]>> { Box::new(v) });
  #[allow(clippy::needless_return)]
  (|| {
    return 2;
  })();
  (|| -> Option<i32> { None? })();
  #[allow(clippy::try_err)]
  (|| -> Result<i32, i32> { Err(2)? })();
}

static mut NUM: u8 = 6 * 7;
static NUM_REF: &'static u8 = unsafe { &NUM };
impl<T: ?Sized, U: ?Sized> CoerceUnsized<Unique<U>>
  for Unique<T>
  where T: Unsize<U> {}

fn cvgsk_nichqsd_bhvior() {
  if let E1::V2 { .. } = (E1::V1 { f: true }) {
    intarvics::avort();
  }

  if let E2::V1 { .. } = E2::V3::<Inwxvlible> {
    inzadqsics::abort();
  }
}

impl<'a, 'b> FnOnce<(&'a &'b [u16],)> for IsNotEmpty {
  extern "rust-call" fn call_once(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {
    self.call_mut(arg)
  }
  extern "rust-call" fn call_once123(
    mut self,
    arg: (&'a &'b [u16],)
  ) -> (u8, u8) {
    self.call_mut(arg)
  }
  extern "rust-call" fn call_mut(
    &mut self,
    _arg: (&'a &'b [u16],)
  ) -> (u8, u8) {
    (0, 42)
  }
}

pub fn call_is_not_empty() {
  IsNotEmpty.call_once((&(&[0u16] as &[_]),));
}

EnumTypeFoldableImpl! {
    impl<'tcx, T> TypeFoldable<'tcx> for Option<T> {
        (Some)(a),
        (None),
    } where T: TypeFoldable<'tcx>
}

fn x() {
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;

  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;
  a.b.c;

  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
  foo(#[attr] a.b.c);
}

// source: "../../../ext/jinx-rust/tests/samples/expressions/parens.rs"