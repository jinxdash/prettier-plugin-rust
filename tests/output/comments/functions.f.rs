fn a(/* _______ */) {} // _______
fn b() {} // _______
fn c(/* _______ */ argA, argB, argC) {} // _______
fn a(a /*_______*/) {}
fn b(a /*_______*/) {}
fn d(a /*_______*/, b /*_______*/, c /*_______*/, d /*_______*/) {}
fn d(a /*_______*/, b /*_______*/, c /*_______*/, d /*_______*/) {/*_______*/}
// prettier-ignore
fn c(a /*_______*/
) {}
// prettier-ignore
fn d(
  a /*_______*/,
  b /*_______*/,
  c /*_______*/,
  d /*_______*/
) {}
// prettier-ignore
fn e(
  a /*_______*/,
  b /*_______*/,
  c /*_______*/,
  d /*_______*/
) {} /* _______*/

fn f1 /* _______ */() {}
fn f2(/* _______ */) {}
fn f3() {/* _______ */}
fn f4 /* _______ */(/* _______ */) {/* _______ */}
fn f5(/* _______ */ /* _______ */ a) {}
fn f6(/* _______ */ a /* _______ */) {}
fn f7(/* _______ */ /* _______ */ a) {/* _______ */}
// source: "../../samples/comments/functions.rs"