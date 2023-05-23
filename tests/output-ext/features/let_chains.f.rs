#![feature(let_chains)]

fn f() {
  if h && let A(x) = e {
  }
  if a && let c = d && 1 == 1 {
  }
  if a && let c = d && 1 == 1 {
  }
  if true && let x = 1 {
    let _ = x;
  }
  if let a = b && let c = d && 1 == 1 {
  }
  if let A { .. } = d(7, B) && let C { .. } = d(8, D) {
  }
  if let a = &e && let A(ref b) = a && let B = b.c {
  }
  if let a = b && let c = d && 1 == 1 {
  }
  if let A(_) = d(2, B(2)).c && let D { .. } = d(3, E) {
  } else {
  }
  if let A(a) = e && let A(b) = a && let A(p) = b && p == z {
  } else {
  }
  if let A(v) = x && v.q() {
  }
  if let A(_) = Q(0) && let E(_) = R(1) {
  } else if let G(1) = F(2) {
  }
  if let A(ref a) = e && let b = a && let _p = b {
  }
  if let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {
  }
  while let a = &e && let A(ref b) = a && let B = b.c {}
  while let A(a) = e && let A(b) = a && let A(p) = b && p == z {}
  while let A(ref a) = e && let b = a && let _p = b {}
  while let A(ref a) = e && let R { c: d, y: _ } = a && let B = d {}
}

// source: "../../../ext/jinx-rust/tests/samples/features/let_chains.rs"