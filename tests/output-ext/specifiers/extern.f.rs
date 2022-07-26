type funky_func = extern "C" fn(
  unsafe extern "rust-call" fn(
    *const JSJitInfo,
    *mut JSContext,
    HandleObject,
    *mut libc::c_void,
    u32,
    *mut JSVal
  ) -> u8
);
extern "C" fn sup() {}
extern "C" {
  fn some_fn() -> ();
}
extern {
  fn quux() -> (); // Post comment
  fn syscall(
    number: libc::c_long /* comment 1 */,
    /* comm 2 */ ... /* sup? */
  ) -> libc::c_long;
  unsafe fn foo() -> *mut Bar;
  pub(super) const fn foo() -> *mut Bar;
  pub(crate) unsafe fn foo() -> *mut Bar;
}
// source: "../../../ext/jinx-rust/tests/samples/specifiers/extern.rs"