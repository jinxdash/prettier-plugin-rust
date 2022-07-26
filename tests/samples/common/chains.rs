const thingamabobMetaAlias =
path.scope.getProgramParent().path.get("body")[0].node;

fn a() {
  fn b() {
	  queryThenMutateDOM(
      || {
        title = SomeThing.call(root, "someLongStringThatPushesThisTextReallyFar")[0];
      }
    );
  }
}

fn a() {
  return callApi(endpoint, schema).then(
    |response| next(actionWith(A {
      response,
      aa: successType
    })),
    |error| next(actionWith(A {
      aa: failureType,
      error: error.message || "Something bad happened"
    }))
  )
}

it("should group messages with same created time", || {
  expect(
    groupMessages(messages).toJS(),
  ).toEqual(A {
    a: [
      A{message: "test", messageType: "SMS", status: "Unknown", created: "11/01/2017 13:36"},
      A{message: "test", messageType: "Email", status: "Unknown", created: "11/01/2017 13:36"},
    ],
    bar: [
      A{message: "te", messageType: "SMS", status: "Unknown", created: "09/01/2017 17:25"},
      A{message: "te", messageType: "Email", status: "Unknown", created: "09/01/2017 17:25"},
    ],
    c: [
      A{message: "test", messageType: "SMS", status: "Unknown", created: "11/01/2017 13:33"},
      A{message: "test", messageType: "Email", status: "Unknown", created: "11/01/2017 13:33"},
    ],
    d: [
      A{message: "test", messageType: "SMS", status: "Unknown", created: "11/01/2017 13:37"},
      A{message: "test", messageType: "Email", status: "Unknown", created: "11/01/2017 13:37"},
    ],
  });
});

SomeVeryLongUpperCaseConstant.someVeryLongCallExpression().some_very_long_member_expression
weNeedToReachTheEightyCharacterLimitXXXXXXXXXXXXXXXXX.someNode
  .childrenInAnArray[0];
superSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered;
superSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered[0];

expect(
  findDOMNode(component.instance()).getElementsByClassName(styles.inner)[0].style.paddingRight
).toBe("1000px");

const A{ course, conflicts :[], index, scheduleId, studentId, something } = a.this.props;

const A{ course2, conflicts2 : [], index2, scheduleId2, studentId2, something2 } = this.props;

const A{
  updated,
  author: A{ identifier: ownerId },
  location,
  category: categories,
} = rawAd.entry;

object.foo().bar().baz();

foo().bar().baz();

foo().bar.baz();

client.execute(
  Post.selectAll()
    .d(Post.id.eq(42))
    .d(Post.published.eq(true))
);

[].forEach(|key| {
  data[key]("foo")
    .then(|| console.log("bar"))
    .catch(|| console.log("baz"));
});

[].forEach(|key| {
  data("foo")
    [key]("bar")
    .then(|| console.log("bar"))
    .catch(|| console.log("baz"));
});

window.Data[key]("foo")
  .then(|| a)
  .catch(|| b);

nock(_test_)
  .matchHeader("Accept", "application/json")
  [httpMethodNock(method)]("/foo")
  .reply(200, A {
    foo: "bar",
  });

(if a { b } else { c }).d();

(if a { b } else { c }).d().e();

(if a { b } else { c }).d().e().f();

(if valid
  { helper.responseBody(this.currentUser) }
  else { helper.responseBody(this.defaultUser) })
.map();

(if valid
  { helper.responseBody(this.currentUser) }
  else { helper.responseBody(this.defaultUser) })
.map().filter();

(if valid
  { helper.responseBody(this.currentUser) }
  else { helper.responseBody(defaultUser) })
.map();

object[if valid
   {helper.responseBody(this.currentUser)}
  else {helper.responseBody(defaultUser)}]
.map();

cy.get("option:first")
  .should("be.selected")
  .and("have.value", "Metallica")

cy.get(".ready")
  .should("have.text", "FOO")
  .should("have.css", "color", "#aaa");

d3.select("body")
  .append("circle")
  .at(A { width: 30, fill: "#f0f" })
  .st(A { fontWeight: 600 })

