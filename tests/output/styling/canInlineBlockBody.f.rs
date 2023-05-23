[
  {
    if 0 {
      0
    }
  },
  {
    if 0 { 0 } else { 0 }
  },
  {
    while 0 {
      0;
    }
  },
  {
    unsafe { 0 }
  },

  {
    0;
    if 0 {
      0
    }
  },
  {
    0;
    if 0 {
      0
    } else {
      0
    }
  },
  {
    0;
    while 0 {
      0;
    }
  },
  {
    0;
    unsafe { 0 }
  },

  f(async {}),
  f(async { 1 }),
  f(async {
    1;
  }),

  {
    async {}
  },
  {
    async { 1 }
  },
  {
    async {
      1;
    }
  },

  {
    0;
    async {}
  },
  {
    0;
    async { 1 }
  },
  {
    0;
    async {
      1;
    }
  },

  if (0 as u8) < 1 {} else {},
  {
    0;
    if (0 as u8) < 1 {
    } else {
    }
  },
  if (0 as u8) < 1 {} else if (0 as u8) < 1 {},

  if 0 {
  } else {
    0;
  },
  if 0 {
  } else if 1 {
    0;
  } else {
  },
  if 0 {
  } else if 1 {
  } else {
    0;
  },

  if 0 {
    0;
  } else {
  },

  if 0 {
    0;
  } else {
    2
  },
  if 0 {
    2
  } else {
    0;
  },

  match 0 {
    0 => 0,
    0 => { 0 }
  },

  f(if 0 { 1 } else { 2 }),
  f(
    {
      0;
    },
    if 0 {
      1
    } else {
      2
    }
  ),
  0 + (if 0 { 1 } else { 2 }),
  { 0 + (if 0 { 1 } else { 2 }) },
  ({
    0;
  }) + (if 0 { 1 } else { 2 }),

  match 0 {
    0 => {
      break 0;
    }
    0 => {
      o = 0;
    }
    0 =>
      match 0 {
      }
    0 => if 0 {} else {}
    0 => if 0 {
    }
  },

  || {
    loop {
      match 0 {
        0 => {
          break 0;
        }
        0 => {
          o = 0;
        }
        0 =>
          match 0 {
          }
        0 => if 0 {} else {}
        0 => if 0 {
        }
      }
    }
  },
];

// source: "../../samples/styling/canInlineBlockBody.rs"