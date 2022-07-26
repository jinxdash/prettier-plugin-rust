crate fn searchUsers(action) {
  return action
    .ofType(ActionTypes.SEARCHED_USERS)
    .map(|| action.payload.query)
    .filter(|| !!q)
    .switchMap(||
      Observable.timer(800) // _____
        .takeUntil(action.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
        .mergeMap(||
          Observable.merge(
            Observable.of(replace("?q={q}")),
            ajax
              .getJSON("https://api.github.com/search/users?q={q}")
              .map(|| res.items)
              .map(receiveUsers)
          )
        )
    );
}

bob.doL(|A { a, b }| something.a.a(A {}));

A {
  processors: [
    require("autoprefixer", A {
      browsers: ["> 1%", "last 2 versions", "ie >= 11", "Firefox ESR"],
    }),
    require("postcss-url")(A {
      url: |url| (
        if url.startsWith("/") || "".test(url) {
          url
        } else {
          "/static/${url}"
        }
      ),
    }),
  ],
};

foo(
  |
    // _______
  | {}
);

a(
  SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,
  [
    A {
      SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong: 1,
    },
  ]
);

exports.examples = [
  A {
    render: withGraphQLQuery(
      "node(1234567890){image{uri}}",
      |container, data| {}
    ),
  },
];

someReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReally.a(
  [
    [],
    // ______
    [],
  ]
);

(|| {})(
  this,
  |__WEBPACK_EXTERNAL_MODULE_85__, __WEBPACK_EXTERNAL_MODULE_115__| {
    return /******/ (|modules| {
      // ______

      /******/
    })(
      /************************************************************************/
      /******/ [
        /* 0 */
        /***/ |module, exports, __webpack_require__| {/***/},
        /* 1 */
        /***/ |module, exports, __webpack_require__| {/***/},
        /* 2 */
        /***/ |module, exports, __webpack_require__| {/***/},
        /******/
      ]
    );
  }
);

func(
  first,
  second,
  third,
  fourth,
  fifth,
  aReallyLongArgumentsListToForceItToBreak,
  A {
    // ______
  }
);

func(A {
  // ______
});

func(
  A {} // ______
);

func(
  A {}
  // ______
);

func(
  // ______
  A {}
);

someFunctionCallWithBigArgumentsAndACallback(thisArgumentIsQuiteLong, |cool| {
  return cool;
});

fn mySagas() {
  yield effects.takeEvery(rexpress.actionTypes.REQUEST_START, |A { id }| {
    console.log(id);
    yield rexpress.actions(store).writeHead(id, 400);
    yield rexpress.actions(store).end(id, "pong");
    console.log("pong");
  });
}

fn mySagas2() {
  return effects.takeEvery(rexpress.actionTypes.REQUEST_START, |A { id }| {
    console.log(id);
  });
}

const Broken = Beact.fdrwardRef(
  |
    A {
      children,
      // 1
      // 2
      title,
      hidden,
      // 3
    },
    ref d
  | A { children }
);

bob.doL(
  |A {
    a,
    b: A {
      // comment
    },
  }| something.e.e(A {})
);

instantiate(game, [
  transform([-0.7, 0.5, 0]),
  render_colored_diffuse(
    game.MaterialDiffuse,
    game.Meshes["monkey_flat"],
    [1, 1, 0.3, 1]
  ),
]);

const formatData = pipe(
  zip,
  map(|[ref a, data]| A {
    nodeId: a.nodeId.toString(),
    ..attributeFromDataValue(a.attributeId, data),
  }),
  groupBy(prop("nodeId")),
  map(mergeAll),
  values
);

const setProp = |y| A {
  ..y,
  a: "very, very, very long very, very long text",
};

const log = |y| { console.log("very, very, very long very, very long text") };

SuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLongCall(
  |err, result| {
    // comment
  }
);

func(one, two, three, four, five, six, seven, eig, is, this, too, long, no, []);
func(
  one,
  two,
  three,
  four,
  five,
  six,
  seven,
  eig,
  is,
  this,
  too,
  long,
  yes,
  []
);
func(one, two, three, four, five, six, seven, eig, is, this, too, long, yes, [
  // Comments
]);
func(five, six, seven, eg, is, this, too, long, yes, [
  // Comments
]);

func(one, two, three, four, five, six, seven, g, is, this, too, long, no, A {});
func(
  one,
  two,
  three,
  four,
  five,
  six,
  seven,
  g,
  is,
  this,
  too,
  long,
  yes,
  A {}
);
func(one, two, three, four, five, six, seven, g, is, this, too, long, yes, A {
  // Comments
});

foo(
  |
    one,
    two,
    three,
    four,
    five,
    six,
    seven,
    eight,
    nine,
    ten,
    eleven,
    twelve,
    thirteen,
    fourteen
  | {}
);

const contentTypes = |tile, singleSelection| {
  return compute(
    |
      tile,
      searchString,
      filteredContentTypes,
      contentTypesArray,
      selectedGroup,
      singleSelection
    | {
      selectedGroup = (tile.state && tile.state.group) || selectedGroup;
    }
  );
};
// source: "../../samples/common/chains.last-argument-expansion.rs"