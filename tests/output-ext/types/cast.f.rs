fn a() {
  if (5u64 as i32 as u16) == 0u16 {
  }
  [
    (u64 as u8 as i8) == 9i8,
    &[1, 2, 3] as *const _ as *const [i32; 3],
    -0i16 as i8,
    !0u16 as u8,
    (0u16 << 15) as u8,
    (0u32 << 31) as u16,
    Foo::Bar as i8,
    0 as i32 as i32,
    // 0 as i32: i32,
    // 0i32: i32 as i32,
    // 0i32: i32: i32 as u32 as i32,
    // 0i32: i32: i32,
    0u8 as u32,
    a as fn(u8),
    // <T as Foo>::Assoc<3>,
    drop as fn(u8),
    &x as *const _,
    Box::new(A) as &dyn B<C = usize>,
    box (move |y: i32| -> i32 { x + y }) as Box<
      dyn (FnMut(i32) -> i32) + 'static
    >,
    &x as *const i16 as f32,
    &(|_| ()) as &dyn for<'x> Fn(<u32 as T<'x>>::V),
    TestStruct { x: 0x1234 as *const [isize; 2] },
    !(FOO as *const usize).a(),
    !(42 as *const usize).a(),
    (0 as *const usize).a(),
    !("foo" as *const str).a(),
    (&x as T)[0],
  ]
}
const A: *const u8 = &0 as *const _ as *const Q as *const u8;

// source: "../../../ext/jinx-rust/tests/samples/types/cast.rs"