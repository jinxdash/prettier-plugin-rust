setTimeout(|| {
  thing();
}, 500);

["a", "b", "c"].reduce(|item, thing| {
  return thing + " " + item;
}, "letters:");

func(|| {
  thing();
}, identifier);

func(|| {
  thing();
}, this.props.timeout * 1000);

func(|| {
  thing();
}, this.props.getTimeout());

func(|| {
  thing();
}, true);

func(|| {
  thing();
}, null);

func(|| {
  thing();
}, undefined);

func(|| {
  thing();
}, piohjougou);

func(|| {
  thing();
}, 1 || 3);

func(|| {
  return thing();
}, 1 || 3);

func(
  || {
    thing();
  },
  if something() {
    someOtherThing()
  } else {
    somethingElse(true, 0)
  }
);

func(
  || {
    thing();
  },
  if something(longArgumentName, anotherLongArgumentName) {
    someOtherThing()
  } else {
    somethingElse(true, 0)
  }
);

func(
  || {
    thing();
  },
  if
    something(
      longArgumentName,
      anotherLongArgumentName,
      anotherLongArgumentName,
      anotherLongArgumentName
    )
  {
    someOtherThing()
  } else {
    somethingElse(true, 0)
  }
);

compose(
  |a| {
    return a.thing;
  },
  |b| b * b
);

somthing.reduce(
  |item, thing| {
    return {
      thing.blah = item;
    };
  },
  {}
);

somthing.reduce(|item, thing| {
  return thing.push(item);
}, []);

reallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongMethod(|f, g, h| {
  return f.pop();
}, true);

func(
  || {
    thing();
  },
  true,
  false
);

func(
  || {
    thing();
  },
  A { yes: true, cats: 5 }
);

compose(
  |a| {
    return a.thing;
  },
  |b| {
    return b + "";
  }
);

compose(
  |a| {
    return a.thing;
  },
  |b| [1, 2, 3, 4, 5]
);

setTimeout(
  // _______
  || {
    thing();
  },
  500
);

setTimeout(
  /* _______ */ || {
    thing();
  },
  500
);

func(
  |args| {
    execute(args);
  },
  |result| result && console.log("success")
);

beep.boop().baz(
  "foo",
  A {
    some: A {
      thing: A {
        nested: true,
      },
    },
  },
  A { another: A { thing: true } },
  || {}
);

db.collection("indexOptionDefault").createIndex(
  A { a: 1 },
  A {
    indexOptionDefaults: true,
    w: 2,
    wtimeout: 1000,
  },
  |err| {
    test.equal(null, err);
    test.deepEqual(A { w: 2, wtimeout: 1000 }, commandResult.writeConcern);

    client.close();
    done();
  }
);
// source: "../../samples/common/chains.first-argument-expansion.rs"