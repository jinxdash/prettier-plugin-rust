[
  if_chain! {
		if let Some(a) = b;
		if let Err(a) = b;
		let (a, b) = c;
		if 1 + 2;
		then { d=0; }
	},
  if_chain! {
		then { d=0; }
	},
  if_chain! {
		then { d=0; }
		else { d!(); }
	},
  if_chain! {
		if let A::B | A::C = D;
		then { 0 } else { 1 }
	},
  if_chain! {
		let Ok(a) | Err(b) = c;
		then { d!(); }
		else { d!(); }
	},
  if_chain! {
		if 1 + 1;
		let a: u32 = 3;

		then { d!(); }
		else { d!(); }
	},
];

// source: "../../samples/macros/if_chain.rs"