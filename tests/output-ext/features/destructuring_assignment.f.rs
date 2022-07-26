#![feature(destructuring_assignment)]

fn main() {
  _ = 1;
  _ = DropRecorder(1);
  _val = DropRecorder(2);
  (_,) = (1, 2);
  (.., a) = (1, 2);
  (..) = (3, 4);
  ((a, b), c) = ((2, 3), d);
  ((a, .., b), .., ..) = ((4, 5), ());
  (a, b) = (0, 1);
  (*foo(&mut x), *foo(&mut x)) = (5, 6);
  (a, _) = (8, 9);
  (a, .., b) = (1, 2);
  (a, a, b) = (1, 2);
  (a, b) += (3, 4);
  (a, b) = (0, 1);
  (a, b) = (3, 4);
  (b, ..) = (5, 6, 7);
  (b, a) = (a, b);
  (C, ..) = (0, 1);
  (c, d) = ("c".to_owned(), "d".to_owned());
  (d, c) = (c, d);
  test() = TupleStruct(0, 0);
  (x, _) = (DropRecorder(3), DropRecorder(4));
  [_, a, _] = [1, 2, 3];
  [_] = [1, 2];
  [..] = [1, 2, 3];
  [a, .., b, ..] = [0, 1];
  [a, .., b, c] = [1, 2, 3, 4, 5];
  [a, a, b] = [1, 2];
  [a, b] += [3, 4];
  [a, b] = [0, 1];
  [a, b] = [3, 4];
  [c, ..] = [5, 6, 6];
  <Alias<isize> as Test>::test() = TupleStruct(0, 0);
  Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);
  Enum::SingleVariant(_) = Enum::SingleVariant(1, 2);
  Enum::SingleVariant(a, .., b, ..) = Enum::SingleVariant(0, 1);
  Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);
  Enum::SingleVariant(a, b) = Enum::SingleVariant(7, 8);
  (S { x: a, ..s } = S { x: 3, y: 4 });
  (S { x: a, y: b } += s);
  (S { x: a, y: b } = s);
  (Struct { .. } = Struct { a: 1, b: 4 });
  (Struct { a, .. } = Struct { a: 1, b: 3 });
  Struct { a, .. };
  (Struct { a, ..d } = Struct { a: 1, b: 2 });
  (Struct { a, b } = Struct { a: 0, b: 1 });
  (Struct { a, b, c } = Struct { a: 0, b: 1 });
  (Struct { a: _, b } = Struct { a: 1, b: 2 });
  (Struct { a: b, b: a } = Struct { a: 1, b: 2 });
  (Struct { a: TupleStruct((a, b), c), b: [d] } = Struct {
    a: TupleStruct((0, 1), 2),
    b: [3],
  });
  test() = TupleStruct(0, 0);
  TupleStruct(_, a) = TupleStruct(2, 2);
  TupleStruct(_) = TupleStruct(1, 2);
  TupleStruct(..) = TupleStruct(3, 4);
  TupleStruct(5, 6).assign(&mut a, &mut b);
  TupleStruct(a, .., b, ..) = TupleStruct(0, 1);
  TupleStruct(a, .., b) = TupleStruct(1, 2);
  TupleStruct(a, a, b) = TupleStruct(1, 2);
  TupleStruct(a, b) = TupleStruct(0, 1);
}
// source: "../../../ext/jinx-rust/tests/samples/features/destructuring_assignment.rs"