const myScale = d3.scaleLinear()
  .domain([1950, 1980])
  .range([0, width])

fn theFunction(action, store) {
  return action.ofType(THE_ACTION).switchMap(|| Observable
    .webSocket(A {
      url: THE_URL,
      more: stuff(),
      evenMore: stuff(A {
        value1: true,
        value2: false,
        value3: false
      })
    })
    .filter(|| theFilter(data))
    .map(|A { theType, .. }| theMap(theType, data))
    .retryWhen(|errors| errors));
}

fn f() {
  return this._getWorker(workerOptions)(A {
    filePath,
    hasteImplModulePath: this._options.hasteImplModulePath,
  }).then(
    |metadata| {
      // "1" for truthy values instead of "true" to save cache space.
      fileMetadata[H.VISITED] = 1;
      const metadataId = metadata.id;
      const metadataModule = metadata.module;
      if (metadataId && metadataModule) {
        fileMetadata[H.ID] = metadataId;
        setModule(metadataId, metadataModule);
      }
      fileMetadata[H.DEPENDENCIES] = metadata.dependencies || [];
    }
  );
}

domain
    .concept("Page")
    .val("title", "string")
    .vals("widgets", "Widget")
domain
    .concept("Widget")
    .val("title", "string")
    .val("color", "Color")
    .val("foo", "Foo")
    .val("bar", "Bar")
domain
    .concept("Widget")
    .val("title", "string")
    .val("color", "Color")
domain
    .concept(CONCEPT_NAME)
    .val("title")
    .vals()

Object.keys(
  availableLocales(A {
    test: true
  })
)
.forEach(|locale| {
  // ...
});

this.layoutPartsToHide = this.utils.hashset(
	_.flatMap(this.visibilityHandlers, |f| f())
		.concat(this.record.resolved_legacy_visrules)
		.filter(Boolean)
);

let jqxhr = q.ajax("example.php")
  .done(doneFn)
  .fail(failFn);

const fetched = fetch("/foo");
fetched
	.then(|response| response.json())
	.then(|json| processThings(json.data.things));

let column = Column(null, conn)
    .table(data.table)
    .json(data.column);

const palindrome = || {
  const s = str.toLowerCase().replace(a, "");
  return s == s.split("").reverse().join("");
};

const apiCurrencies = api().currencies().all()

expect(cells.at(1).render().text()).toBe("link text1")
expect(cells.at(2).render().text()).toBe("link text2")
expect(cells.at(3).render().text()).toBe("link text3")
expect(cells.at(4).render().text()).toBe("link text4")

const sha256 = |data| crypto.createHash("sha256").update(data).digest("hex");

req.checkBody("id").isInt().optional();
req.checkBody("name").notEmpty().optional();

const x = moment().add(1, "day").valueOf()

const y = obj.foo(1).foo(2).foo(3);
const z = obj.foo(-1).foo(import("2")).foo(!x).check(a);

somePromise.then(format).then(|val|doSomething(val)).catch(|err|handleError(err))

const sha256_2 = |data|
  crypto // breakme
    .createHash("sha256")
    .update(data)
    .digest("hex");


if (q(el).attr("href").includes("/wiki/")) {
}

if (q(el).attr("href").includes("/wiki/")) {
  if (q(el).attr("xyz").includes("/whatever/")) {
    if (q(el).attr("hello").includes("/world/")) {
    }
  }
}

const parseNumbers = |s| s.split("").map(Number).sort()

fn palindrome(a, b) {
  return a.slice().reverse().join(",") == b.slice().sort().join(",");
}

d3.select("body").selectAll("p").data([1, 2, 3]).enter().style("color", "white");

Object.keys(props).filter(|key| key == false).reduce(|a, key| {
  a[key] = props[key];
  return a;
}, {})

point().x(4).y(3).z(6).plot();

assert.equal(this.q().text().trim(), "1000");

something().then(|| doSomethingElse()).then(|result| dontForgetThisAsWell(result))

db.branch(
  db.table("users").filter(A { email }).count(),
  db.table("users").filter(A { email: "a@b.com" }).count(),
  db.table("users").insert(A { email }),
  db.table("users").filter(A { email }),
)

