pub macro c {
  () => ();
  // _______
  // _______
  ($msg:expr $(,)?) => ();
  (
    $fmt:expr,
    $($arg:tt)*
  ) => ();
  ($foo:expr) => {};
  ($($t:tt)+) => ();
  ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => ();
  (
    $left:expr,
    $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?,
    $($arg:tt)+
  ) => ();
  // ______
  ("{}", $aze:expr $(,)?) => ();
  ($($t:tt)+) => ();
  ($x:pat | $y:pat) => {};
  ($($x:pat)+ | $($y:pat)+) => {};
  ($x:pat_param | $y:pat_param) => {};
  ($x:pat_param | $y:pat) => {};
  ($x:pat | $y:pat_param) => {};
}

macro_rules! c {
  // ____
  // ____
  // ____
  //
  // ____
  // ____
  // ____
  // ____
  //
  // ____
  // ____

  // ____
  (
    @ {
      // ____
      // ____
      start = $start:expr;

      // ____
      // ____
      ($($count:tt)*)

      // ____
      // ____
      // ____
      // ____
      // ____
      // ____
      $(
        ($($skip:tt)*)
        $bind:pat = $fut:expr,
        if $c:expr => $handle:expr,
      )+;
      // ____
      $else:expr
    }
  ) => {};

  // ____

  // ____
  // ____

  (
    @ {
      start = $start:expr;
      ($($s:tt)*)
      $($t:tt)*
    }
    $p:pat = $f:expr,
    if $c:expr => $h:block,
    $($r:tt)*
  ) => {};
  (
    @ {
      start = $start:expr;
      ($($s:tt)*)
      $($t:tt)*
    }
    $p:pat = $f:expr,
    if $c:expr => $h:expr
  ) => {};
  (
    @ {
      start = $start:expr;
      ($($s:tt)*)
      $($t:tt)*
    }
    $p:pat = $f:expr => $h:expr,
    $($r:tt)*
  ) => {};
  (
    @ {
      start = $start:expr;
      ($($s:tt)*)
      $($t:tt)*
    }
    $p:pat = $f:expr => $h:expr
  ) => {};
  (
    @ {
      start = $start:expr;
      $($t:tt)*
    }
    else => $else:expr $(,)?
  ) => {};
  (
    @ {
      start = $start:expr;
      $($t:tt)*
    }
  ) => {};

  (($($id:ident),*)) => (());
  ([$($id:ident),*]) => (());
  ({ $($id:ident),* }) => (());

  // ____

  (
    biased;
    $p:pat = $($t:tt)*
  ) => {};

  (
    $p:pat = $($t:tt)*
  ) => {};
  () => {};
  ($($tts:tt)+) => { loom::thread_local!{ $($tts)+ } };
  (
    @ {
      // ____
      // ____
      ($($count:tt)*)

      // ____
      $(
        ($($skip:tt)*)
        $e:expr,
      )*
    }
  ) => {};

  // ____

  (
    @ {
      ($($s:tt)*)
      $($t:tt)*
    }
    $e:expr,
    $($r:tt)*
  ) => {};
  (
    $(
      $(#[$cfg:meta])*
      $name:ident,
    )*
  ) => {};

  // ____

  ($($e:expr),* $(,)?) => {};
  () => {};

  (
    $(#[$attr:meta])*
    $vis:vis static $name:ident: $t:ty;
    $($rest:tt)*
  ) => {};

  (
    $(#[$attr:meta])*
    $vis:vis static $name:ident: $t:ty
  ) => {};
  (Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};
  (!Send & $(!)?Sync & $(!)?Unpin, $value:expr) => {};
  (
    $($f:ident $(< $($generic:ty),* >)?)::+($($arg:ty),*): $($tok:tt)*
  ) => {};
  ($i:ident, $start:ident, $($delta:expr),* $(,)?) => {};
  ($i:ident, $start:ident) => {};
  ($($name:ident = $sem:ident($bits:tt: $exp_bits:tt)),*) => {};
  ($ty:ident < $t:tt >) => {};
  (
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = $inner_attrs:literal;
    $($ty:path),*
  ) => {};
  ($($name:literal => $feature:ident)*) => {};
  ($($T:ident { $($field:ident: $A:ident),* $(,)? })*) => {};
  ($wr:ident.write_facts_to_path($this:ident.[$($field:ident,)*])) => {};
  (
    $in_:expr,
    {
      $param:expr,
      $flags:expr,
      $width:expr,
      $prec:expr,
      $len:expr,
      $type_:expr,
      $pos:expr,
    }
  ) => {};
  (
    ($($dollar:tt $placeholder:ident)*);
    $($($values:ident),+);*: $($test:tt)*
  ) => {};
  ($($name:ident: $($($p:ident),* => $call:ident),*;)*) => {};
  ($($name:ident($($arg:ident),*) => $llvm_capi:ident),+ $(,)?) => {};
  ($name:expr, fn() -> $ret:expr) => {};
  ($name:expr, fn(...) -> $ret:expr) => {};
  ($name:expr, fn($($arg:expr),*) -> $ret:expr) => {};
  ($($field_ty:expr),*) => {};
  ($($name:ident: $($($p:ident),* => $call:ident),*;)*) => {};
  (
    $where:expr,
    { $($what_fmt:expr),+ }
    $(expected { $($expected_fmt:expr),+ })?
  ) => {};
  (
    $e:expr,
    $where:expr,
    $(
      $($p:pat_param)|+ => { $($what_fmt:expr),+ }
      $(expected { $($expected_fmt:expr),+ })?
    ),+
    $(,)?
  ) => {};
  (
    $(#[$attr:meta])*
    pub enum $name:ident {
      $(
        $(#[$var_attr:meta])*
        $variant:ident = $e:expr,
      )*
    }
  ) => {};
  (
    $(#[$attr:meta])*
    pub enum $name:ident {
      $(
        $(#[$var_attr:meta])*
        $variant:ident,
      )*
    }
  ) => {};
  (impl $fblock:tt [$($c:tt,)*] [$block:tt $(, $rest:tt)*]) => {};
  (impl $fblock:tt [$($blocks:tt,)*] []) => {};
  ($($ecode:ident: $message:expr,)*; $($code:ident,)*) => {};
  (
    $(#[$attrs:meta])*
    pub fn $n:ident(&self, $($name:ident: $ty:ty),* $(,)?) -> &Self
  ) => {};
  (
    $enc:expr, // _______
    $idx:expr, // _______
    $struct:expr, // _______
    $struct_name:ident, // _______
    [$($name:ident),+ $(,)?], // _______
    [$($ignore:ident),+ $(,)?] // _______
  ) => {};
  (
    $(
      $Kind:ident($AstTy:ty)
      {
        $kind_name:expr;
        $(
          one fn $mut_visit_ast:ident;
          fn $visit_ast:ident;
        )?
        $(
          many fn $flat_map_ast_elt:ident;
          fn $visit_ast_elt:ident($($args:tt)*);
        )?
        fn $make_ast:ident;
      }
    )*
  ) => {};
  ($ty:ident { $($field:ident $(: $value:expr)*),+ $(,)? }) => {};
  ($ty:ident::$method:ident($($value:expr),*)) => {};
  (
    $(
      $(#[doc = $doc:tt])*
      (accepted, $feature:ident, $ver:expr, $issue:expr, None),
    )+
  ) => {};
  (
    $(
      $(#[$attr:meta])*
      $variant:ident $($group:expr)?,
      $module:ident::$name:ident,
      $method:ident,
      $target:expr,
      $generics:expr;
    )*
  ) => {};
  (
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => {};
  (
    [$hir:tt],
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => {};
  (
    [
      $span:expr,
      $($fmt:tt)*
    ] $arg:expr,
    $($rest:tt)*
  ) => {};
  ($($T:ty),*) => {};
  (#$var:ident) => {};
  (
    $call:ident! $extra:tt
    ($($b1:tt)*)
    ($($curr:tt)*)
  ) => {};
  (
    $call:ident!($($extra:tt)*)
    #$var:ident
  ) => {};
  (
    $tokens:ident
    ($($b3:tt)*)
    ($($b2:tt)*)
    ($($b1:tt)*)
    ($($curr:tt)*)
    ($($a1:tt)*)
    ($($a2:tt)*)
    ($($a3:tt)*)
  ) => {};
  ($tokens:ident $b3:tt $b2:tt $b1:tt @ $a1:tt $a2:tt $a3:tt) => {};
  (
    $tokens:ident $b3:tt $b2:tt $b1:tt (#)
    ($($inner:tt)*)
    *$a3:tt
  ) => {};
  (
    [$($attrs_pub:tt)*]
    enum $name:ident #no_visit
    $($rest:tt)*
  ) => {};
  (
    $(#[$enum_attr:meta])*
    $pub:ident $enum:ident $name:ident #$tag:ident $body:tt
    $($remaining:tt)*
  ) => {};
  (
    $pub:ident $enum:ident $name:ident {
      $(
        $(#[$variant_attr:meta])*
        $variant:ident $(($($member:ident)::+))*,
      )*
    }
    $($remaining:tt)*
  ) => {};
  (
    ($($arms:tt)*)
    $tokens:ident $name:ident {
      $variant:ident $member:ident,
      $($next:tt)*
    }
  ) => {};
  (
    $mac:ident!(
      $(#[$m:meta])*
      $pub:ident
      $($t:tt)*
    )
  ) => {};
  ($($token:tt pub struct $name:ident #[$doc:meta])*) => {};
  ($($token:tt pub struct $name:ident / $len:tt #[$doc:meta])*) => {};
  {
    $($name:ident)::+$(< $param:ident >)?
    $([$field:tt $this:ident $other:ident])*
    $(![$ignore:tt])*;
    !$next:tt
    $($rest:tt)*
  } => {};
  {
    $($name:ident)::+;
    $([
      $($variant:ident)::+;
      $([$field:tt $this:ident $other:ident])*
      $(![$ignore:tt])*
    ])*
  } => {};
  {
    $($name:ident)::+;
    $([
      $($variant:ident)::+;
      $($fields:tt)*
    ])*
    $next:ident
    $($rest:tt)*
  } => {};
  (($expr:ident) as $t:ty, @ $snapshot:literal) => {};
  (
    $(#[$smeta:meta])*
    pub struct $stratname:ident
    [$($sgen:tt)*]
    [$($swhere:tt)*]
    ($innerstrat:ty) -> $stratvtty:ty;
    $(#[$vmeta:meta])*
    pub struct $vtname:ident
    [$($vgen:tt)*]
    [$($vwhere:tt)*]
    ($innervt:ty) -> $actualty:ty;
  ) => {};

  { $ head: expr } => { Cons ( $ head , Nil ) };
  {
    $ head: expr,
    $ ($ tail: expr),
    *
  } => { Cons ( $ head , hlist ! ( $ ( $ tail ) , * ) ) };
  { $ head: ty } => { Cons < $ head , Nil > };
  {
    $ head: ty,
    $ ($ tail: ty),
    *
  } => { Cons < $ head , HList ! ( $ ( $ tail ) , * ) > };
  { ($ ($ LHS: tt) +) } => { Expr ! ( $ ( $ LHS ) + ) };
  {
    HList![$ ($ LHS: tt) *] + $ ($ RHS: tt) +
  } => { < Expr ! ( HList ! [ $ ( $ LHS ) * ] ) as Add < Expr ! ( $ ( $ RHS ) + ) >> :: Output };
  {
    $ LHS: tt + $ ($ RHS: tt) +
  } => { < Expr ! ( $ LHS ) as Add < Expr ! ( $ ( $ RHS ) + ) >> ::  Output };
  { $ LHS: ty } => { $ LHS };
  (
    $(
      $name:ident {
        $(
          fn $method:ident($($arg:ident: $arg_ty:ty),* $(,)?)
          $(-> $ret_ty:ty)*;
        )*
      }
    ),*
    $(,)?
  ) => {};
}
// source: "../../../ext/jinx-rust/tests/samples/macro/macro.match.rs"