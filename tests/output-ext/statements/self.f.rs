fn f(self) {}
fn f(&self) {}
fn f(mut self) {}
fn f(&mut self) {}
fn f(&'a self) {}
fn f(&'a mut self) {}
fn f(self: u8) {}
fn f(mut self: u8) {}
type X = fn(self);
type X = fn(&self);
// type X = fn(mut self);
type X = fn(&mut self);
type X = fn(&'a self);
type X = fn(&'a mut self);
type X = fn(self: u8);
// type X = fn(mut self: u8);
async fn foo<'b>(self: &'b Foo<'a>) -> &() {
  self.0
}
fn f<'b>(self: &'b Foo<'a>) -> &() {
  self.0
}
fn f<'a>(self: &Alias, arg: &'a ()) -> &() {
  arg
}
fn f(&mut self) -> u32;
fn f(mut self: Box<Self>);
fn f(self: _) {}
fn f(self: &_) {}
fn f(&self) -> Self;
// fn f(self::S: S) {}
// fn g(&self::S: &S) {}
// fn h(&mut self::S: &mut S) {}
// source: "../../../ext/jinx-rust/tests/samples/statements/self.rs"