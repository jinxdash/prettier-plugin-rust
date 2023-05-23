fn preserve_last_semicolon() {
  if let Some(left) = node.borrow().left.as_ref() {
    deque.push_back(left.clone());
  }
  if let Some(right) = node.borrow().right.as_ref() {
    deque.push_back(right.clone());
  };
}

fn a() {
  if let Ok(_) = lock.try_lock() {
  };
}

fn b() {
  let lock = std::sync::Mutex::new(10);
  match lock.try_lock() {
    Ok(_) => {}
    Err(_) => {}
  }
  match lock.try_lock() {
    Ok(_) => {}
    Err(_) => {}
  };
}

fn c() {
  if let Ok(_) = lock.try_lock() {
  };
  // comment
}

fn d() {
  if let Ok(_) = lock.try_lock() {
  }; // comment
}

fn e() {
  if let Ok(_) = lock.try_lock() {
  } /** comment */;
}

fn f() {
  if let Ok(_) = lock.try_lock() {
  };
}

fn g() {
  if let Ok(_) = lock.try_lock() {
  };
  // comment
}

fn h() {
  if let Ok(_) = lock {
  }
  if let Ok(_) = lock {
  }
}

fn i() {
  match lock {
  }
  match lock {
  }
}

fn inner_attr() {
  if let Ok(_) = lock.try_lock() {
  }
  #![attr]
}

// source: "../../samples/issues/22.rs"