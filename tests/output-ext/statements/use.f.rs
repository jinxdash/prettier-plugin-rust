pub use self::bb::{ aa, bb };
pub use self::cc::*;
use Self::f;
use ::super::{ S, Z };
use ::super::main;
use a::*;
use m::S;
use ::{ ::{}, ::{} };
extern crate x;
use std::mem::self;
use foo::bar::self as abc;
extern crate priv_impl_prim_ty as bar;
extern crate crate_method_reexport_grrrrrrr2;
use std::io::{ self, Error as IoError };
use std::net::{ self as stdnet, TcpStream };
use foo::{ Foo, bar::{ baz::{}, foobar::* }, * };
use foo::bar::baz::{ *, * };
use foo::{};
mod m {
  use S;
  use self::{ self };
  use super::{ self };
}
pub use ::E::*;
use crate as _;
pub use ::E::V::{ self };
use std::{ ops::A, marker::{ C, B } };
mod bar {
  pub use bar::*;
  pub use main as f;
  pub use super::*;
  use ::std::mem;
  use crate_method_reexport_grrrrrrr2::rust::add;
  crate struct Foo;
}
use rustc_hir::BinOpKind::{
  Add,
  And,
  BitAnd,
  BitOr,
  BitXor,
  Div,
  Eq,
  Ge,
  Gtab,
};
use rustc_ast::ast::{
  ItemForeignMod,
  ItemImpl,
  ItemMac,
  ItemMod,
  ItemStatic,
  ItemDefaultImpl,
};
use exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{
  ItemA,
  ItemB,
};
use exceedingly::loooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{
  ItemA,
  ItemB,
};

use list::{
  // Some item
  SomeItem /* Comment */,
  /* Another item */ AnotherItem /* Another Comment */, // Last Item
  LastItem,
};

use test::{ Other /* C   */, /*   A   */ self /*    B     */ };

use rustc_ast::{ self };
use ::{ /* Pre-comment! */ Foo, Bar /* comment */ };
use Foo::{ Bar, Baz };
pub use rustc_ast::ast::{
  Expr_,
  Expr,
  ExprAssign,
  ExprCall,
  ExprMethodCall,
  ExprPath,
};

use rustc_ast::some::{};

use self;
use std::io::{ self };
use std::io::self;

mod Foo {
  pub use rustc_ast::ast::{ A };
  mod Foo2 {
    pub use rustc_ast::ast::{ A, self, B };
  }
}

fn test() {
  use Baz::*;
  use Qux;
}
use foo::bar::baz as baz;
use bar::quux as kaas;
use foo;
use foo::{ self as bar, baz };
use foo::{ self as bar };
use foo::{ qux as bar };
use foo::{ baz, qux as bar };
use ::foo;
use ::foo::{ Bar };
use ::foo::{ Bar, Baz };
use ::{ Foo };
use ::{ Bar, Baz };
use *;
use *;
error;
use super::*;
use foo::issue_1356::*;
#[cfg(unix)]
use self::unix::{};
use foo::{
  a,
  bar::{
    baz,
    qux,
    xxxxxxxxxxx,
    yyyyyyyyyyyyy,
    zzzzzzzzzzzzzzzz,
    foo::{ a, b, cxxxxxxxxxxxxx, yyyyyyyyyyyyyy, zzzzzzzzzzzzzzzz },
  },
  b,
  boo,
  c,
};
use fooo::{
  baar::{
    foobar::{
      xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx,
      yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy,
      zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz,
    },
  },
  z,
  bar,
  bar::*,
  x,
  y,
};
use exonum::{
  api::{ Api, ApiError },
  blockchain::{ self, BlockProof, Blockchain, Transaction, TransactionSet },
  crypto::{ Hash, PublicKey },
  helpers::Height,
  node::TransactionSend,
  storage::{ ListProof, MapProof },
};
use a::{ b::{ c::* } };
use a::{ b::{ c::{} } };
use a::{ b::{ c::d } };
use a::{ b::{ c::{ xxx, yyy, zzz } } };
/// a
// b
use c;
#[macro_use]
use imports_with_attr;
use std::f64::consts::{ SQRT_2, E, PI };
#[rustfmt::skip]
use std::fmt::{self, {Display, Formatter}};
// source: "../../../ext/jinx-rust/tests/samples/statements/use.rs"