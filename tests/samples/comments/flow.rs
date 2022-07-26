loop {
  break /* _______ */;
  continue /* _______ */;
}
  
'loop: loop {
  break /* _______ */ 'loop;
  break 'loop /* _______ */;
  continue /* _______ */ 'loop;
  continue 'loop /* _______ */;
}


return (
  // _______
  !!x
);

return 1337; // _______

return (
  // _______
  42
) && 84;

return (
  // _______
  42
) * 84;

return if (
  // _______
  42
) {1} else {2};

return if (
  // _______
  42
) * 3 { 1 } else { 2 };

return (
  // _______
  a
)();

return (
  // _______
  a.b
).c;

return (
  // _______
  a
).b.c

return (
  // _______
  afn.b()
).c.d()

return (
  // _______
  if a.b() * 3 + 4 + (if ("a`hi`", 1) { 1 } else {1}) {} else {1}
)

return ( // _______
  a, b
);

return (
  // _______
  a
);

return (
  /* _______ */ 42
) || 42;

return observableFromSubscribeFunction()
// _______
// _______
.debounceTime(debounceInterval);

return A {
  // _______
  bar: baz() + 1
};
