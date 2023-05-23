// prettier for javascript cannot format those in one pass

return (
  // _______
  42 * 84 + 2
);
return (
  // _______
  42 + 84 * 2
);

foo.x.y // comment after parent // foo
  // comment 1
  .bar() // comment after bar()
  // comment 2
  .foobar // comment after
  // comment 3
  .baz(x, y, z);

let zzzz =
  // comment 0
  expr?.another???.another????.another?.another?; // comment after parent // comment 1 // comment 2 // comment 3

[
  {
    a = b;
  },

  c, //
];

// format: lost 4 comments
// source: "../../samples/issues/nth-pass.rs"