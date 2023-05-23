use std::{/* comment */};

macro_rules! m {/* comment */}
macro_rules! m {
  /* comment */ /* comment */ (/* comment */) => {/* comment */};
}

macro m/* comment */ (/* comment */) {/* comment */}

{/* comment */}

f(/* comment */);

f /* comment */!(/* comment */);
f /* comment */! {/* comment */}
f /* comment */![/* comment */];

f!([/* comment */]);
f!({/* comment */});
f!((/* comment */));

f!(~[/* comment */]);
f!(~{/* comment */});
f!(~(/* comment */));

[/* comment */];
(/* comment */);
A /* comment */ {/* comment */};

if let A {/* comment */} | a(/* comment */) | [/* comment */] = (/* comment */) {
}

if 0 {/* comment */}
if 0 {/* comment */} else {
  /* comment */
  /* comment */
  /* comment */
}
if 0 {/* comment */} else if /* comment */ /* comment */ 0 {/* comment */}
if 0 {/* comment */} else if
  /* comment */ /* comment */ /* comment */ let _ = 0
{/* comment */}

match (/* comment */) {/* comment */}

fn a(/* comment */) {
  /* comment */
  /* comment */
}

|/* comment */| 0;

impl A {/* comment */}
trait A {/* comment */}
enum A {/* comment */}
struct A(/* comment */);
struct A {/* comment */}
mod A {/* comment */}
use A::{/* comment */};
union A {/* comment */}

A::</* comment */>;
A::</* comment */>(/* comment */);

type A</* comment */>
  : fn(/* comment */)
  where
    for</* comment */> A</* comment */>: for</* comment */> Fn(/* comment */) -> (/* comment */);

#![/* comment */]
#[/* comment */]
struct A;

#![/* comment */]
#[/* comment */]
struct A;

#![/* comment */]
#[/* comment */]
#[/* comment */]
struct A;

#![/* comment */]
#![/* comment */]
#[/* comment */]
struct A;

#![/* comment */]
#![/* comment */]
#[/* comment */]
#[/* comment */]
struct A;

#![/* comment */]
#![/* comment */]
#[/* comment */]
#[/* comment */]
struct A;

// source: "../../samples/comments/dangling.rs"