a = b || /** 5_______ */ c;

a = b || /** 6_______ */ c;

a =
  b ||
  /** 7_____________________________________________________________________________ */
  c;

a =
  b ||
  /** 8_____________________________________________________________________________ */ c;

a =
  b ||
  /** 9_____________________________________________________________________________ */ c;

a = b && /** 10_______ */ c;

a = b && /** 11_______ */ c;

a =
  b &&
  /** 12_____________________________________________________________________________ */
  c;

a =
  b &&
  /** 13_____________________________________________________________________________ */ c;

a =
  b &&
  /** 14_____________________________________________________________________________ */ c;

a = b + /** 15_______ */ c;

a = b + /** 16_______ */ c;

a =
  b +
  /** 17_____________________________________________________________________________ */
  c;

a =
  b +
  /** 18_____________________________________________________________________________ */ c;

a =
  b +
  /** 19_____________________________________________________________________________ */ c;

a =
  b || // 20_______
  c;

a =
  b || // 21_____________________________________________________________________________
  c;

a =
  b && // 22_______
  c;

a =
  b && // 23_____________________________________________________________________________
  c;

a =
  b + // 24_______
  c;

a =
  b + // 25_____________________________________________________________________________
  c;

0 +
  // 26_______
  x;

0 *
  // 27_______
  x;
0 /
  // 28_______
  x;
0 -
  // 29_______
  x;
0 %
  // 30_______
  x;
0 <<
  // 31_______
  x;
0 >>
  // 32_______
  x;
0 &
  // 33_______
  x;
0 |
  // 34_______
  x;
0 ^
  // 35_______
  x;
// source: "../../samples/comments/binaryish.rs"