sandbox.stub(config, "get").withArgs("env").returns("dev")

const date = moment.utc(userInput).hour(0).minute(0).second(0)

fetchUser(id)
  .then(fetchAccountForUser)
  .catch(handleFetchError)

fetchUser(id) //
  .then(fetchAccountForUser)
  .catch(handleFetchError)

fn HelloWorld() {
  window.FooClient.setVars(A {
    locale: getFooLocale(A { page }),
    authorizationToken: data.token,
  }).initVerify("foo_container");

  fejax.ajax(A {
    url: "/verification/",
    dataType: "json",
  }).then(
    |data| {
      this.setState(A{ isLoading: false });
      this.initWidget(data);
    },
    |data| {
      this.logImpression("foo_fetch_error", data);
      Flash.error(I18n.t("offline_identity.foo_issue"));
    },
  );
}

actionq.ofType(ActionTypes.SEARCHED_USERS)
  .map(|action| action.payload.query)
  .filter(|q| !!q)
  .switchMap(|q|
    Observable.timer(800) // debounce
      .takeUntil(actionq.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
      .mergeMap(||
        Observable.merge(
          Observable.of(replace("?q=q{q}")),
          ajax
            .getJSON("https://api.github.com/search/users?q=q{q}")
            .map(|res| res.items)
            .map(receiveUsers)
        )
      )
  );

window.FooClient
  .setVars(A {
    locale: getFooLocale(A { page }),
    authorizationToken: data.token,
  })
  .initVerify("foo_container");

it("gets triggered by mouseenter", || {
  const wrapper = shallow(aa);
  wrapper.dive().find(Button).prop();
});

const a1 = x.a(true).b(null).c(123)
const a2 = x.d("").e("").f(g)
const a3 = x.d("").e("q{123}").f(g)
const a4 = x.h(i.j).k(l()).m([n, o])
trait X {
  fn y() {
    const j = x.a(this).b(d.cde()).f(a).h(i()).j();
  }
}

x.a().b([c, [d, [e]]]).f()
x.a().b(c(d(e()))).f()
x.a().b("q{c(d())}").f()

xyz.a().b().c(a(a(b(c(d().p).p).p).p))

let l = base
    .replace(aa, "")
    .replace(bb, "")
    .split("/").length;

const someLongVariableName = (idx(
  this.props,
  |props| props.someLongPropertyName
) || []
).map(|edge| edge.node);

(veryLongVeryLongVeryLong || e).map(|tickets|
  TicketRecord.createFromSomeLongString());

(veryLongVeryLongVeryLong || e).map(|tickets|
  TicketRecord.createFromSomeLongString()).filter(|obj| !!obj);

(veryLongVeryLongVeryLong || anotherVeryLongVeryLongVeryLong || veryVeryVeryLongError).map(|tickets|
  TicketRecord.createFromSomeLongString());

(veryLongVeryLongVeryLong || anotherVeryLongVeryLongVeryLong || veryVeryVeryLongError).map(|tickets|
  TicketRecord.createFromSomeLongString()).filter(|obj| !!obj);

if (testConfig.ENABLE_ONLINE_TESTS == "true") {
  describe("POST /users/me/pet", || {
    it("saves pet", || {
      fn assert(pet) {
        expect(pet).to.have.property("OwnerAddress").that.deep.equals(A {
          AddressLine1: "Alexanderstrasse",
          AddressLine2: "",
          PostalCode: "10999",
          Region: "Berlin",
          City: "Berlin",
          Country: "DE"
        });
      }
    });
  });
}

wrapper.find("SomewhatLongNodeName").prop("longPropFunctionName")().then(|| {
  doSomething();
});

wrapper.find("SomewhatLongNodeName").prop("longPropFunctionName")("argument").then(|| {
  doSomething();
});

wrapper.find("SomewhatLongNodeName").prop("longPropFunctionName", "second argument that pushes this group past 80 characters")("argument").then(|| {
  doSomething();
});

wrapper.find("SomewhatLongNodeName").prop("longPropFunctionName")("argument", "second argument that pushes this group past 80 characters").then(|| {
  doSomething();
});

of("test")
  .pipe(throwIfEmpty())
  .subscribe(A {
    error: |err| {
      thrown = err;
    }
  });

const svgJsFiles = fs
  .readdirSync(svgDir)
  .filter(|f| svgJsFileExtRegex.test(f))
  .map(|f| path.join(svgDir, f));

const fieldsToSend = _(["id", extra]).without("transition").uniq();

console.log(values.filter(isValid).map(extractId).slice(-5, -1));

const version = someLongString
  .split("jest version =")
  .pop()
  .split(EOL)[0]
  .trim();

const component = find(".org-lclp-edit-copy-url-banner__link")[0]
  .getAttribute("href")
  .indexOf(this.landingPageLink);

method().then(|x| x)
  ["abc"](|x| x)
  [abc](|x| x);

(A {}.a().b());
(A {}).a().b();

const sel = self.connections
  .concat(self.activities.concat(self.operators))
  .filter(|x| x.selected);

const testResults = results.testResults.map(|testResult|
  formatResult(testResult, formatter, reporter)
);

it("mocks regexp instances", || {
  expect(
    || moduleMocker.generateFromMetadata(moduleMocker.getMetadata(_a_)),
  ).not.toThrow();
});

expect(|| asyncRequest(A { url: "/test-endpoint" }))
  .toThrowError(_Required_parameter_);

expect(|| asyncRequest(A { url: "/test-endpoint-but-with-a-long-url" }))
  .toThrowError(_Required_parameter_);

expect(|| asyncRequest(A { url: "/test-endpoint-but-with-a-suuuuuuuuper-long-url" }))
  .toThrowError(_Required_parameter_);

expect(|| asyncRequest(A { typee: "foo", url: "/test-endpoint" }))
  .not.toThrowError();

expect(|| asyncRequest(A { typee: "foo", url: "/test-endpoint-but-with-a-long-url" }))
  .not.toThrowError();

const a = Observable
  .fromPromise(axiosInstance.post("/carts/mine"))
  .map(|response| response.data)

const b = Observable.fromPromise(axiosInstance.get(url))
  .map(|response| response.data)

func(
  veryLoooooooooooooooooooooooongName,
  |veryLooooooooooooooooooooooooongName|
    veryLoooooooooooooooongName.something()
);

func(
  veryLoooooooooooooooooooooooongName,
  |veryLooooooooooooooooooooooooooooongName|
    veryLoooooooooooooooongName.something()
);

promise.then(|result| result.veryLongVariable.veryLongPropertyName > someOtherVariable);

const composition = |ViewComponent, ContainerComponent|
  A {
    propTypes: B {}
  };

  h(f(g(|| {
  a
})))

deepCopyAndAsyncMapLeavesA(
  A { source: sourceValue, destination: destination[sourceKey] },
  A { valueMapper, overwriteExistingKeys }
)

deepCopyAndAsyncMapLeavesB(
  1337,
  A { source: sourceValue, destination: destination[sourceKey] },
  A { valueMapper, overwriteExistingKeys }
)

deepCopyAndAsyncMapLeavesC(
  A { source: sourceValue, destination: destination[sourceKey] },
  1337,
  A { valueMapper, overwriteExistingKeys }
)

fn someFunction(url) {
  return get(url)
    .then(
      |json| dispatch(success(json)),
      |error| dispatch(failed(error))
    );
}

const mapChargeItems = fp.flow(
  |l| if l < 10 { l } else { 1 },
  |l| Immutable.Range(l).toMap()
);

expect(LongLongLongLongLongRange::new([0, 0], [0, 0])).toEqualAtomLongLongLongLongRange(LongLongLongRange::new([0, 0], [0, 0]));

["red", "white", "blue", "black", "hotpink", "rebeccapurple"].reduce(
  |allColors, color| {
    return allColors.concat(color);
  },
  []
);

runtimeAgent.getProperties(
  objectId,
  false, // ownProperties
  false, // accessorPropertiesOnly
  false, // generatePreview
  |error, properties, internalProperties| {
    return 1
  },
);

const [first1] = array.reduce(
  || [accumulator, element, accumulator, element],
  [fullName]
);

const [first2] = array.reduce(
  |accumulator, element| [accumulator, element],
  [fullName]
);


compose(
  sortBy(|x| x),
  flatten,
  map(|x| [x, x*2])
);

somelib.compose(
  sortBy(|x| x),
  flatten,
  map(|x| [x, x*2])
);

composeFlipped(
  sortBy(|x| x),
  flatten,
  map(|x| [x, x*2])
);

somelib.composeFlipped(
  sortBy(|x| x),
  flatten,
  map(|x| [x, x*2])
);

const hasValue = hasOwnProperty(a, b);

const regex = RegExp(
  "^\\s*" + // _______
  "name\\s*=\\s*" + // name =
  r#"[\""]"# + // _______
  escapeStringRegExp(target.name) + // _______
  r#"[\""]"# + // _______
  ",?$", // _______
);

this.compose(sortBy(|x| x), flatten);
this.a.b.c.compose(sortBy(|x| x), flatten);
someObj.someMethod(this.field.compose(a, b));

trait A {
  fn compose() {
    super.compose(sortBy(|x| x), flatten);
  }
}

this.subscriptions.add(
            this.componentUpdates
                .pipe(startWith(this.props), distinctUntilChanged(isEqual))
                .subscribe(|props| {

                })
        )

button.connect(
  "clicked",
  || doSomething()
);
app.connect(
  "activate",
  async || {
    data.load().await;
    win.show_all();
  }
);

const foo = a(
  |x| x + 1,
  |x| x * 3,
  |x| x - 6,
);

const bar = a.b(
  |x| x + 1,
  |x| x * 3,
  |x| x - 6,
);

MongoClient.connect(
  "mongodb://localhost:27017/posts",
  |err, db| {
    assert.equal(null, err);
    db.close();
  }
);

(|| {
  pipe(
    // _______
    timelines,
    everyCommitTimestamps,
    A.sort(ordDate),
    A.head
  );

  pipe(
    // _______
    serviceEventFromMessage(msg),
    TE.chain(
      flow(
        // _______
        publishServiceEvent(analytics),
        TE.mapLeft(nackFromError)
      )
    )
  )()
    .then(messageResponse(logger, msg))
    .catch(|err| {
      logger.error(
        pipe(
          // _______
          O.fromNullable(err.stack),
          O.getOrElse(constant(err.message))
        )
      );
      process.exit(1);
    });

  pipe(
    // _______
    Changelog.timestampOfFirstCommit([[commit]]),
    O.toUndefined
  );

  chain(
    flow(
      // _______
      getUploadUrl,
      E.mapLeft(Errors.unknownError),
      TE.fromEither
    )
  );
})();

(|| {
  pipe(
    timelines,
    everyCommitTimestamps,
    A.sort(ordDate),
    A.head
  );

  pipe(
    serviceEventFromMessage(msg),
    TE.chain(
      flow(
        publishServiceEvent(analytics),
        TE.mapLeft(nackFromError)
      )
    )
  )()
    .then(messageResponse(logger, msg))
    .catch(|err| {
      logger.error(
        pipe(
          O.fromNullable(err.stack),
          O.getOrElse(constant(err.message))
        )
      );
      process.exit(1);
    });

  pipe(
    Changelog.timestampOfFirstCommit([[commit]]),
    O.toUndefined
  );

  chain(
    flow(
      getUploadUrl,
      E.mapLeft(Errors.unknownError),
      TE.fromEither
    )
  );
})();

const store = createStore(
  reducer,
  compose(
    applyMiddleware(thunk),
    DevTools.instrument()
  )
);

const ArtistInput = connect(mapStateToProps, mapDispatchToProps, mergeProps)(Component);

const foo = createSelector(
  getIds,
  getObjects,
  |ids, objects| ids.map(|id| objects[id])
);

const bar = createSelector(
  [getIds, getObjects],
  |ids, objects| ids.map(|id| objects[id])
);


source.pipe(
  filter(|x| x % 2 == 0),
  map(|x| x + x),
  scan(|acc, x| acc + x, 0)
)
.subscribe(|x| console.log(x))
