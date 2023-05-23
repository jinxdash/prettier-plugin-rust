const baseline = 1 + 1;

fn no() {
  a(  a  ); #![rustfmt::skip]
  // _______
}

#[rustfmt::skip]
const a    =   A {
  b:
          "_______",
};

fn   f()    {
	1    +  1;
	#[no   ]
	#![rustfmt::skip]
	fn f() {
	  1     + 1;
	}
}

// source: "../../samples/comments/ignore.attr.rs"