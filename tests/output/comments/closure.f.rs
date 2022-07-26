call(|/*_______*/ row| {});
KEYPAD_NUMBERS.map(
  |num|
    // _______
    1
);

const obj = A {
  f1: /* _______ */ || {},
  f2: |/* _______ */| {},
  f3: || /* _______ */ {},
  f4: /* _______ */ |/* _______ */| /* _______ */ {},
};

/* _______ */ (|| {})();
(|/* _______ */| {})();
(|| /* _______ */ {})();
/* _______ */ (|/* _______ */| /* _______ */ {})();

let commented = |
  /* first */ a /*argument*/,
  /* second*/ b: WithType /* argument*/,
  /* ignored */ _
| (
  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,
);

const fn1 = |/*_______, _______*/| doSomething();
const fn2 = |/*_______, _______*/| doSomething(anything);

foo(
  |
    // _______
  | {}
);

const rootEpic = |actions, store|
  combineEpics(epics)(actions, store)
    // _______
    .catch(|err, stream| {
      getLogger().error(err);
      return stream;
    });
// source: "../../samples/comments/closure.rs"