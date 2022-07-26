#![feature(arbitrary_enum_discriminant)]

enum Enum {
  Unit = 1,
  Tuple() = 2,
  Struct {} = 3,
}
#[repr(u8)]
enum Enum {
  Unit = 3,
  Tuple(u16) = 2,
  Struct {
    a: u8,
    b: u16,
  } = 1,
}
#[repr(i8)]
enum E2 {
  A = 7,
  B = -2,
}
#[repr(C)]
enum E3 {
  A = 42,
  B = 100,
}
#[repr(i128)]
enum E4 {
  A = 0x1223_3445_5667_7889,
  B = -0x1223_3445_5667_7889,
}
enum ADT {
  First(u32, u32),
  Second(u64),
}
enum CLike1 {
  A,
  B,
  C,
  D,
}
enum CLike2 {
  A = 5,
  B = 2,
  C = 19,
  D,
}
#[repr(i8)]
enum CLike3 {
  A = 5,
  B,
  C = -1,
  D,
}
enum ADT {
  First(u32, u32),
  Second(u64),
}
enum NullablePointer {
  Something(&'static u32),
  Nothing,
}
#[repr(isize)]
enum Mixed {
  Unit = 3,
  Tuple(u16) = 2,
  Struct {
    a: u8,
    b: u16,
  } = 1,
}
enum MyWeirdOption<T> {
  None = 0,
  Some(T) = std::mem::size_of::<T>(),
}
enum Test {
  A(Box<u64>) = 0,
  B(usize) = (u64::MAX as i128) + 1,
}
pub enum Foo {
  A = 2,
}
pub enum Bar {
  A(Foo),
  B,
  C,
}
pub enum Size {
  One = 1,
  Two = 2,
  Three = 3,
}
#[repr(i128)]
enum Signed {
  Zero = 0,
  Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,
  U64Limit = (u64::MAX as i128) + 1,
  SmallNegative = -1,
  BigNegative = i128::MIN,
  Next,
}
#[repr(u128)]
enum Unsigned {
  Zero = 0,
  Staircase = 0x01_02_03_04_05_06_07_08_09_0a_0b_0c_0d_0e_0f,
  U64Limit = (u64::MAX as u128) + 1,
  Next,
}
// source: "../../../ext/jinx-rust/tests/samples/features/arbitrary_enum_discriminant.rs"