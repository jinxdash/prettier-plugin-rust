// a

/// b
fn a() {}

fn b() {
  //! c
}

//////////////////////////////////
// d
/// let heart = '❤️';
//////////////////////////////////
/// e
/// f
fn c() {}

/*
 * g
 */

/**
 * h
 */
fn d() {}

fn e() {
  /*!
   * i
   */
  //! ```a
  //! b
  //! ```
}

/********************************/
/*
 * j
 */

/********************************/
/**
 * k
 */
fn f() {}

// before
#[macro_use]
// after
extern crate x;

a! {
  /// line 2
  /// line 3
  #[a(b, c(d(e(f = "g", h = "i"))))]
  pub enum X {
    /// line 6
    /// line 7
    A(B),

    /// line 10
    C(D),
  }
}
// 5 comments inside a!{}

/// ____
/// ____
#[attr_0]
pub type A = Vec<B>;

#[attr_1]
extern crate b as d;

#![attr_2]
#![attr_3]
/**
 * directly below attr_2 and 1 line above attr_3
 */

fn attr_3_3a_target() {
  #![attr_3a]
  #[attr_3b]
  fn attr_3b_target() {}

  #[attr_3c]
  fn attr_3c_target() {}
}

trait foo_C {
  #![attr_4]
}

#![attr_6]
#[attr_5]
fn main() {
  // comment
  #[attr]
  ();
  #[attr]
  [1; 4];
  #[attr]
  Foo { data: () };
  #[attr]
  if let Some(_) = Some(true) {
  }
  #[attr]
  if let Some(_) = Some(true) {
  } else if let Some(false) = Some(true) {
  }
  #[attr]
  if true {
  }
  #[attr]
  let _ = 0;
  #[attr]
  if true {
  } else if false {
  } else {
  }
  {
    #![attr]
    foo();
  }
  #[attr]
  if true {
  }
  #[attr]
  (0, 1);
  #[attr]
  (0,);
  #[attr]
  0;
  #[attr]
  [1, 2, 3];
  #[attr]
  {
    #![attr]
  }
  #[attr]
  {
    foo();
  }
  #[attr]
  0;
  #[attr]
  expr_mac!();
  #[attr]
  foo();
  #[attr]
  let x = 1;
  #[attr]
  match () {
    _ => {}
  }
  #[attr]
  match () {
    #![attr]
    _ => (),
  }
  #[attr]
  unsafe {/**/}
  ||
    #[attr] {
      return;
    };
  let a = #[attr] [1; 4];
  let a = #[attr] box 0;
  let a = #[attr] [1, 2, 3];
  let a = #[attr] Foo { data: () };
  let a = (#[attr] s).data;
  let a = (#[attr] t).0;
  let a = (#[attr] v)[0];
  let a = #[attr] -0i32;
  let a = #[attr] !0;
  let a = #[attr] 'c';
  let a = #[attr] ..;
  let a = #[attr] ..0;
  let a = #[attr] (#[attr] 0,);
  let a = #[attr] (#[attr] 0, 0);
  let a = #[attr] ();
  let a = #[attr] (0, 0);
  let a = #[attr] (0,);
  let a = #[attr] 0..;
  let a = #[attr] 0..0;
  let a = #[attr] 0;
  let a = #[attr] 0;
  let a = #[attr] [0, 0];
  let a = #[attr] [0; 0];
  let a = #[attr] { #![attr] };
  let a = #[attr] {
    #![attr]
    let _ = ();
    ()
  };
  let a = #[attr] {
    #![attr]
    let _ = ();
  };
  let a = #[attr] &(#[attr] 0);
  let a = #[attr] &0;
  let a = #[attr] &mut (#[attr] 0);
  let a = #[attr] &mut 0;
  let a = #[attr] || {
    #![attr]
    #[attr]
    ()
  };
  let a = #[attr] || (#[attr] ());
  let a = #[attr] 0 + (#[attr] 0);
  let a = #[attr] 0 as usize;
  let a = #[attr] 0;
  let a = #[attr] 0..;
  let a = #[attr] 0..(#[attr] 0);
  let a = #[attr] (1i32).clone();
  let a = #[attr] x! {};
  let a = #[attr] x!();
  let a = #[attr] x![];
  let a = #[attr] false;
  let a = #[attr] x();
  let a = #[attr] x!();
  let a =
    #[attr] x!(
      #![attr]
    );
  let a = #[attr] x![];
  let a =
    #[attr] x![
      #![attr]
    ];
  let a = #[attr] x! {};
  let a = #[attr] x! {
    #![attr]
  };
  let a = #[attr] Foo { ..s };
  let a = #[attr] Foo { data: (), ..s };
  let a = #[attr] Foo { data: () };
  let a = #[attr] for _ in 0..0 { #![attr] };
  let a = #[attr] loop { #![attr] };
  let a = #[attr] match 0 {
    #![attr]
    () => (),
  };
  let a = #[attr] match 0 {
    #![attr]
    _ => (),
  };
  let a = #[attr] move || {
    #![attr]
    #[attr]
    ()
  };
  let a = #[attr] move || (#[attr] ());
  let a = #[attr] s.data;
  let a = #[attr] t.0;
  let a = #[attr] v[0];
  let a = #[attr] while false { #![attr] };
  let a = #[attr] while let None = Some(()) { #![attr] };
  let a = #[attr] {
    x += 15;
  };
  let a = #[attr] {
    x += 15;
  };
  let a = #[attr] {
    x = 15;
  };
  let a = #[attr] {
    x = 15;
  };
  let a: [(); 0] = #[attr] [];
  let a: fn(&u32) -> u32 = #[attr] std::clone::Clone::clone;
  let a = #[attr] 1;
  let a = |
    #[allow(C)] a: u32,
    #[cfg(something)] b: i32,
    #[cfg_attr(something, cfg(nothing))] #[deny(C)] c: i32
  | {};
  qux(3 + (#[attr] 2));
  foo3(x, #[attr] y, z);
  while false {
    let _ = #[attr] {
      continue;
    };
  }
  while true {
    let _ = #[attr] {
      break;
    };
  }
  match (Q { #[attr] C: 1 }) {
    Q { #[attr] C } => {}
    Q(#[attr] C) => {}
    #[attr]
    _ => {}
  }
}
static X: &[Y] = &[#[a(b = "c")] 0];
#[attr]
const C: C = C { #[attr] field: 0, #[attr] field: 1 };
#[attr]
struct S;
#[attr]
struct I {
  #[attr] i: u8,
  #[attr] pub i: u8,
}
#[attr]
struct I(#[attr] u8, #[attr] pub u8);
#[attr]
struct BreaksWithComment(
  #[attr] u8,
  #[attr] // comment
  pub u8,
);
struct C {
  #[attr]
  /// below attr
  a: b,
}
struct Q {
  #[attr] C: i32,
}
struct A<#[attr] 'a>();
struct A<#[attr] I>(I);
enum E {
  #[attr] C(i32),
}
enum E<#[attr] 'b> {}
enum E<#[attr] J> {}
trait T {
  #![attr]
}
trait T<#[attr] 'c> {}
trait T<#[attr] K> {}
type Y<#[attr] 'd> = ();
type Y<#[attr] L> = ();
#[attr]
type A = fn(#[a1] u8, #[a2] ...);
impl<#[attr] 'e> A<'e> {}
impl<#[attr] M> A<M> {}
impl<#[attr] 'f> T<'f> for A<'f> {}
impl<#[attr] N> T<N> for A<N> {}
#[attr]
#[attr]
macro_rules! m_e {}
#[attr]
macro_rules! m {}

#[attr]
fn f() {}
#[attr]
fn f(#[a1] a: u8) {
  let f = |#[a2] W(x), #[a3] y| ();
}
fn f<#[attr] 'g>() {}
fn f<'e, #[attr] 'g>() {}
fn f<#[attr] G>() {}
fn f<E, #[attr] G>() {}
fn f() where for<#[attr] 'i> X: for<#[attr] 'i> Y {}
fn f(#[d(true)] a: i32, #[a2] b: i32, #[what = "how"] c: u32) {}
fn f(#[a1] #[a2] a: i32, #[what = "how"] b: i32, #[e(true)] c: u32) {}
fn b(#[cfg(x)] x: i32, y: i32) -> i32 {}
fn f(
  #[a1] #[a2] &self,
  #[a1] #[a2] a: i32,
  #[what = "how"] b: i32,
  #[f(true)] c: u32
) {}
fn c(#[cfg(foo)] &mut self, #[deny(C)] b: i32) {}

#[a = "q"]
#[b = "q"]
mod a {
  #![c = "q"]
}
#[a = "q"]
pub static X: u8 = ();
#[a = "q"]
pub fn f() {}
#[a = "q"]
pub mod a {}
#[a = "q"]
extern "C" {}

//(#[b(c(#(d = #e),*))]);
a!(#[b(c(#(d = #e),*))]);

//(#![b(c(#(d = #e),*))]);
a!(
  #![b(c(#(d = #e),*))]
);

#[attr]
extern "C" {
  fn ffi(#[a1] a: i32, #[a2] ...);
}
#[attr]
unsafe extern "C" fn f(a: i32, #[a1] mut args: ...) {}

impl W {
  #[attr]
  fn f(#[a1] self, #[a2] a: u8) {}
  #[attr]
  fn f(#[a1] &self, #[a2] a: u8) {}
  #[attr]
  fn f<'a>(#[a1] &'a mut self, #[a2] a: u8) {}
  #[attr]
  fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8) {}
  #[attr]
  fn f(#[a1] #[a2] a: u8, #[a3] b: u8) {}
}

trait A {
  #[attr]
  fn f(#[a1] self, #[a2] a: u8);
  #[attr]
  fn f(#[a1] &self, #[a2] a: u8);
  #[attr]
  fn f<'a>(#[a1] &'a mut self, #[a2] a: u8);
  #[attr]
  fn f<'a>(#[a1] self: Box<Self>, #[a2] a: u8, #[a3] b: Vec<u8>);
  #[attr]
  fn f(#[a1] #[a2] a: u8, #[a3] b: u8);
}

// directly above #![doc(...
#![doc(
  ___ = "____________________________________________________________",
  ___________________ = "_________________________________________________",
  ______________ = "_________________________________________________",
  ___________________ = "_________________________________________________",
  a(b(c(d)))
)]

//! 1 line below #![doc(...

#![attr]

//! aaa

// bbb

// ccc
#![attr(b)]

// ddd

/// eee
/// eee
/// eee
/// eee

/// fff
impl Bar {
  /// 0
  /// 1
  /// 2
  /// 3
  #[attr]
  #[doc = " ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ ___ "]
  fn foo(&mut self) -> isize {}

  /// \n 4
  /// 5
  /// 6

  /// \n\n 7
  /// 8
  /// 9
  pub fn f2(self) {
    (foo, bar)
  }

  #[attr]
  fn f3(self) -> Q {}

  /// \n 10 \n

  #[attrib1]
  /// 11
  #[attrib2]
  // 12
  /// 13
  fn f4(self) -> A {}

  // \n 14
  #[a(b = "c")]
  fn f5(self) -> X {}
}

struct Foo {
  #[derive(A, B, C, D, E)]
  foo: usize,
}

// expected_{x} comments inside each attribute

#[should_panic]
#[should_panic(expected_0 = "(")]
#[should_panic(expected_1 = /* ( */ "(")]
#[should_panic(
  /* ((((( */ expected_4 = /* ((((( */ /* ((((( */ "(" /* ((((( */
)]
#[should_panic(
  /* (((((((( */ /*
    (((((((((()(((((((( */
  expected_3 = "("
  // ((((((((
)]
fn f() {}

fn f() {
  #[allow(unreachable_code)] // right of attr
  Some(Err(error));

  #[allow(unreachable_code)]
  // below attr
  Some(Err(error));
}

fn f() {
  #![this_is_an_inner_attribute(foo)]

  foo();
}

impl InnerAttributes() {
  #![this_is_an_inner_attribute(foo)]

  fn f() {}
}

mod InnerAttributes {
  #![this_is_an_inner_attribute(foo)]
}

fn f() {
  // Local
  #[attr(on(local))]
  let x = 3;

  // Item
  #[attr(on(item))]
  use foo;

  // Expr
  #[attr(on(expr))]
  {
  }

  // Semi
  #[attr(on(semi))]
  foo();

  // Mac
  #[attr(on(mac))]
  foo!();
}

#[derive(
  Add,
  Sub,
  Mul,
  Div,
  Clone,
  Copy,
  Eq,
  PartialEq,
  Ord,
  PartialOrd,
  Debug,
  Hash,
  Serialize,
  Mul
)]

///

#[derive(
  Add,
  Sub,
  Mul,
  Div,
  Clone,
  Copy,
  Eq,
  PartialEq,
  Ord,
  PartialOrd,
  Debug,
  Hash,
  Serialize,
  Deserialize
)]
pub struct HP(pub u8);

//
struct A {
  #[doc = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"] b: i32,
}

#[cfg(
  feature = "this_line_is_101_characters_long_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
)]
pub fn foo() {}

//
#[clippy::bar]
#[clippy::bar(a, b, c)]
pub fn foo() {}

mod v {
  #[derive(Debug, StructOpt)]
  #[structopt(about = "x")]
  pub struct Params {
    #[structopt(help = "x")]
    server: String,
    #[structopt(help = "x")]
    first_name: String,
    #[structopt(help = "x")]
    last_name: String,
    #[structopt(
      short = "j",
      long = "job",
      help = "The job to look at",
      parse(try_from_str)
    )]
    job: Option<Job>,
  }
}

#[cfg(
  not(
    all(
      feature = "std",
      any(
        target_os = "linux",
        target_os = "android",
        target_os = "netbsd",
        target_os = "dragonfly",
        target_os = "haiku",
        target_os = "emscripten",
        target_os = "solaris",
        target_os = "cloudabi",
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "fuchsia",
        windows,
        all(target_arch = "wasm32", feature = "stdweb"),
        all(target_arch = "wasm32", feature = "wasm-bindgen")
      )
    )
  )
)]
type Os = NoSource;

fn stmt_expr_attributes() {
  let foo;
  #[must_use]
  foo = false;
}

fn a() {
  match () {
    #![attr]
  }
  match () {#[attr]}
}

fn x() {
  match MyEnum {
  }
}

// 2 comments inside this attribute
#[derive(
  /* ---------- ------------------------------------------------------------------- --------- */
  Debug,
  Clone,
  /* --------------- */ Eq,
  PartialEq
)]
struct Foo {
  a: i32,
  b: T,
}

// 1 comment inside this attribute
#[derive(/*Debug, */ Clone)]
struct Foo;

#[cfg(
  all(
    any(
      target_arch = "x86",
      target_arch = "x86_64",
      target_arch = "aarch64",
      target_arch = "powerpc64",
      target_arch = "powerpc64le",
      target_arch = "mips64",
      target_arch = "s390x",
      target_arch = "sparc64"
    )
  )
)]
const MIN_ALIGN: usize = 16;
// source: "../../../ext/jinx-rust/tests/samples/macro/attributes.rs"