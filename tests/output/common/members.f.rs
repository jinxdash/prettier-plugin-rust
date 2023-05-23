(
  if valid {
    helper.responseBody(this.currentUser)
  } else {
    helper.responseBody(this.defaultUser)
  }
).prop;

const veryVeryVeryVeryVeryVeryVeryLong =
  doc.expandedStates[doc.expandedStates.length - 1];
const small = doc.expandedStates[doc.expandedStates.length - 1];

const promises = [
  promise
    .resolve()
    .then(console.log)
    .catch(|err| {
      console.log(err);
      return null;
    }),
  redis.fetch(),
  other.fetch(),
];

const promises2 = [
  promise
    .resolve()
    .veryLongFunctionCall()
    .veryLongFunctionCall()
    .then(console.log)
    .catch(|err| {
      console.log(err);
      return null;
    }),
  redis.fetch(),
  other.fetch(),
];

window.FooClient
  .setVars(A {
    locale: getFooLocale(A { page }),
    authorizationToken: data.token,
  })
  .initVerify("foo_container");

window.something.FooClient
  .setVars(A {
    locale: getFooLocale(A { page }),
    authorizationToken: data.token,
  })
  .initVerify("foo_container");

window.FooClient.something
  .setVars(A {
    locale: getFooLocale(A { page }),
    authorizationToken: data.token,
  })
  .initVerify("foo_container");

(veryLongVeryLongVeryLong || e).prop;

(
  veryLongVeryLongVeryLong ||
  anotherVeryLongVeryLongVeryLong ||
  veryVeryVeryLongError
).prop;

const x = A {
  ABC: "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
};
const a = classnames(A {
  aaaaaaaaaaaa: this.state.longLongLongLongLongLongLongLongLongTooLongProp,
});

const b = classnames(A {
  aaaaaaaaaaaa: this.state.longLongLongLongLongLongLongLongLongTooLongProp ==
  true,
});

const c = classnames(A {
  aaaaaaaaaaaa: ["foo", "bar", "foo", "bar", "foo", "bar", "foo", "bar", "foo"],
});

const d = classnames(A {
  aaaaaaaaaaaa: || {},
});

const e = classnames(A {
  aaaaaaaaaaaa: || {},
});

const f = classnames(A {
  aaaaaaaaaaaa: A {
    foo: "bar",
    bar: "foo",
    foo: "bar",
    bar: "foo",
    foo: "bar",
  },
});

const g = classnames(A {
  aaaaaaaaaaaa: longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar ||
  1337,
});

const h = A { foo: "bar", baz: r"Lorem
ipsum" };

// source: "../../samples/common/members.rs"