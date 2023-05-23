pub fn main() {
  loop {
    return ({
      break;
    }) as ();
  }
  return ();
  ({
    return;
  }) as ();
  return if 1 { () } else { () };
  return ({
    return;
  }) as ();
  return {
    return {
      return;
    };
  };
  return try { 4 };
  return;
  return 'aaa: loop {
    break 'aaa 1;
  };
  loop {
    break 'aaa: loop {
      break 'aaa 1;
    };
  }
  'aaa: loop {
    break 'aaa 'bbb: loop {
      break 1;
    };
  }
  let a = 'a: loop {
    break 'a 1;
  };
  [
    ();
    {
      return || {
        let tx;
      };
    }
  ];
  [
    ();
    {
      return;
    }
  ];
  [
    ();
    {
      return match 0 {
        n => n,
      };
    }
  ];
  [
    ();
    {
      return match () {
        'a' => 0,
        _ => 0,
      };
    }
  ];
  let a = loop {
    break {
      return 0;
      ()
    };
  };
  let a = loop {
    break {
      break;
    };
  };
  let a = loop {
    break loop {
    };
  };
  let a = loop {
    break {
      return 0;
    };
  };
  loop {
    if (
      {
        break;
      }
    ) {
    }
  }
  for _ in (
    {
      return ();
      0..3
    }
  ) {
  }
  loop {
    while (
      {
        return;
      }
    ) {
      if (
        {
          return;
        }
      ) {
        match (
          {
            return;
          }
        ) {
          1 => {
            if (
              {
                return;
              }
            ) {
              return;
            } else {
              return;
            }
          }
          _ => {
            return;
          }
        };
      } else if (
        {
          return;
        }
      ) {
        return;
      }
    }
    if (
      {
        return;
      }
    ) {
      break;
    }
  }
  let () = if 0 {
  } else {
    return;
  };
}
// source: "../../../ext/jinx-rust/tests/samples/expressions/flow_expr.rs"