if 0 {
	0;
	//
} else if 0 {
}

if 1 {
	/*
	 * _______
	 */
}

if 1 {
  // _______
}

if 1 {
}
// _______
else {

}

if 1
// if 1 (ln trailing)
{
  1
}
// else if 2 (leading)
else if 2
  { 2 }
// else if 3 (leading)
// else if 3 (leading)
// else if 3 (leading)
else if 3
  // if 3 (ln trailing)
  { 3 }
// else if 4 (leading)
else if 4 {
  // 4 body
}
// else (leading)
else {
}

if 5 // if 5 (trailing)
{ 1 }

if 6 // if 6 (trailing)
{ 6 }
else if 7 // else if 7 (trailing)
{ 7 }
else // else (trailing)
{ 0 }

if 8 // if 8 (trailing)
// ^ if 8 (ln trailing)
{ 1 }
else if 9 // else if 9 
// else if 9 (ln trailing)
{ 1 }
else // else (trailing)
// else (ln trailing)
{ 1 }

if 10 /* _______ */ // _______
{ 10 }
else if 11 /* _______ */
{ 11 }
else if 12 // _______ /* _______ */ // _______
{ 12 }
else if 13 /* _______ */ /* _______ */ // _______
{ 13 }
else /* _______ */
{ 0 }

if 14 // _______
/* _______ */
// _______
{ 14 }
else if 15 // _______
/* _______ */
/* _______ */ // _______
{ 15 }


for // _______
a in b {}

for /* _______ */ a in b {}

let a = {/* _______ */};
let b = {
  // _______
};

for e in q {
  r = *e; // c 
}

while 
  true
  // _______
 {}

while true // _______
{}

while true {}// _______

while true /*_______*/{}
while true  /* _______ */ {}

while 
true // _______
&& true // _______
 {}

while true  {} // _______


if (cond) {
  stuff;
} /* _______ */ else if (cond) {
  stuff;
}
// _______
else {
  stuff;
}

if (cond){ stuff;}
// _______
else {stuff;}

ret = if __DEV__ 
  // _______
{vm.runInContext(source, ctx)}
else {a}

if (a == 0) {doSomething(); // _______
}else if (a == 1){ doSomethingElse(); // _______
}else if (a == 2) {doSomethingElse(); // _______
}
if (a == 0) {doSomething(); /* _______ */
}else if (a == 1){ doSomethingElse(); /* _______ */
}else if (a == 2){ doSomethingElse(); /* _______ */
}
if (a == 0){ expr; // _______
}else if (a == 1){ expr; // _______
}else if (a == 2){ expr; // _______
}
if (a == 0){ expr; /* _______ */
}else if (a == 1){ expr; /* _______ */
}else if (a == 2){ expr; /* _______ */
}
if (a == 0) {looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // _______
}else if (a == 1){ looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // _______
}else if (a == 2){ looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // _______
}
if (code == 92 /* _______ */) {}
if (code == 92 /* _______ */ /* _______ */) {}

if (code == 92) /* _______ */ {}
if (code == 92) { /* _______ */ }

if (
  1
  // _______
) {
  a;
}

{
  'a: // _______
  loop {}
}
{
  'a:
  // _______
  loop { }
}


fn f() {
  a
  /* _______ */
}

fn f() {
  a

  /* _______ */
}

fn d() {
  /* _______ */
}

fn f() {
  // _______
  f()

  // _______
  f()

  // _______
  // _______
}


fn f() 
// _______
{
  return 1
}

fn f() // _______
{
  return 1
}

fn f() { // _______
  return 1
}

fn f() {
  // _______
  return 1;
}

unsafe // So this is a very long comment.
// Multi-line, too.
// Will it still format correctly?
{
  a
}

{ /* a block with a comment */ }
{

}
{
    // A block with a comment.
}

fn foo() {
  async {
      // Do
      // some
      // work
  }
  .await;

  async {
      // Do
      // some
      // work
  }
      .await;
}
