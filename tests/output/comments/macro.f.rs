a!(~ " {    }  ");
a!(~ // 1
);
a!(~ {  // 2
});

cfg_if::cfg_if! {
  if #[attr] {
    if 0 {
    } else {
      // ERROR!
    }
  }
}

a! {
    if #[attr] {
        // ERROR!
    } 
}

x! {~ {
    // ERROR!
}
}

// source: "../../samples/comments/macro.rs"