f1 = |
  //comment
  a 
| {};
  
f2 = |
  a //comment
| {};
  
f3 = |
  a
  //comment
| {};
  
f4 = // Comment
  || {};

f5 =
  
  // Comment
  
 || {}
  
f6 = /* comment */
  
  // Comment
  
  || {}
  
let f4 = // Comment
  || {};
  
let f5 =
  
  // Comment
  
  || {}
  
let f6 = /* comment */
  
  // Comment
  
  || {}

const kochabCooieGameOnOboleUnweave = // ???
      annularCooeedSplicesWalksWayWay;

const bifornCringerMoshedPerplexSawder = // !!!
  glimseGlyphsHazardNoopsTieTie +
  averredBathersBoxroomBuggyNurl -
  anodyneCondosMalateOverateRetinol;

fnNumber =
  // Comment
  3;

fnNumber =

  // Comment

  3;

fnNumber =
  // Comment0
  // Comment1
  3;

fnNumber = /* comment */
  3;

fnNumber = /* comments0 */
  /* comments1 */
  3;

fnNumber =
  // Comment
  3;

let fnNumber =

  // Comment

  3;

let fnNumber =
  // Comment0
  // Comment1
  3;

let fnNumber = /* comment */
  3;

let fnNumber = /* comments0 */
  /* comments1 */
  3;

fnString =
  // Comment
  "some" + "long" + "string";

fnString =
  // Comment

  "some" + "long" + "string";

fnString =

  // Comment

  "some" + "long" + "string";

fnString =
  /* comment */
  "some" + "long" + "string";

fnString =
  /**
   * multi-line
   */
  "some" + "long" + "string";

fnString =
  /* inline */ "some" + "long" + "string" + "some" + "long" + "string" + "some" + "long" + "string" + "some" + "long" + "string";

fnString = // Comment0
  // Comment1
  "some" + "long" + "string";

fnString = // Comment
  "some" + "long" + "string";

fnString =
  // Comment
  "some" + "long" + "string";

let fnString =
  // Comment

  "some" + "long" + "string";

let fnString =

  // Comment

  "some" + "long" + "string";

let fnString =
  /* comment */
  "some" + "long" + "string";

let fnString =
  /**
   * multi-line
   */
  "some" + "long" + "string";

let fnString =
  /* inline */ "some" + "long" + "string" + "some" + "long" + "string" + "some" + "long" + "string" + "some" + "long" + "string";

let fnString = // Comment0
  // Comment1
  "some" + "long" + "string";

let fnString = // Comment
  "some" + "long" + "string";


let obj1 = // 36_______
A {
  key: "val"
}

let obj2 // 37_______
= A {
  key: "val"
}

let obj3 = A { // 38_______
  key: "val"
}

let obj4 = A {
  // 39_______
  key: "val"
}

let obj5 = // 40_______
[
  "val"
]

let obj6 // 41_______
= [
  "val"
]

let obj7 = [ // 42_______
  "val"
]

let obj8 = [
  // 43_______
  "val"
]


const A{ a /* 0_______ */ : 1 } = b;

const A{ c : 1 /* 1_______ */ } = d;

let A{d //2_______
: b} = c

const foo = A {
  a: "a" /* 3_______________ */,

   /* 4_________ */
  b: "b",
};

let // 44_______
  foo1 = "val";

const foo3 = 123
// 45_______
;["2", "3"].forEach(|x| console.log(x))

let a = b || /** 46_______ */
  (c);

let a = A {
	a /* 47_______ */: || 1
};

let a /* 48 */ = 0;

let b /*
       * 4
       * 9
       */ = 0;

let c = /*
       * 5
       * 0
       */ 0;

let d /*
       * 5

       * 1
       */ = 0;
       
fn foo() {
  let x = foo
      .bar??  ? // comment
      .baz;
  let x = foo
      .bar?  ??
  // comment
      .baz;
  let x = foo
      .bar? ? ? // comment
  // comment
      .baz;
  let x = foo
      .bar? ?? // comment
  // comment
      ? ??
  // comment
      ?  ??
  // comment
      ???  
  // comment
      ? ? ?
      .baz;
  let x = try /* Invisible comment */ { foo()? };
  let loooooooooooooooooooooooooooooooooooooooooong = does_this?.look?.good?.should_we_break?.after_the_first_question_mark?;
 
      let x = y // comment
        .z;

    self.rev_dep_graph
        .iter()
       // Remove nodes that are not dirty
        .filter(|&(unit, _)| dirties.contains(&unit))
     // Retain only dirty dependencies of the ones that are dirty
       .map(|(k, deps)| {
            (
                k.clone(),
                deps.iter()
                .cloned()
               .filter(|d| dirties.contains(&d))
              .collect(),
            )
        });

    let y = expr /* comment */.kaas()?;

    (Foo {
        ..// comment
        a
    } = a);
}
