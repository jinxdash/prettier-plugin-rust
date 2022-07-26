// prettier for javascript cannot format those in one pass

return (
	// _______
	42
  ) * 84 + 2;
  return (
	// _______
	42
  ) + 84 * 2;

    foo // foo
        // comment after parent
        .x
        .y
        // comment 1
        .bar() // comment after bar()
  // comment 2
        .foobar
  // comment after
        // comment 3
        .baz(x, y, z);

let zzzz = expr?   // comment after parent
// comment 0
.another??? // comment 1
.another????  // comment 2
.another? // comment 3
.another?;

[
  a = b,
  
  c //
];