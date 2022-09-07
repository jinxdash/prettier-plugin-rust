macro_rules! x {
  () => {
		$cx.pass.$f(&$cx.context, $($args),*);
        $(pub struct $sem;)*
        $(pub type $name = IeeeFloat<$sem>;)*
        $(impl Semantics for $sem {
            const BITS: usize = $bits;
            const PRECISION: usize = ($bits - 1 - $exp_bits) + 1;
            const MAX_EXP: ExpInt = (1 << ($exp_bits - 1)) - 1;
        })*
		for elem in $list {
            $visitor.$method(elem)
        }
		for elem in $list {
            $visitor.$method(elem, $($extra_args,)*)
        }
  };
  ($this:expr; $($x:expr),*) => (
        $this.arena.alloc_from_iter([$($x),*])
  );
  (path, < $type:ty as $trait:path > ::$name:ident) => {
        <$type as $trait>::$name
  };

  (ty, < $type:ty as $trait:ty > ::$name:ident) => {
        <$type as $trait>::$name
  };
  (
    $fnname:ident,
    $arg:ident,
    $ty:ty,
    $body:block,
    $val:expr,
    $pat:pat,
    $res:path
  ) => (
    {
        fn $fnname($arg: $ty) -> Option<$ty> $body
        match $fnname($val) {
          Some($pat) => {
            $res
          }
          _ => { panic!(); }
        }
    }
  );
  ($l:lifetime, $l2:lifetime) => {
        fn f<$l: $l2, $l2>(arg: &$l str, arg2: &$l2 str) -> &$l str {
            arg
        }
  };
  ($a:lifetime) => {
        $a: loop {
            break $a;
            panic!("failed");
        }
  };
  ($a:lifetime) => {
        break $a;
  };
  ($b:lifetime) => {
        'b: loop { // comment
            break $b; // comment
        }
  };
  ($l:lifetime) => {
        fn f(arg: &$l str) -> &$l str {
            arg
        }
  };
  ($l:lifetime) => {
        fn f<$l>(arg: &$l str) -> &$l str {
            arg
        }
  };
  ($s:literal ..= $e:literal) => {
        match 3 {
            $s ..= $e => "literal, in range",
            _ => "literal, other",
        }
  };
  ($s:pat) => {
        match 3 {
            $s => "pat, single",
            _ => "pat, other",
        }
  };
  ($s:literal ..= $e:literal) => {
        &format!("macro caught literal: {} ..= {}", $s, $e)
  };
  (
    ($s:expr) ..= ($e:expr)
  ) => { // comment
        &format!("macro caught expr: {} ..= {}", $s, $e)
    };
  ($s:expr, $e:expr) => {
        {
            let mut v = Vec::new();
            for i in $s .. $e {
                v.push(i);
            }
            "expr"
        }
  };
  ($at:meta) => {
        #[cfg($at)]
        static MISTYPED: () = "foo";
  };
  ($($x:tt)*) => { $($x)* };
  (subst $lhs:tt => $rhs:tt) => (
    {
        macro_rules! anon { $lhs => $rhs }
        anon!(1_usize, 2_usize, "foo")
    }
  );
  ($x:expr; $fragment:ident) => {
        macro_rules! inner { ($y:$fragment) => { $x + $y } }
  };
  ($expr:expr, $($($pat:pat)|+ => $expr_arm:expr),+) => {
        match $expr {
            $(
                $( $pat => $expr_arm, )+
            )+
        }
  };
  ($nm:ident, #[$a:meta], $i:item) => (mod $nm { #[$a] $i });
  ($p:pat | $p2:pat) => {
    {
        match Some(1u8) {
            $p | $p2 => {}
            _ => {}
        }
    }
  };
  ($p:pat in $e:expr) => {
    {
        let mut iter = $e.into_iter();
        while let $p = iter.next() {}
    }
  };
  ($p:pat if $e:expr) => {
    {
        match Some(1u8) {
            $p if $e => {}
            _ => {}
        }
    }
  };
  ($name:ident { $($variant:ident = $value:expr,)* }) => {
        enum $name {
            $($variant = $value,)*
        }

        fn foo(value: i32) -> Option<$name> {
            match value {
                $( $value => Some($name::$variant), )*
                _ => None
            }
        }
  };
  ($f:ident, ($($x:ident),*), $body:block) => (
        fn $f( $( $x : isize),* ) -> isize $body
  );
  (
    < $a:expr;
    > $($b:tt)*
  ) => { Keep(parse_item!($a),parse_list!($($b)*)) };
  (
    $a:tt
    $($b:tt)*
  ) => { Skip(parse_item!($a), parse_list!($($b)*)) };
  ($x:tt) => (type Alias = $x<i32>;);
  ($($code:tt)*) => {
        expr!(thread::spawn(move|| {$($code)*}).join())
  };
  ($($m:ident $($f:ident $v:tt)+),*) => {
        $($(macro_rules! $f { () => { $v } })+)*
        $(macro_rules! $m { () => { $(fn $f() -> i32 { $v })+ } })*
  };
  ($n:ident $x:expr) => { macro_rules! $n { ($y:expr) => { $x + $y }; } };
  ($name:ident) => {
        #[derive(Debug)]
        struct Foo {
            #[cfg(not(FALSE))]
            field: fn($name: bool)
        }
  };
  { $A:ty, $B:ty } => { ($A, $B) };
  ($id1:ident, $id2:ident, $e:expr) => (
        fn foo(a:T, b:T) -> T {
            match (a, b) {
                (T::A($id1), T::A($id2)) => T::A($e),
                (T::B($id1), T::B($id2)) => T::B($e),
                _ => panic!()
            }
        }
  );
  ($expression:expr, $($pattern:pat)|+ $(if $guard:expr)? $(,)?) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
  };
  (
    $dst:expr,
    $($arg:tt)*
  ) => ($dst.write_fmt(format_args!($($arg)*)));
  ($x:pat |) => ();
  () => {
		// comment
		// comment
		mod bar {
			#[derive(Double)]
			struct Bar($crate::Foo);
		}
	
		mod qself {
			#[derive(Double)]
			struct QSelf(<::Foo as $crate::Trait>::Assoc);
		}
	
		mod qself_recurse {
			#[derive(Double)]
			struct QSelfRecurse(<<$crate::Foo as $crate::Trait>::Assoc as $crate::Trait>::Assoc);
		}
	
		mod qself_in_const {
			#[derive(Double)]
			#[repr(u32)]
			enum QSelfInConst {
				Variant = <::Foo as $crate::Trait>::CONST,
			}
		}
  };
  ($item:item) => {
        #[derive(Print)]
        struct Foo {
            #[cfg(FALSE)] removed: bool,
            field: [bool; {
                $item
                0
            }]
        }
  };
  {
    $(#[$attr:meta])*
    pub $name:ident,
    $_reason:expr
  } => {
        $(#[$attr])*
        #[allow(dead_code)]
        pub static $name: ClippyDeprecatedLint = ClippyDeprecatedLint {};
  };
  ($($name:ident $assign:ident)*) => {
        /// comment
        pub static BINOP_TRAITS: &[LangItem] = &[$(LangItem::$name,)*];

        /// comment
        pub static OP_ASSIGN_TRAITS: &[LangItem] = &[$(LangItem::$assign,)*];

        /// comment
        pub fn binop_traits(kind: hir::BinOpKind) -> Option<(LangItem, LangItem)> {
            match kind {
                $(hir::BinOpKind::$name => Some((LangItem::$name, LangItem::$assign)),)*
                _ => None,
            }
        }
  };
  ($($t:ident),*) => {
        $(impl HirNode for hir::$t<'_> {
            fn hir_id(&self) -> HirId {
                self.hir_id
            }
            fn span(&self) -> Span {
                self.span
            }
        })*
  };
  (
    $(
      $major:literal,
      $minor:literal,
      $patch:literal { $($name:ident),* $(,)? }
    )*
  ) => {
        $($(
        pub const $name: RustcVersion = RustcVersion::new($major, $minor, $patch);
        )*)*
  };
  (
    impl $imp:ident,
    $method:ident for $t:ty,
    type Output = $o:ty
  ) => {
        impl $imp<$t> for &$t {
            type Output = $o;

            fn $method(self, other: $t) -> $o {
                $imp::$method(self, &other)
            }
        }

        impl $imp<&$t> for $t {
            type Output = $o;

            fn $method(self, other: &$t) -> $o {
                $imp::$method(&self, other)
            }
        }

        impl $imp for $t {
            type Output = $o;

            fn $method(self, other: $t) -> $o {
                $imp::$method(&self, &other)
            }
        }
  };
  (
    $iter:ident,
    $($token:ident $({ $($fields:tt)* })? $(($capture:ident))?)*
  ) => {
        {
           $($(let $capture =)? if let Some((TokenKind::$token $({$($fields)*})?, _x)) = $iter.next() {
               _x
           } else {
               continue;
           };)*
           #[allow(clippy::unused_unit)]
           { ($($($capture,)?)*) }
       }
  };
  (exp $e:expr) => { $e };
  (
    exp
    $($t:tt)+
  ) => { exp!($($t)+) };
  {} => { Nil };
  { $ head: expr } => { Cons ( $ head , Nil ) };
  {
    $ head: expr,
    $ ($ tail: expr),
    *
  } => { Cons ( $ head , hlist ! ( $ ( $ tail ) , * ) ) };
  {} => { Nil };
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
  } => { < Expr ! ( $ LHS ) as Add < Expr ! ( $ ( $ RHS ) + ) >> :: Output };
  { $ LHS: ty } => { $ LHS };
  (
    [$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt | $source:ident
  ) => {
        impl From<$source> for $id {
            #[inline]
            fn from(source: $source) -> Self {
                fn static_assert_same_number_of_lanes<T, U>()
                where
                    T: crate::sealed::Simd,
                    U: crate::sealed::Simd<LanesType = T::LanesType>,
                {
                }
                use llvm::simd_cast;
                static_assert_same_number_of_lanes::<$id, $source>();
                Simd(unsafe { simd_cast(source.0) })
            }
        }

        // comment
                // comment
                /*
                comment
                */

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _from_ $source] {
                    use super::*;
                    #[test]
                    fn from() {
                        assert_eq!($id::lanes(), $source::lanes());
                        let source: $source = Default::default();
                        let vec: $id = Default::default();

                        let e = $id::from(source);
                        assert_eq!(e, vec);

                        let e: $id = source.into();
                        assert_eq!(e, vec);
                    }
                }
            }
        }
  };
  (
    [
      $elem_ty:ident;
      $elem_count:expr
    ]: $id:ident | $test_tt:tt | $($source:ident),*
  ) => {
        $(
            impl_from_vector!([$elem_ty; $elem_count]: $id | $test_tt | $source);
        )*
  };
  (
    $(
      $(#[$cfg:meta])*
      $name:ident,
    )*
  ) => {
        [$($(#[$cfg])* (b!($name), $name::bench as fn(&str) -> C<(), ()>),)*]
  };
  (
    $(
      $(#[$attr:meta])*
      pub fn $func:ident($input:ident: &str) -> String;
    )+
  ) => {
        $(
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(_input: ::proc_macro::TokenStream) -> ::proc_macro::TokenStream {
                panic!()
            }
        )+
  };
  (
    $(
      $(#[$attr:meta])*
      pub fn $func:ident($input:ident: &str) -> String $body:block
    )+
  ) => {
        $(
            // comment
            $( #[$attr] )*
            #[proc_macro_derive($func)]
            pub fn $func(input: f!()) -> f!() {
                unsafe { a(0); }
                panic!()
            }
        )+
  };
  (
    $context:ty,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(#[inline(always)] fn $name(&mut self, _: $context, $(_: $arg),*) {})*
  );
  (
    [],
    [$hir:tt],
    [$($methods:tt)*]
  ) => (
        pub trait LateLintPass<$hir>: LintPass {
            expand_lint_pass_methods!(&LateContext<$hir>, [$($methods)*]);
        }
  );
  ([$($passes:ident),*], $self:ident, $name:ident, $params:tt) => (
    {
        $($self.$passes.$name $params;)*
    }
  );
  (
    $passes:tt,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(fn $name(&mut self, context: &LateContext<'tcx>, $($param: $arg),*) {
            expand_combined_late_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
  );
  (
    [$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]],
    [$hir:tt],
    $methods:tt
  ) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($passes: $passes,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($passes: $constructor,)*
                }
            }

            $v fn get_lints() -> LintArray {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$passes::get_lints());)*
                lints
            }
        }

        impl<'tcx> LateLintPass<'tcx> for $name {
            expand_combined_late_lint_pass_methods!([$($passes),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
  );
  ($macro:path, $args:tt) => (
        $macro!($args, [
            fn check_param(a: &ast::Param);
        ]);
  );
  (
    $context:ty,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(#[inline(always)] fn $name(&mut self, _: $context, $(_: $arg),*) {})*
  );
  (
    [],
    [$($methods:tt)*]
  ) => (
        pub trait EarlyLintPass: LintPass {
            expand_early_lint_pass_methods!(&EarlyContext<'_>, [$($methods)*]);
        }
  );
  ([$($passes:ident),*], $self:ident, $name:ident, $params:tt) => (
    {
        $($self.$passes.$name $params;)*
    }
  );
  (
    $passes:tt,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(fn $name(&mut self, context: &EarlyContext<'_>, $($param: $arg),*) {
            expand_combined_early_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
  );
  (
    [$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]],
    $methods:tt
  ) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($passes: $passes,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($passes: $constructor,)*
                }
            }

            $v fn get_lints() -> LintArray {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$passes::get_lints());)*
                lints
            }
        }

        impl EarlyLintPass for $name {
            expand_combined_early_lint_pass_methods!([$($passes),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
  );
  (
    $ty:expr,
    $val:expr,
    $negative:expr,
    $($type:ident => [$($utypes:expr),*] => [$($itypes:expr),*]),+
  ) => {
           {
               let _neg = if negative { 1 } else { 0 };
               match $ty {
                   $($type => {
                       $(if !negative && val <= uint_ty_range($utypes).1 {
                           return Some($utypes.name_str())
                       })*
                       $(if val <= int_ty_range($itypes).1 as u128 + _neg {
                           return Some($itypes.name_str())
                       })*
                       None
                   },)+
                   _ => None
               }
           }
  };
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr
  ) => (
        $crate::declare_tool_lint!{$(#[$attr])* $vis $tool::$NAME, $Level, $desc, false}
  );
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr,
    report_in_external_macro: $rep:expr
  ) => (
         $crate::declare_tool_lint!{$(#[$attr])* $vis $tool::$NAME, $Level, $desc, $rep}
  );
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr,
    $external:expr
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: &concat!(stringify!($tool), "::", stringify!($NAME)),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: None,
            report_in_external_macro: $external,
            future_incompatible: None,
            is_plugin: true,
            feature_gate: None,
            crate_level_only: false,
        };
  );
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr
  ) => (
        $crate::declare_lint!(
            $(#[$attr])* $vis $NAME, $Level, $desc,
        );
  );
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr,
    $(@feature_gate = $gate:expr;)? $(
      @future_incompatible = FutureIncompatibleInfo {
        $($field:ident: $val:expr),*
        $(,)*
      };
    )? $($v:ident),*
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: stringify!($NAME),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: None,
            is_plugin: false,
            $($v: true,)*
            $(feature_gate: Some($gate),)*
            $(future_incompatible: Some($crate::FutureIncompatibleInfo {
                $($field: $val,)*
                ..$crate::FutureIncompatibleInfo::default_fields_for_macro()
            }),)*
            ..$crate::Lint::default_fields_for_macro()
        };
  );
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr,
    $lint_edition:expr => $edition_level:ident
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: stringify!($NAME),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: Some(($lint_edition, $crate::Level::$edition_level)),
            report_in_external_macro: false,
            is_plugin: false,
        };
  );
  ($($lint:expr),*,) => { lint_array!( $($lint),* ) };
  ($($lint:expr),*) => {
    {
        vec![$($lint),*]
    }
  };
  ($ty:ty => [$($lint:expr),* $(,)?]) => {
        impl $crate::LintPass for $ty {
            fn name(&self) -> &'static str { stringify!($ty) }
        }
        impl $ty {
            pub fn get_lints() -> $crate::LintArray { $crate::lint_array!($($lint),*) }
        }
  };
  (
    $(#[$m:meta])*
    $name:ident => [$($lint:expr),* $(,)?]
  ) => {
        $(#[$m])* #[derive(Copy, Clone)] pub struct $name;
        $crate::impl_lint_pass!($name => [$($lint),*]);
  };
  ($cfg:meta, $($method:ident),*) => {
    {
        #[cfg($cfg)]
        fn init() {
            extern "C" {
                $(fn $method();)*
            }
            unsafe {
                $($method();)*
            }
        }
        #[cfg(not($cfg))]
        fn init() { }
        init();
    }
  };
  (
    $attr:expr,
    $nested_attr:expr
  ) => {{ throw_invalid_nested_attr!($attr, $nested_attr, |diag| diag) }};
  ($attr:expr, $nested_attr:expr, $f:expr) => {
    {
        let diag = crate::diagnostics::error::invalid_nested_attr($attr, $nested_attr);
        return Err(crate::diagnostics::error::_throw_err(diag, $f));
    }
  };
  (
    $attr:expr,
    $meta:expr
  ) => {{ throw_invalid_attr!($attr, $meta, |diag| diag) }};
  ($attr:expr, $meta:expr, $f:expr) => {
    {
        let diag = crate::diagnostics::error::invalid_attr($attr, $meta);
        return Err(crate::diagnostics::error::_throw_err(diag, $f));
    }
  };
  ($span:expr, $msg:expr) => {{ throw_span_err!($span, $msg, |diag| diag) }};
  ($span:expr, $msg:expr, $f:expr) => {
    {
        let diag = span_err($span, $msg);
        return Err(crate::diagnostics::error::_throw_err(diag, $f));
    }
  };
  ($ty:ty { $(($($pat:tt)*))* }) => {
        impl FixedSizeEncoding for Option<$ty> {
            type ByteArray = [u8;1];

            #[inline]
            fn from_bytes(b: &[u8;1]) -> Self {
                use $ty::*;
                if b[0] == 0 {
                    return None;
                }
                match b[0] - 1 {
                    $(${index()} => Some($($pat)*),)*
                    _ => panic!("Unexpected ImplPolarity code: {:?}", b[0]),
                }
            }

            #[inline]
            fn write_to_bytes(self, b: &mut [u8;1]) {
                use $ty::*;
                b[0] = match self {
                    None => 0,
                    $(Some($($pat)*) => 1 + ${index()},)*
                }
            }
        }
  };
  ($msg:expr) => ({ $crate::util::bug::bug_fmt(::std::format_args!($msg)) });
  ($msg:expr,) => ({ $crate::bug!($msg) });
  (
    $fmt:expr,
    $($arg:tt)+
  ) => (
    {
        $crate::util::bug::bug_fmt(::std::format_args!($fmt, $($arg)+))
    }
  );
  (
    $span:expr,
    $msg:expr
  ) => ({ $crate::util::bug::span_bug_fmt($span, ::std::format_args!($msg)) });
  ($span:expr, $msg:expr,) => ({ $crate::span_bug!($span, $msg) });
  (
    $span:expr,
    $fmt:expr,
    $($arg:tt)+
  ) => (
    {
        $crate::util::bug::span_bug_fmt($span, ::std::format_args!($fmt, $($arg)+))
    }
  );
  (for < $tcx:lifetime > { $($ty:ty,)+ }) => {
        $(
            impl<$tcx> $crate::ty::Lift<$tcx> for $ty {
                type Lifted = Self;
                fn lift_to_tcx(self, _: $crate::ty::TyCtxt<$tcx>) -> Option<Self> {
                    Some(self)
                }
            }
        )+
  };

  ($($ty:ty,)+) => {
        CloneLiftImpls! {
            for <'tcx> {
                $($ty,)+
            }
        }
  };
  (for < $tcx:lifetime > { $($ty:ty,)+ }) => {
        $(
            impl<$tcx> $crate::ty::fold::TypeFoldable<$tcx> for $ty {
                fn try_super_fold_with<F: $crate::ty::fold::FallibleTypeFolder<$tcx>>(
                    self,
                    _: &mut F
                ) -> ::std::result::Result<$ty, F::Error> {
                    Ok(self)
                }

                fn super_visit_with<F: $crate::ty::fold::TypeVisitor<$tcx>>(
                    &self,
                    _: &mut F)
                    -> ::std::ops::ControlFlow<F::BreakTy>
                {
                    ::std::ops::ControlFlow::CONTINUE
                }
            }
        )+
  };

  ($($ty:ty,)+) => {
        TrivialTypeFoldableImpls! {
            for <'tcx> {
                $($ty,)+
            }
        }
  };
  ($($t:tt)*) => {
        TrivialTypeFoldableImpls! { $($t)* }
        CloneLiftImpls! { $($t)* }
  };
  (
    impl < $($p:tt),*
    > TypeFoldable < $tcx:tt > for $s:path { $($variants:tt)* }
    $(
      where
      $($wc:tt)*
    )*
  ) => {
        impl<$($p),*> $crate::ty::fold::TypeFoldable<$tcx> for $s
            $(where $($wc)*)*
        {
            fn try_super_fold_with<V: $crate::ty::fold::FallibleTypeFolder<$tcx>>(
                self,
                folder: &mut V,
            ) -> ::std::result::Result<Self, V::Error> {
                EnumTypeFoldableImpl!(@FoldVariants(self, folder) input($($variants)*) output())
            }

            fn super_visit_with<V: $crate::ty::fold::TypeVisitor<$tcx>>(
                &self,
                visitor: &mut V,
            ) -> ::std::ops::ControlFlow<V::BreakTy> {
                EnumTypeFoldableImpl!(@VisitVariants(self, visitor) input($($variants)*) output())
            }
        }
  };
  ($($ty:ty),+ $(,)?) => {
        $(
            impl $crate::ty::ParameterizedOverTcx for $ty {
                #[allow(unused_lifetimes)]
                type Value<'tcx> = $ty;
            }
        )*
  };
  ($($ident:ident),+ $(,)?) => {
        $(
            impl $crate::ty::ParameterizedOverTcx for $ident<'static> {
                type Value<'tcx> = $ident<'tcx>;
            }
        )*
  };
  ($($field:ident: $method:ident($ty:ty)),+ $(,)?) => (
        impl<'tcx> TyCtxt<'tcx> {
            $(pub fn $method(self, v: &[$ty]) -> &'tcx List<$ty> {
                self.interners.$field.intern_ref(v, || {
                    InternedInSet(List::from_arena(&*self.arena, v))
                }).0
            })+
        }
  );
  ($($name:ident: $method:ident($ty:ty): $ret_ctor:ident -> $ret_ty:ty,)+) => {
        $(impl<'tcx> Borrow<$ty> for InternedInSet<'tcx, $ty> {
            fn borrow<'a>(&'a self) -> &'a $ty {
                &self.0
            }
        }

        impl<'tcx> PartialEq for InternedInSet<'tcx, $ty> {
            fn eq(&self, other: &Self) -> bool {
                // comment
                // comment
                self.0 == other.0
            }
        }

        impl<'tcx> Eq for InternedInSet<'tcx, $ty> {}

        impl<'tcx> Hash for InternedInSet<'tcx, $ty> {
            fn hash<H: Hasher>(&self, s: &mut H) {
                // comment
                // comment
                self.0.hash(s)
            }
        }

        impl<'tcx> TyCtxt<'tcx> {
            pub fn $method(self, v: $ty) -> $ret_ty {
                $ret_ctor(Interned::new_unchecked(self.interners.$name.intern(v, |v| {
                    InternedInSet(self.interners.arena.alloc(v))
                }).0))
            }
        })+
  };
  ($DecoderName:ident < $($typaram:tt),* >) => {
        mod __ty_decoder_impl {
            use std::borrow::Cow;
            use rustc_serialize::Decoder;

            use super::$DecoderName;

            impl<$($typaram ),*> Decoder for $DecoderName<$($typaram),*> {
                $crate::__impl_decoder_methods! {
                    read_u128 -> u128;
                    read_u64 -> u64;
                    read_u32 -> u32;
                    read_u16 -> u16;
                    read_u8 -> u8;
                    read_usize -> usize;

                    read_i128 -> i128;
                    read_i64 -> i64;
                    read_i32 -> i32;
                    read_i16 -> i16;
                    read_i8 -> i8;
                    read_isize -> isize;

                    read_bool -> bool;
                    read_f64 -> f64;
                    read_f32 -> f32;
                    read_char -> char;
                    read_str -> &str;
                }

                #[inline]
                fn read_raw_bytes(&mut self, len: usize) -> &[u8] {
                    self.opaque.read_raw_bytes(len)
                }
            }
        }
  };
  ($($t:ty),+ $(,)?) => {
        $(
            impl<'tcx, E: TyEncoder<I = TyCtxt<'tcx>>> Encodable<E> for ty::Binder<'tcx, $t> {
                fn encode(&self, e: &mut E) -> Result<(), E::Error> {
                    self.bound_vars().encode(e)?;
                    self.as_ref().skip_binder().encode(e)
                }
            }
            impl<'tcx, D: TyDecoder<I = TyCtxt<'tcx>>> Decodable<D> for ty::Binder<'tcx, $t> {
                fn decode(decoder: &mut D) -> Self {
                    let bound_vars = Decodable::decode(decoder);
                    ty::Binder::bind_with_vars(Decodable::decode(decoder), bound_vars)
                }
            }
        )*
  };
  (< $tcx:tt > $($ty:ty,)*) => {
        $(impl<'tcx, D: TyDecoder<I = TyCtxt<'tcx>>> RefDecodable<'tcx, D> for $ty {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decoder.interner().arena.alloc(Decodable::decode(decoder))
            }
        }

        impl<'tcx, D: TyDecoder<I = TyCtxt<'tcx>>> RefDecodable<'tcx, D> for [$ty] {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decoder.interner().arena.alloc_from_iter(<Vec<_> as Decodable<D>>::decode(decoder))
            }
        })*
  };
  ([$($a:tt $name:ident: $ty:ty,)*]) => {
        $(
            impl_arena_allocatable_decoder!($a [$name: $ty]);
        )*
  };
  ([] $args:tt) => {};
  ([decode $(, $attrs:ident)*] [$name:ident: $ty:ty]) => {
        impl<'tcx, D: TyDecoder<I = TyCtxt<'tcx>>> RefDecodable<'tcx, D> for $ty {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decode_arena_allocable(decoder)
            }
        }

        impl<'tcx, D: TyDecoder<I = TyCtxt<'tcx>>> RefDecodable<'tcx, D> for [$ty] {
            #[inline]
            fn decode(decoder: &mut D) -> &'tcx Self {
                decode_arena_allocable_slice(decoder)
            }
        }
  };
  ($($name:ident -> $ty:ty;)*) => {
        $(
            #[inline]
            fn $name(&mut self) -> $ty {
                self.opaque.$name()
            }
        )*
  };
  (@ $lit:literal) => {
        write!(scoped_cx!(), $lit)?
  };
  (@write($($data:expr),+)) => {
        write!(scoped_cx!(), $($data),+)?
  };
  (@print($x:expr)) => {
        scoped_cx!() = $x.print(scoped_cx!())?
  };
  (@$method:ident($($arg:expr),*)) => {
        scoped_cx!() = scoped_cx!().$method($($arg),*)?
  };
  ($($elem:tt $(($($args:tt)*))?),+) => {
    {
        $(p!(@ $elem $(($($args)*))?);)+
    }
  };
  ($cx:ident) => {
        #[allow(unused_macros)]
        macro_rules! scoped_cx {
            () => {
                $cx
            };
        }
  };
  (
    $(
      $(#[$a:meta])*
      fn $name:ident($helper:ident, $tl:ident);
    )+
  ) => {
        $(
            #[must_use]
            pub struct $helper(bool);

            impl $helper {
                pub fn new() -> $helper {
                    $helper($tl.with(|c| c.replace(true)))
                }
            }

            $(#[$a])*
            pub macro $name($e:expr) {
                {
                    let _guard = $helper::new();
                    $e
                }
            }

            impl Drop for $helper {
                fn drop(&mut self) {
                    $tl.with(|c| c.set(self.0))
                }
            }
        )+
  };
  ($($ty:ty),+) => {
        // comment
        $(#[allow(unused_lifetimes)] impl<'tcx> fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                ty::tls::with(|tcx| {
                    let cx = tcx.lift(*self)
                        .expect("could not lift for printing")
                        .print(FmtPrinter::new(tcx, Namespace::TypeNS))?;
                    f.write_str(&cx.into_buffer())?;
                    Ok(())
                })
            }
        })+
  };
  (($self:ident, $cx:ident): $($ty:ty $print:block)+) => {
        $(impl<'tcx, P: PrettyPrinter<'tcx>> Print<'tcx, P> for $ty {
            type Output = P;
            type Error = fmt::Error;
            fn print(&$self, $cx: P) -> Result<Self::Output, Self::Error> {
                #[allow(unused_mut)]
                let mut $cx = $cx;
                define_scoped_cx!($cx);
                let _: () = $print;
                #[allow(unreachable_code)]
                Ok($cx)
            }
        })+

        forward_display_to_print!($($ty),+);
  };
  ($($ty:ty),*) => {
        $(
            impl From<$ty> for ScalarInt {
                #[inline]
                fn from(u: $ty) -> Self {
                    Self {
                        data: u128::from(u),
                        size: std::mem::size_of::<$ty>() as u8,
                    }
                }
            }
        )*
  };
  ($($ty:ty),*) => {
        $(
            impl TryFrom<ScalarInt> for $ty {
                type Error = Size;
                #[inline]
                fn try_from(int: ScalarInt) -> Result<Self, Size> {
                    // comment
                    // comment
                    int.to_bits(Size::from_bytes(std::mem::size_of::<$ty>()))
                       .map(|u| u.try_into().unwrap())
                }
            }
        )*
  };
  (
    < $tcx:tt > $(
      [$($attrs:tt)*]
      $variant:ident $(($tuple_arg_ty:ty $(,)?))*,
    )*
  ) => (
            #[macro_export]
            macro_rules! make_dep_kind_array {
                ($mod:ident) => {[ $($mod::$variant()),* ]};
            }
    
            /// comment
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Encodable, Decodable)]
            #[allow(non_camel_case_types)]
            pub enum DepKind {
                $($variant),*
            }
    
            fn dep_kind_from_label_string(label: &str) -> Result<DepKind, ()> {
                match label {
                    $(stringify!($variant) => Ok(DepKind::$variant),)*
                    _ => Err(()),
                }
            }
    
            /// comment
            /// comment
            #[allow(dead_code, non_upper_case_globals)]
            pub mod label_strs {
               $(
                    pub const $variant: &str = stringify!($variant);
                )*
            }
  );
  ($ty:ty { $(($($pat:tt)*))* }) => {
        impl FixedSizeEncoding for Option<$ty> {
            type ByteArray = [u8;1];

            #[inline]
            fn from_bytes(b: &[u8;1]) -> Self {
                use $ty::*;
                if b[0] == 0 {
                    return None;
                }
                match b[0] - 1 {
                    $(${index()} => Some($($pat)*),)*
                    _ => panic!("Unexpected ImplPolarity code: {:?}", b[0]),
                }
            }

            #[inline]
            fn write_to_bytes(self, b: &mut [u8;1]) {
                use $ty::*;
                b[0] = match self {
                    None => 0,
                    $(Some($($pat)*) => 1 + ${index()},)*
                }
            }
        }
  };
  ($($name:ident: Table < $IDX:ty, $T:ty >),+ $(,)?) => {
        #[derive(MetadataEncodable, MetadataDecodable)]
        pub(crate) struct LazyTables {
            $($name: LazyTable<$IDX, $T>),+
        }

        #[derive(Default)]
        struct TableBuilders {
            $($name: TableBuilder<$IDX, $T>),+
        }

        impl TableBuilders {
            fn encode(&self, buf: &mut Encoder) -> LazyTables {
                LazyTables {
                    $($name: self.$name.encode(buf)),+
                }
            }
        }
  };
  ($($name:ident($ty:ty);)*) => {
        $(fn $name(&mut self, value: $ty) -> Result<(), Self::Error> {
            self.opaque.$name(value)
        })*
  };
  ($self:ident.$tables:ident.$table:ident [$def_id:expr] < -$value:expr) => {
    {
        {
            let value = $value;
            let lazy = $self.lazy(value);
            $self.$tables.$table.set($def_id.index, lazy);
        }
    }
  };
  (
    < $lt:tt > $tcx:ident,
    $def_id:ident,
    $other:ident,
    $cdata:ident,
    $name:ident => { table }
  ) => {
        provide_one! {
            <$lt> $tcx, $def_id, $other, $cdata, $name => {
                $cdata
                    .root
                    .tables
                    .$name
                    .get($cdata, $def_id.index)
                    .map(|lazy| lazy.decode(($cdata, $tcx)))
                    .process_decoded($tcx, || panic!("{:?} does not have a {:?}", $def_id, stringify!($name)))
            }
        }
  };
  (
    < $lt:tt > $tcx:ident,
    $def_id:ident,
    $other:ident,
    $cdata:ident,
    $(
      $name:ident => { $($compute:tt)* }
    )*
  ) => {
          pub fn provide_extern(providers: &mut ExternProviders) {
              $(provide_one! {
                  <$lt> $tcx, $def_id, $other, $cdata, $name => { $($compute)* }
              })*
  
              *providers = ExternProviders {
                  $($name,)*
                  ..*providers
              };
          }
  };
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr
  ) => (
        $crate::declare_lint!(
            $(#[$attr])* $vis $NAME, $Level, $desc,
        );
  );
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr,
    $(@feature_gate = $gate:expr;)? $(
      @future_incompatible = FutureIncompatibleInfo {
        $($field:ident: $val:expr),*
        $(,)*
      };
    )? $($v:ident),*
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: stringify!($NAME),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: None,
            is_plugin: false,
            $($v: true,)*
            $(feature_gate: Some($gate),)*
            $(future_incompatible: Some($crate::FutureIncompatibleInfo {
                $($field: $val,)*
                ..$crate::FutureIncompatibleInfo::default_fields_for_macro()
            }),)*
            ..$crate::Lint::default_fields_for_macro()
        };
  );
  (
    $(#[$attr:meta])*
    $vis:vis $NAME:ident,
    $Level:ident,
    $desc:expr,
    $lint_edition:expr => $edition_level:ident
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: stringify!($NAME),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: Some(($lint_edition, $crate::Level::$edition_level)),
            report_in_external_macro: false,
            is_plugin: false,
        };
  );
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr
  ) => (
        $crate::declare_tool_lint!{$(#[$attr])* $vis $tool::$NAME, $Level, $desc, false}
  );
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr,
    report_in_external_macro: $rep:expr
  ) => (
         $crate::declare_tool_lint!{$(#[$attr])* $vis $tool::$NAME, $Level, $desc, $rep}
  );
  (
    $(#[$attr:meta])*
    $vis:vis $tool:ident::$NAME:ident,
    $Level:ident,
    $desc:expr,
    $external:expr
  ) => (
        $(#[$attr])*
        $vis static $NAME: &$crate::Lint = &$crate::Lint {
            name: &concat!(stringify!($tool), "::", stringify!($NAME)),
            default_level: $crate::$Level,
            desc: $desc,
            edition_lint_opts: None,
            report_in_external_macro: $external,
            future_incompatible: None,
            is_plugin: true,
            feature_gate: None,
            crate_level_only: false,
        };
  );
  ($($lint:expr),*,) => { lint_array!( $($lint),* ) };
  ($($lint:expr),*) => {
    {
        vec![$($lint),*]
    }
  };
  ($ty:ty => [$($lint:expr),* $(,)?]) => {
        impl $crate::LintPass for $ty {
            fn name(&self) -> &'static str { stringify!($ty) }
        }
        impl $ty {
            pub fn get_lints() -> $crate::LintArray { $crate::lint_array!($($lint),*) }
        }
  };
  (
    $(#[$m:meta])*
    $name:ident => [$($lint:expr),* $(,)?]
  ) => {
        $(#[$m])* #[derive(Copy, Clone)] pub struct $name;
        $crate::impl_lint_pass!($name => [$($lint),*]);
  };
  (
    [$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]],
    [$hir:tt],
    $methods:tt
  ) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($passes: $passes,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($passes: $constructor,)*
                }
            }

            $v fn get_lints() -> LintArray {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$passes::get_lints());)*
                lints
            }
        }

        impl<'tcx> LateLintPass<'tcx> for $name {
            expand_combined_late_lint_pass_methods!([$($passes),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
  );
  (
    $passes:tt,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(fn $name(&mut self, context: &LateContext<'tcx>, $($param: $arg),*) {
            expand_combined_late_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
  );
  ([$($passes:ident),*], $self:ident, $name:ident, $params:tt) => (
    {
        $($self.$passes.$name $params;)*
    }
  );
  (
    $context:ty,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(#[inline(always)] fn $name(&mut self, _: $context, $(_: $arg),*) {})*
  );
  (
    $context:ty,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(#[inline(always)] fn $name(&mut self, _: $context, $(_: $arg),*) {})*
  );
  ([$($passes:ident),*], $self:ident, $name:ident, $params:tt) => (
    {
        $($self.$passes.$name $params;)*
    }
  );
  (
    $passes:tt,
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(fn $name(&mut self, context: &EarlyContext<'_>, $($param: $arg),*) {
            expand_combined_early_lint_pass_method!($passes, self, $name, (context, $($param),*));
        })*
  );
  (
    [$v:vis $name:ident, [$($passes:ident: $constructor:expr,)*]],
    $methods:tt
  ) => (
        #[allow(non_snake_case)]
        $v struct $name {
            $($passes: $passes,)*
        }

        impl $name {
            $v fn new() -> Self {
                Self {
                    $($passes: $constructor,)*
                }
            }

            $v fn get_lints() -> LintArray {
                let mut lints = Vec::new();
                $(lints.extend_from_slice(&$passes::get_lints());)*
                lints
            }
        }

        impl EarlyLintPass for $name {
            expand_combined_early_lint_pass_methods!([$($passes),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
  );
  ($cx:expr, $f:ident, $($args:expr),*) => (
    {
        $cx.pass.$f(&$cx.context, $($args),*);
    }
  );
  (
    [
      $(
        $(#[$attr:meta])*
        fn $name:ident($($param:ident: $arg:ty),*);
      )*
    ]
  ) => (
        $(fn $name(&mut self, context: &EarlyContext<'_>, $($param: $arg),*) {
            for obj in self.lints.iter_mut() {
                obj.$name(context, $($param),*);
            }
        })*
  );
  ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
  };
  (
    $(
      $(#[$attr:meta])*
      $variant:ident $($group:expr)?,
      $module:ident::$name:ident,
      $method:ident,
      $target:expr,
      $generics:expr;
    )*
  ) => {

        enum_from_u32! {
            /// comment
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encodable, Decodable)]
            pub enum LangItem {
                $(
                    #[doc = concat!("The `", stringify!($name), "` lang item.")]
                    /// comment
                    $(#[$attr])*
                    $variant,
                )*
            }
        }
        // comment
  };
  (
    $(
      $(#[doc = $doc:tt])*
      (removed, $feature:ident, $ver:expr, $issue:expr, None, $reason:expr),
    )+
  ) => {
        /// comment
        pub const REMOVED_FEATURES: &[Feature] = &[
            $(
                Feature {
                    state: State::Removed { reason: $reason },
                    name: sym::$feature,
                    since: $ver,
                    issue: to_nonzero($issue),
                    edition: None,
                }
            ),+
        ];
  };

  (
    $(
      $(#[doc = $doc:tt])*
      (stable_removed, $feature:ident, $ver:expr, $issue:expr, None),
    )+
  ) => {
        /// comment
        pub const STABLE_REMOVED_FEATURES: &[Feature] = &[
            $(
                Feature {
                    state: State::Stabilized { reason: None },
                    name: sym::$feature,
                    since: $ver,
                    issue: to_nonzero($issue),
                    edition: None,
                }
            ),+
        ];
  };
  (Word) => { template!(@ true, None, None) };
  (List: $descr:expr) => { template!(@ false, Some($descr), None) };
  (NameValueStr: $descr:expr) => { template!(@ false, None, Some($descr)) };
  (Word, List: $descr:expr) => { template!(@ true, Some($descr), None) };
  (
    Word,
    NameValueStr: $descr:expr
  ) => { template!(@ true, None, Some($descr)) };
  (List: $descr1:expr, NameValueStr: $descr2:expr) => {
        template!(@ false, Some($descr1), Some($descr2))
  };
  (Word, List: $descr1:expr, NameValueStr: $descr2:expr) => {
        template!(@ true, Some($descr1), Some($descr2))
  };
  (
    @ $word:expr,
    $list:expr,
    $name_value_str:expr
  ) => { AttributeTemplate {
        word: $word, list: $list, name_value_str: $name_value_str
    } };
  (
    $attr:ident,
    $typ:expr,
    $tpl:expr,
    $duplicates:expr $(, @ only_local: $only_local:expr)? $(,)?
  ) => {
        BuiltinAttribute {
            name: sym::$attr,
            only_local: or_default!(false, $($only_local)?),
            type_: $typ,
            template: $tpl,
            gate: Ungated,
            duplicates: $duplicates,
        }
  };
  ($attr:ident) => {
        concat!("the `#[", stringify!($attr), "]` attribute is an experimental feature")
  };
  (
    $(
      $(#[doc = $doc:tt])*
      ($status:ident, $feature:ident, $ver:expr, $issue:expr, $edition:expr),
    )+
  ) => {
        /// comment
        /// comment
        pub const ACTIVE_FEATURES:
            &[Feature] =
            &[$(
                // comment
                Feature {
                    state: State::Active { set: set!($feature) },
                    name: sym::$feature,
                    since: $ver,
                    issue: to_nonzero($issue),
                    edition: $edition,
                }
            ),+];

        /// comment
        #[derive(Clone, Default, Debug)]
        pub struct Features {
            /// comment
            pub declared_lang_features: Vec<(Symbol, Span, Option<Symbol>)>,
            /// comment
            pub declared_lib_features: Vec<(Symbol, Span)>,
            /// comment
            pub active_features: FxHashSet<Symbol>,
            $(
                $(#[doc = $doc])*
                pub $feature: bool
            ),+
        }

        impl Features {
            pub fn walk_feature_fields(&self, mut f: impl FnMut(&str, bool)) {
                $(f(stringify!($feature), self.$feature);)+
            }

            /// comment
            pub fn active(&self, feature: Symbol) -> bool {
                self.active_features.contains(&feature)
            }

            /// comment
            /// comment
            /// comment
            pub fn enabled(&self, feature: Symbol) -> bool {
                match feature {
                    $( sym::$feature => self.$feature, )*

                    _ => panic!("`{}` was not listed in `declare_features`", feature),
                }
            }

            /// comment
            /// comment
            /// comment
            pub fn incomplete(&self, feature: Symbol) -> bool {
                match feature {
                    $(
                        sym::$feature => declare_features!(__status_to_bool $status),
                    )*
                    // comment
                    _ if self.declared_lang_features.iter().any(|f| f.0 == feature) => false,
                    _ if self.declared_lib_features.iter().any(|f| f.0 == feature) => false,
                    _ => panic!("`{}` was not listed in `declare_features`", feature),
                }
            }
        }
  };
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
  ) => {
        /// comment
        /// comment
        pub enum AstFragment {
            OptExpr(Option<P<ast::Expr>>),
            $($Kind($AstTy),)*
        }

        /// comment
        #[derive(Copy, Clone, PartialEq, Eq)]
        pub enum AstFragmentKind {
            OptExpr,
            $($Kind,)*
        }

        impl AstFragmentKind {
            pub fn name(self) -> &'static str {
                match self {
                    AstFragmentKind::OptExpr => "expression",
                    $(AstFragmentKind::$Kind => $kind_name,)*
                }
            }

            fn make_from<'a>(self, result: Box<dyn MacResult + 'a>) -> Option<AstFragment> {
                match self {
                    AstFragmentKind::OptExpr =>
                        result.make_expr().map(Some).map(AstFragment::OptExpr),
                    $(AstFragmentKind::$Kind => result.$make_ast().map(AstFragment::$Kind),)*
                }
            }
        }

        impl AstFragment {
            pub fn add_placeholders(&mut self, placeholders: &[NodeId]) {
                if placeholders.is_empty() {
                    return;
                }
                match self {
                    $($(AstFragment::$Kind(ast) => ast.extend(placeholders.iter().flat_map(|id| {
                        ${ignore(flat_map_ast_elt)}
                        placeholder(AstFragmentKind::$Kind, *id, None).$make_ast()
                    })),)?)*
                    _ => panic!("unexpected AST fragment kind")
                }
            }

            pub fn make_opt_expr(self) -> Option<P<ast::Expr>> {
                match self {
                    AstFragment::OptExpr(expr) => expr,
                    _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
                }
            }

            $(pub fn $make_ast(self) -> $AstTy {
                match self {
                    AstFragment::$Kind(ast) => ast,
                    _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
                }
            })*

            fn make_ast<T: InvocationCollectorNode>(self) -> T::OutputTy {
                T::fragment_to_output(self)
            }

            pub fn mut_visit_with<F: MutVisitor>(&mut self, vis: &mut F) {
                match self {
                    AstFragment::OptExpr(opt_expr) => {
                        visit_clobber(opt_expr, |opt_expr| {
                            if let Some(expr) = opt_expr {
                                vis.filter_map_expr(expr)
                            } else {
                                None
                            }
                        });
                    }
                    $($(AstFragment::$Kind(ast) => vis.$mut_visit_ast(ast),)?)*
                    $($(AstFragment::$Kind(ast) =>
                        ast.flat_map_in_place(|ast| vis.$flat_map_ast_elt(ast)),)?)*
                }
            }

            pub fn visit_with<'a, V: Visitor<'a>>(&'a self, visitor: &mut V) {
                match *self {
                    AstFragment::OptExpr(Some(ref expr)) => visitor.visit_expr(expr),
                    AstFragment::OptExpr(None) => {}
                    $($(AstFragment::$Kind(ref ast) => visitor.$visit_ast(ast),)?)*
                    $($(AstFragment::$Kind(ref ast) => for ast_elt in &ast[..] {
                        visitor.$visit_ast_elt(ast_elt, $($args)*);
                    })?)*
                }
            }
        }

        impl<'a> MacResult for crate::mbe::macro_rules::ParserAnyMacro<'a> {
            $(fn $make_ast(self: Box<crate::mbe::macro_rules::ParserAnyMacro<'a>>)
                           -> Option<$AstTy> {
                Some(self.make(AstFragmentKind::$Kind).$make_ast())
            })*
        }
  };
  ($($fld:ident: $t:ty,)*) => {
        /// comment
        /// comment
        #[derive(Default)]
        pub struct MacEager {
            $(
                pub $fld: Option<$t>,
            )*
        }

        impl MacEager {
            $(
                pub fn $fld(v: $t) -> Box<dyn MacResult> {
                    Box::new(MacEager {
                        $fld: Some(v),
                        ..Default::default()
                    })
                }
            )*
        }
  };
  (
    $session:expr,
    $span:expr,
    $code:ident,
    $($message:tt)*
  ) => (
    {
        $session.struct_span_err_with_code(
            $span,
            &format!($($message)*),
            $crate::error_code!($code),
        )
    }
  );
  (
    $(#[$attr:meta])*
    pub enum $name:ident {
      $(
        $(#[$var_attr:meta])*
        $variant:ident = $e:expr,
      )*
    }
  ) => {
        $(#[$attr])*
        pub enum $name {
            $($(#[$var_attr])* $variant = $e),*
        }

        impl $name {
            pub fn from_u32(u: u32) -> Option<$name> {
                $(if u == $name::$variant as u32 {
                    return Some($name::$variant)
                })*
                None
            }
        }
  };
  (
    $(#[$attr:meta])*
    pub enum $name:ident {
      $(
        $(#[$var_attr:meta])*
        $variant:ident,
      )*
    }
  ) => {
        $(#[$attr])*
        pub enum $name {
            $($(#[$var_attr])* $variant,)*
        }

        impl $name {
            pub fn from_u32(u: u32) -> Option<$name> {
                $(if u == $name::$variant as u32 {
                    return Some($name::$variant)
                })*
                None
            }
        }
  };
  (
    $e:expr,
    $where:expr,
    $(
      $($p:pat_param)|+ => { $($what_fmt:expr),+ }
      $(expected { $($expected_fmt:expr),+ })?
    ),+
    $(,)?
  ) => {
    {
        match $e {
            Ok(x) => x,
            // comment
            // comment
            Err(e) => match e.kind() {
                $(
                    $($p)|+ =>
                       throw_validation_failure!(
                            $where,
                            { $( $what_fmt ),+ } $( expected { $( $expected_fmt ),+ } )?
                        )
                ),+,
                #[allow(unreachable_patterns)]
                _ => Err::<!, _>(e)?,
            }
        }
    }
  };
  (
    $where:expr,
    { $($what_fmt:expr),+ }
    $(expected { $($expected_fmt:expr),+ })?
  ) => {
    {
        let mut msg = String::new();
        msg.push_str("encountered ");
        write!(&mut msg, $($what_fmt),+).unwrap();
        $(
            msg.push_str(", but expected ");
            write!(&mut msg, $($expected_fmt),+).unwrap();
        )?
        let path = rustc_middle::ty::print::with_no_trimmed_paths!({
            let where_ = &$where;
            if !where_.is_empty() {
                let mut path = String::new();
                write_path(&mut path, where_);
                Some(path)
            } else {
                None
            }
        });
        throw_ub!(ValidationFailure { path, msg })
    }
  };
  ($($name:ident: $($($p:ident),* => $call:ident),*;)*) => {
        $(if name == sym::$name {
            match in_elem.kind() {
                $($(ty::$p(_))|* => {
                    return Ok(bx.$call(args[0].immediate()))
                })*
                _ => {},
            }
            require!(false,
                     "unsupported operation on `{}` with element `{}`",
                     in_ty,
                     in_elem)
        })*
  };
  ($($name:ident: $($($p:ident),* => $call:ident),*;)*) => {
        $(if name == sym::$name {
            match in_elem.kind() {
                $($(ty::$p(_))|* => {
                    return Ok(bx.$call(args[0].immediate(), args[1].immediate()))
                })*
                _ => {},
            }
            require!(false,
                     "unsupported operation on `{}` with element `{}`",
                     in_ty,
                     in_elem)
        })*
  };
  (
    $register:ident;
    $(fn $name:ident($($arg_name:ident: $arg_ty:ty),*) -> $ret_ty:ty;)*
  ) => {
        #[cfg(feature = "jit")]
        #[allow(improper_ctypes)]
        extern "C" {
            $(fn $name($($arg_name: $arg_ty),*) -> $ret_ty;)*
        }

        #[cfg(feature = "jit")]
        pub(crate) fn $register(builder: &mut cranelift_jit::JITBuilder) {
            for (name, val) in [$((stringify!($name), $name as *const u8)),*] {
                builder.symbol(name, val);
            }
        }
  };
  (
    $fx:expr,
    $intrinsic:expr,
    $args:expr,
    _ => $unknown:block;
    $(
      $($($name:tt).*)|+ $(if $cond:expr)?,
      ($($a:ident $arg:ident),*) $content:block;
    )*
  ) => {
        match $intrinsic {
            $(
                $(intrinsic_pat!($($name).*))|* $(if $cond)? => {
                    if let [$($arg),*] = $args {
                        $(intrinsic_arg!($a $fx, $arg);)*
                        $content
                    } else {
                        bug!("wrong number of args for intrinsic {:?}", $intrinsic);
                    }
                }
            )*
            _ => $unknown,
        }
  };
  ($($t:ty => $variant:path,)*) => {
        $(
            impl From<$t> for PrefValue {
                fn from(other: $t) -> Self {
                    $variant(other.into())
                }
            }
        )+
        $(
            impl From<Option<$t>> for PrefValue {
                fn from(other: Option<$t>) -> Self {
                    other.map(|val| $variant(val.into())).unwrap_or(PrefValue::Missing)
                }
            }
        )+
  };
  ($($variant:path => $t:ty,)*) => {
        $(
            impl From<PrefValue> for $t {
                #[allow(unsafe_code)]
                fn from(other: PrefValue) -> Self {
                    if let $variant(value) = other {
                        value.into()
                    } else {
                        panic!("Cannot convert {:?} to {:?}", other, std::any::type_name::<$t>())
                    }
                }
            }
        )+
        $(
            impl From<PrefValue> for Option<$t> {
                fn from(other: PrefValue) -> Self {
                    if let PrefValue::Missing = other {
                        None
                    } else {
                        Some(other.into())
                    }
                }
            }
        )+
  };
  (
    $(
      $variant:path => {
        $($flex_relative_side:ident <= > $flow_relative_side:ident,)+
      },
    )+
  ) => {
        pub fn sides_to_flex_relative<T>(self, flow_relative: Sides<T>) -> FlexRelativeSides<T> {
            match self {
                $(
                    $variant => FlexRelativeSides {
                        $( $flex_relative_side: flow_relative.$flow_relative_side, )+
                    },
                )+
            }
        }

        pub fn sides_to_flow_relative<T>(self, flex_relative: FlexRelativeSides<T>) -> Sides<T> {
            match self {
                $(
                    $variant => Sides {
                        $( $flow_relative_side: flex_relative.$flex_relative_side, )+
                    },
                )+
            }
        }
  };
  ($($name:expr,)+) => {
        {
            $(
                atom_type.atom($name);
            )+
        }
  };
  ($tryer:ident, $getter:ident, $gltype:ty, $glcall:ident, $rstype:ty) => {
        #[allow(unsafe_code)]
        fn $tryer(self, parameter: GLenum) -> Option<$rstype> {
            let mut value = [<$gltype>::default()];
            unsafe {
                self.$glcall(parameter, &mut value);
            }
            if self.get_error() != gl::NO_ERROR {
                None
            } else {
                Some(value[0] as $rstype)
            }
        }

        fn $getter(self, parameter: GLenum) -> $rstype {
            self.$tryer(parameter).unwrap()
        }
  };
  (
    $name:ident {
      $($variant:ident($kind:ident { $($param:ident = gl::$value:ident,)+ }),)+
    }
  ) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
        pub enum $name { $(
            $variant($kind),
        )+}

        $(
            #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
            #[repr(u32)]
            pub enum $kind { $(
                $param = gl::$value,
            )+}
        )+

        impl $name {
            pub fn from_u32(value: u32) -> WebGLResult<Self> {
                match value {
                    $($(gl::$value => Ok($name::$variant($kind::$param)),)+)+
                    _ => Err(WebGLError::InvalidEnum)
                }
            }
        }
  };
  (
    $(
      pub enum $name:ident { $($variant:ident = $mod:ident::$constant:ident,)+ }
    )*
  ) => {
        $(
            #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MallocSizeOf)]
            #[derive(PartialEq, Serialize)]
            #[repr(u32)]
            pub enum $name { $($variant = $mod::$constant,)+ }

            impl $name {
                pub fn from_gl_constant(constant: u32) -> Option<Self> {
                    Some(match constant {
                        $($mod::$constant => $name::$variant, )+
                        _ => return None,
                    })
                }

                #[inline]
                pub fn as_gl_constant(&self) -> u32 {
                    *self as u32
                }
            }
        )*
  };
  ($t1:expr, $t2:expr, $t3:expr, $t4:expr) => {
        (($t1 as u32) << 24) | (($t2 as u32) << 16) | (($t3 as u32) << 8) | ($t4 as u32)
  };
  ($result:expr) => ($result.map_err(|_| (()))?);
  (
    $(
      $variant:path => {
        $($flex_relative_side:ident <= > $flow_relative_side:ident,)+
      },
    )+
  ) => {
        pub fn sides_to_flex_relative<T>(self, flow_relative: Sides<T>) -> FlexRelativeSides<T> {
            match self {
                $(
                    $variant => FlexRelativeSides {
                        $( $flex_relative_side: flow_relative.$flow_relative_side, )+
                    },
                )+
            }
        }

        pub fn sides_to_flow_relative<T>(self, flex_relative: FlexRelativeSides<T>) -> Sides<T> {
            match self {
                $(
                    $variant => Sides {
                        $( $flow_relative_side: flex_relative.$flex_relative_side, )+
                    },
                )+
            }
        }
  };
  (
    $prev:ident,
    $name:ident,
    $($rest:ident,)* [$($tt:tt)*]
  ) => {
        capabilities!($name, $($rest,)* [$($tt)* $name = Self::$prev.bits << 1;]);
  };
  (
    [$($InSelector:tt)*]
    [$($CommonBounds:tt)*]
    [$($FromStr:tt)*]
  ) => {
        /// comment
        /// comment
        pub trait SelectorImpl: Clone + Debug + Sized + 'static {
            type ExtraMatchingData: Sized + Default + 'static;
            type AttrValue: $($InSelector)*;
            type Identifier: $($InSelector)*;
            type LocalName: $($InSelector)* + Borrow<Self::BorrowedLocalName>;
            type NamespaceUrl: $($CommonBounds)* + Default + Borrow<Self::BorrowedNamespaceUrl>;
            type NamespacePrefix: $($InSelector)* + Default;
            type BorrowedNamespaceUrl: ?Sized + Eq;
            type BorrowedLocalName: ?Sized + Eq;

            /// comment
            /// comment
            type NonTSPseudoClass: $($CommonBounds)* + NonTSPseudoClass<Impl = Self>;

            /// comment
            type PseudoElement: $($CommonBounds)* + PseudoElement<Impl = Self>;
        }
  };
  (
    [$($CommonBounds:tt)*]
    [$($FromStr:tt)*]
  ) => {
        with_all_bounds! {
            [$($CommonBounds)* + $($FromStr)* + ToCss]
            [$($CommonBounds)*]
            [$($FromStr)*]
        }
  };
  ($({ $name:ident, $boxed:expr })+) => {
        /// comment
        /// comment
        /// comment
        pub mod computed_values {
            $(
                pub use crate::properties::longhands::$name::computed_value as $name;
            )+
            // comment
            pub use crate::properties::longhands::border_top_style::computed_value as border_style;
        }
  };
  ($($fun:ident = $flag:path;)*) => (
        impl ThreadState {
            /// comment
            pub fn is_worker(self) -> bool {
                self.contains(ThreadState::IN_WORKER)
            }
    
            $(
                #[allow(missing_docs)]
                pub fn $fun(self) -> bool {
                    self.contains($flag)
                }
            )*
        }
  );
  ($self:ident.$checker:ident($value:ident)) => {
        if !$self.$checker(&$value) {
            return false;
        }
  };
  (
    $(
      #[$doc:meta] $name:tt $ident:ident / $setter:ident [$checker:tt]: $ty:ty,
    )+
  ) => {
        /// comment
        #[derive(Clone, Debug, ToShmem)]
        pub struct CounterStyleRuleData {
            name: CustomIdent,
            generation: Wrapping<u32>,
            $(
                #[$doc]
                $ident: Option<$ty>,
            )+
            /// comment
            pub source_location: SourceLocation,
        }

        impl CounterStyleRuleData {
            fn empty(name: CustomIdent, source_location: SourceLocation) -> Self {
                CounterStyleRuleData {
                    name: name,
                    generation: Wrapping(0),
                    $(
                        $ident: None,
                    )+
                    source_location,
                }
            }

            $(
                #[$doc]
                pub fn $ident(&self) -> Option<&$ty> {
                    self.$ident.as_ref()
                }
            )+

            $(
                #[$doc]
                pub fn $setter(&mut self, value: $ty) -> bool {
                    checker!(self.$checker(value));
                    self.$ident = Some(value);
                    self.generation += Wrapping(1);
                    true
                }
            )+
        }
  };
  ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
        /// comment
        #[derive(Clone, Debug, Eq, MallocSizeOf, PartialEq, ToShmem)]
        pub enum NonTSPseudoClass {
            $(
                #[doc = $css]
                $name,
            )*
            /// comment
            Lang(Lang),
            /// comment
            Dir(Direction),
            /// comment
            MozLocaleDir(Direction),
        }
  };
  ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
        match *self {
            $(NonTSPseudoClass::$name => concat!(":", $css),)*
            NonTSPseudoClass::Lang(ref s) => {
                dest.write_str(":lang(")?;
                s.to_css(dest)?;
                return dest.write_char(')');
            },
            NonTSPseudoClass::MozLocaleDir(ref dir) => {
                dest.write_str(":-moz-locale-dir(")?;
                dir.to_css(&mut CssWriter::new(dest))?;
                return dest.write_char(')')
            },
            NonTSPseudoClass::Dir(ref dir) => {
                dest.write_str(":dir(")?;
                dir.to_css(&mut CssWriter::new(dest))?;
                return dest.write_char(')')
            },
        }
  };
  ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
        match_ignore_ascii_case! { &name,
            $($css => Some(NonTSPseudoClass::$name),)*
            "-moz-full-screen" => Some(NonTSPseudoClass::Fullscreen),
            "-moz-read-only" => Some(NonTSPseudoClass::ReadOnly),
            "-moz-read-write" => Some(NonTSPseudoClass::ReadWrite),
            "-moz-focusring" => Some(NonTSPseudoClass::FocusVisible),
            "-moz-ui-valid" => Some(NonTSPseudoClass::UserValid),
            "-moz-ui-invalid" => Some(NonTSPseudoClass::UserInvalid),
            "-webkit-autofill" => Some(NonTSPseudoClass::Autofill),
            _ => None,
        }
  };
  ($t:ty, $addref:path, $release:path) => {
        unsafe impl RefCounted for $t {
            #[inline]
            fn addref(&self) {
                unsafe { $addref(self as *const _ as *mut _) }
            }

            #[inline]
            unsafe fn release(&self) {
                $release(self as *const _ as *mut _)
            }
        }
  };
  ($($name:ident: $value:expr),+) => {
        expanded!( $( $name: $value, )+ )
  };
  ($($name:ident: $value:expr,)+) => {
        Longhands {
            $(
                $name: MaybeBoxed::maybe_boxed($value),
            )+
        }
  };
  (< $t:ident > for $ty:ty) => {
        impl<$t> ListAnimation<$t> for $ty {
            fn animate_repeatable_list(
                &self,
                other: &Self,
                procedure: Procedure,
            ) -> Result<Self, ()>
            where
                T: Animate,
            {
                // comment
                if self.is_empty() || other.is_empty() {
                    return Err(());
                }
                use num_integer::lcm;
                let len = lcm(self.len(), other.len());
                self.iter().cycle().zip(other.iter().cycle()).take(len).map(|(this, other)| {
                    this.animate(other, procedure)
                }).collect()
            }

            fn squared_distance_repeatable_list(
                &self,
                other: &Self,
            ) -> Result<SquaredDistance, ()>
            where
                T: ComputeSquaredDistance,
            {
                if self.is_empty() || other.is_empty() {
                    return Err(());
                }
                use num_integer::lcm;
                let len = lcm(self.len(), other.len());
                self.iter().cycle().zip(other.iter().cycle()).take(len).map(|(this, other)| {
                    this.compute_squared_distance(other)
                }).sum()
            }

            fn animate_with_zero(
                &self,
                other: &Self,
                procedure: Procedure,
            ) -> Result<Self, ()>
            where
                T: Animate + Clone + ToAnimatedZero
            {
                if procedure == Procedure::Add {
                    return Ok(
                        self.iter().chain(other.iter()).cloned().collect()
                    );
                }
                self.iter().zip_longest(other.iter()).map(|it| {
                    match it {
                        EitherOrBoth::Both(this, other) => {
                            this.animate(other, procedure)
                        },
                        EitherOrBoth::Left(this) => {
                            this.animate(&this.to_animated_zero()?, procedure)
                        },
                        EitherOrBoth::Right(other) => {
                            other.to_animated_zero()?.animate(other, procedure)
                        }
                    }
                }).collect()
            }

            fn squared_distance_with_zero(
                &self,
                other: &Self,
            ) -> Result<SquaredDistance, ()>
            where
                T: ToAnimatedZero + ComputeSquaredDistance
            {
                self.iter().zip_longest(other.iter()).map(|it| {
                    match it {
                        EitherOrBoth::Both(this, other) => {
                            this.compute_squared_distance(other)
                        },
                        EitherOrBoth::Left(list) | EitherOrBoth::Right(list) => {
                            list.to_animated_zero()?.compute_squared_distance(list)
                        },
                    }
                }).sum()
            }
        }
  };
  ($name:path) => {
        impl From<u8> for $name {
            fn from(bits: u8) -> $name {
                $name(crate::values::specified::align::AlignFlags::from_bits(bits)
                      .expect("bits contain valid flag"))
            }
        }

        impl From<$name> for u8 {
            fn from(v: $name) -> u8 {
                v.0.bits()
            }
        }
  };
  ($($method:ident($variant:ident => $rule_type:ident),)+) => {
        $(
            #[allow(missing_docs)]
            fn $method<F>(&self, device: &Device, guard: &SharedRwLockReadGuard, mut f: F)
                where F: FnMut(&crate::stylesheets::$rule_type),
            {
                use crate::stylesheets::CssRule;

                for rule in self.effective_rules(device, guard) {
                    if let CssRule::$variant(ref lock) = *rule {
                        let rule = lock.read_with(guard);
                        f(&rule)
                    }
                }
            }
        )+
  };
  ($($variant_name:expr => $variant:ident($data:ident),)+) => {
        declare_viewport_descriptor_inner!([] [ $( $variant_name => $variant($data), )+ ] 0);
  };
  (
    [
      $(
        $assigned_variant_name:expr => $assigned_variant:ident($assigned_data:ident) = $assigned_discriminant:expr,
      )*
    ]
    [
      $next_variant_name:expr => $next_variant:ident($next_data:ident),
      $($variant_name:expr => $variant:ident($data:ident),)*
    ]
    $next_discriminant:expr
  ) => {
        declare_viewport_descriptor_inner! {
            [
                $( $assigned_variant_name => $assigned_variant($assigned_data) = $assigned_discriminant, )*
                $next_variant_name => $next_variant($next_data) = $next_discriminant,
            ]
            [ $( $variant_name => $variant($data), )* ]
            $next_discriminant + 1
        }
  };

  (
    [
      $(
        $assigned_variant_name:expr => $assigned_variant:ident($assigned_data:ident) = $assigned_discriminant:expr,
      )*
    ]
    []
    $number_of_variants:expr
  ) => {
        #[derive(Clone, Debug, PartialEq, ToShmem)]
        #[cfg_attr(feature = "servo", derive(MallocSizeOf))]
        #[allow(missing_docs)]
        pub enum ViewportDescriptor {
            $(
                $assigned_variant($assigned_data),
            )+
        }
        impl ViewportDescriptor {
            #[allow(missing_docs)]
            pub fn discriminant_value(&self) -> usize {
                match *self {
                    $(
                        ViewportDescriptor::$assigned_variant(..) => $assigned_discriminant,
                    )*
                }
            }
        }
  };
  {
    $(
      $(#[$($meta:tt)+])*
      $ident:ident / $css:expr => $gecko:ident = $value:expr,
    )+
  } => {
        bitflags! {
            #[derive(MallocSizeOf, ToResolvedValue, ToShmem)]
            /// comment
            pub struct VariantEastAsian: u16 {
                /// comment
                const NORMAL = 0;
                $(
                    $(#[$($meta)+])*
                    const $ident = $value;
                )+
            }
        }

        impl ToCss for VariantEastAsian {
            fn to_css<W>(&self, dest: &mut CssWriter<W>) {
                $(
                    if self.intersects(VariantEastAsian::$ident) {
                        writer.raw_item($css)?;
                    }
                )+
                Ok(())
            }
        }

        /// comment
        #[cfg(feature = "gecko")]
        #[inline]
        pub fn assert_variant_east_asian_matches() {
            use crate::gecko_bindings::structs;
            $(debug_assert_eq!(structs::$gecko as u16, VariantEastAsian::$ident.bits());)+
        }
        impl SpecifiedValueInfo for VariantEastAsian {
            fn collect_completion_keywords(f: KeywordsCollectFn) {
                f(&["normal", $($css,)+]);
            }
        }
  };
  (
    $parser:ident,
    $abs:ident,
    $enum:ident,
    [$para:ident => $func:ident $(, $other_para:ident => $other_func:ident)*]
  ) => {
        {
            loop {
                let $para = $func(&mut $parser.chars)?;
                $(
                    skip_comma_wsp(&mut $parser.chars);
                    let $other_para = $other_func(&mut $parser.chars)?;
                )*
                $parser.path.push(PathCommand::$enum { $para $(, $other_para)*, $abs });

                // comment
                if !skip_wsp(&mut $parser.chars) ||
                   $parser.chars.peek().map_or(true, |c| c.is_ascii_alphabetic()) {
                    break;
                }
                skip_comma_wsp(&mut $parser.chars);
            }
            Ok(())
        }
  };
  (pub enum $name:ident { $($variant:ident = $css:expr,)+ }) => {
        #[allow(missing_docs)]
        #[cfg_attr(feature = "servo", derive(Deserialize, Serialize))]
        #[derive(Clone, Copy, Debug, Eq, Hash, MallocSizeOf, PartialEq, ToShmem)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            /// comment
            pub fn parse<'i, 't>(input: &mut ::cssparser::Parser<'i, 't>) -> Result<$name, $crate::ParseError<'i>> {
                use cssparser::Token;
                let location = input.current_source_location();
                match *input.next()? {
                    Token::Ident(ref ident) => {
                        Self::from_ident(ident).map_err(|()| {
                            location.new_unexpected_token_error(
                                Token::Ident(ident.clone()),
                            )
                        })
                    }
                    ref token => {
                        Err(location.new_unexpected_token_error(token.clone()))
                    }
                }
            }

            /// comment
            pub fn from_ident(ident: &str) -> Result<$name, ()> {
                match_ignore_ascii_case! { ident,
                    $($css => Ok($name::$variant),)+
                    _ => Err(())
                }
            }
        }

        impl $crate::ToCss for $name {
            fn to_css<W>(
                &self,
                dest: &mut $crate::CssWriter<W>,
            ) -> ::std::fmt::Result
            where
                W: ::std::fmt::Write,
            {
                match *self {
                    $( $name::$variant => ::std::fmt::Write::write_str(dest, $css) ),+
                }
            }
        }
  };
  ($($ty:ty),*) => {
        $(
            impl $crate::ToShmem for $ty {
                fn to_shmem( &self, _builder: &mut $crate::SharedMemoryBuilder, ) -> $crate::Result<Self> {
                    $crate::Result::Ok(::std::mem::ManuallyDrop::new(*self))
                }
            }
        )*
  };
  ($($fun:ident = $flag:ident;)*) => ();
  ($($name:expr,)+) => {
        {
            $(
                atom_type.atom($name);
            )+
        }
  };
  (
    $name:ident {
      $($variant:ident($kind:ident { $($param:ident = gl::$value:ident,)+ }),)+
    }
  ) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
        pub enum $name { $(
            $variant($kind),
        )+}

        $(
            #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
            #[repr(u32)]
            pub enum $kind { $(
                $param = gl::$value,
            )+}
        )+

        impl $name {
            pub fn from_u32(value: u32) -> WebGLResult<Self> {
                match value {
                    $($(gl::$value => Ok($name::$variant($kind::$param)),)+)+
                    _ => Err(WebGLError::InvalidEnum)
                }
            }
        }
  };
  (
    $(
      pub enum $name:ident { $($variant:ident = $mod:ident::$constant:ident,)+ }
    )*
  ) => {
        $(
            #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, MallocSizeOf)]
            #[derive(PartialEq, Serialize)]
            #[repr(u32)]
            pub enum $name { $($variant = $mod::$constant,)+ }

            impl $name {
                pub fn from_gl_constant(constant: u32) -> Option<Self> {
                    Some(match constant {
                        $($mod::$constant => $name::$variant, )+
                        _ => return None,
                    })
                }

                #[inline]
                pub fn as_gl_constant(&self) -> u32 {
                    *self as u32
                }
            }
        )*
  };
  ($({ $name:ident, $boxed:expr })+) => {
        /// comment
        /// comment
        /// comment
        pub mod computed_values {
            $(pub use crate::properties::longhands::$name::computed_value as $name;)+
            // comment
            pub use crate::properties::longhands::border_top_style::computed_value as border_style;
        }
  };
  ($($fun:ident = $flag:path;)*) => (
        impl ThreadState {
            /// comment
            pub fn is_worker(self) -> bool {
                self.contains(ThreadState::IN_WORKER)
            }
    
            $(
                #[allow(missing_docs)]
                pub fn $fun(self) -> bool {
                    self.contains($flag)
                }
            )*
        }
  );
  ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
        /// comment
        #[derive(Clone, Debug, Eq, MallocSizeOf, PartialEq, ToShmem)]
        pub enum NonTSPseudoClass {
            $(
                #[doc = $css]
                $name,
            )*
        }
  };
  ($($name:ident: $value:expr),+) => {
        expanded!( $( $name: $value, )+ )
  };
  ($($name:ident: $value:expr,)+) => {
        Longhands { $( $name: MaybeBoxed::maybe_boxed($value), )+ }
  };
  ($($variant_name:expr => $variant:ident($data:ident),)+) => {
        declare_viewport_descriptor_inner!([] [ $( $variant_name => $variant($data), )+ ] 0);
  };
  (
    [
      $(
        $assigned_variant_name:expr => $assigned_variant:ident($assigned_data:ident) = $assigned_discriminant:expr,
      )*
    ]
    [
      $next_variant_name:expr => $next_variant:ident($next_data:ident),
      $($variant_name:expr => $variant:ident($data:ident),)*
    ]
    $next_discriminant:expr
  ) => {
        declare_viewport_descriptor_inner! {
            [
                $( $assigned_variant_name => $assigned_variant($assigned_data) = $assigned_discriminant, )*
                $next_variant_name => $next_variant($next_data) = $next_discriminant,
            ]
            [ $( $variant_name => $variant($data), )* ]
            $next_discriminant + 1
        }
  };
  ($($name:expr,)+) => {
        f(&["symbols", $($name,)+])
  };
  (
    $parser:ident,
    $abs:ident,
    $enum:ident,
    [$para:ident => $func:ident $(, $other_para:ident => $other_func:ident)*]
  ) => {
        {
            loop {
                let $para = $func(&mut $parser.chars)?;
                $(
                    skip_comma_wsp(&mut $parser.chars);
                    let $other_para = $other_func(&mut $parser.chars)?;
                )*
                $parser.path.push(PathCommand::$enum { $para $(, $other_para)*, $abs });

                // comment
                if !skip_wsp(&mut $parser.chars) ||
                   $parser.chars.peek().map_or(true, |c| c.is_ascii_alphabetic()) {
                    break;
                }
                skip_comma_wsp(&mut $parser.chars);
            }
            Ok(())
        }
  };
  ($({ $name:ident, $boxed:expr })+) => {
        $(
            let size = size_of::<style::properties::longhands::$name::SpecifiedValue>();
            let is_boxed = $boxed;
            if (!is_boxed && size > threshold) || (is_boxed && size <= threshold) {
                bad_properties.push((stringify!($name), size, is_boxed));
            }
        )+
  };

  (
    $vis:vis const $name:ident: $ty:ty = $e:expr;
  ) => { $vis const $name: $ty = $e; };
  ($vis:vis enum $name:ident {}) => { $vis struct $name {} };
  (
    $vis:vis extern "C" fn $name:ident()
    {}
  ) => { $vis extern "C" fn $name() {} };
  ($vis:vis fn $name:ident() {}) => { $vis fn $name() {} };
  ($vis:vis mod $name:ident {}) => { $vis mod $name {} };
  (
    $vis:vis static $name:ident: $ty:ty = $e:expr;
  ) => { $vis static $name: $ty = $e; };
  ($vis:vis struct $name:ident;) => { $vis struct $name; };
  ($vis:vis trait $name:ident {}) => { $vis trait $name {} };
  ($vis:vis type $name:ident = $ty:ty;) => { $vis type $name = $ty; };
  (
    $vis:vis use $path:ident as $name:ident;
  ) => { $vis use self::$path as $name; };

  (
    $(#[$($attrs:tt)*])*
    $vis:vis struct $name:ident { $($body:tt)* }
  ) => {
        c! { @parse_fields $(#[$($attrs)*])*, $vis, $name, $($body)* }
  };

  (
    $(#[$($attrs:tt)*])*
    $vis:vis struct $name:ident($($body:tt)*);
  ) => {
        c! { @parse_tuple $(#[$($attrs)*])*, $vis, $name, $($body)* }
  };

  (
    @parse_fields $(#[$attrs:meta])*,
    $vis:vis,
    $name:ident,
    $($fvis:vis $fname:ident: $fty:ty),*
    $(,)*
  ) => {
        $(#[$attrs])* $vis struct $name { $($fvis $fname: $fty,)* }
  };

  (
    @parse_tuple $(#[$attrs:meta])*,
    $vis:vis,
    $name:ident,
    $($fvis:vis $fty:ty),*
    $(,)*
  ) => {
        $(#[$attrs])* $vis struct $name ( $($fvis $fty,)* );
  };
  (
    $(
      $(#[$attr:meta])*
      struct $Name:ident impl $(< $($lifetime:lifetime),+ >)? Fn = | $(
        $arg:ident: $ArgTy:ty
      ),*
      | -> $ReturnTy:ty $body:block;
    )+
  ) => {
        $(
            $( #[$attr] )*
            struct $Name;

            impl $( <$( $lifetime ),+> )? Fn<($( $ArgTy, )*)> for $Name {
                #[inline]
                extern "rust-call" fn call(&self, ($( $arg, )*): ($( $ArgTy, )*)) -> $ReturnTy {
                    $body
                }
            }

            impl $( <$( $lifetime ),+> )? FnMut<($( $ArgTy, )*)> for $Name {
                #[inline]
                extern "rust-call" fn call_mut(
                    &mut self,
                    ($( $arg, )*): ($( $ArgTy, )*)
                ) -> $ReturnTy {
                    Fn::call(&*self, ($( $arg, )*))
                }
            }

            impl $( <$( $lifetime ),+> )? FnOnce<($( $ArgTy, )*)> for $Name {
                type Output = $ReturnTy;

                #[inline]
                extern "rust-call" fn call_once(self, ($( $arg, )*): ($( $ArgTy, )*)) -> $ReturnTy {
                    Fn::call(&self, ($( $arg, )*))
                }
            }
        )+
  };
  (
    $(
      if #[cfg($i_meta:meta)]
      { $($i_tokens:tt)* }
    )else+
    else { $($e_tokens:tt)* }
  ) => {
        cfg_if! {
            @__items () ;
            $(
                (( $i_meta ) ( $( $i_tokens )* )) ,
            )+
            (() ( $( $e_tokens )* )) ,
        }
  };
  (@__items($($_:meta,)*);) => {};
  (
    @__items($($no:meta,)*);
    (
      ($($yes:meta)?)
      ($($tokens:tt)*)
    ),
    $($rest:tt,)*
  ) => {
        #[cfg(all(
            $( $yes , )?
            not(any( $( $no ),* ))
        ))]
        cfg_if! { @__identity $( $tokens )* }
        cfg_if! {
            @__items ( $( $no , )* $( $yes , )? ) ;
            $( $rest , )*
        }
  };
  (
    @__identity
    $($tokens:tt)*
  ) => {
        $( $tokens )*
  };
  (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        forward_ref_op_assign!(impl $imp, $method for $t, $u,
                #[stable(feature = "op_assign_builtins_by_ref", since = "1.22.0")]);
  };
  (
    impl const $imp:ident,
    $method:ident for $t:ty,
    $u:ty
  ) => {
        forward_ref_op_assign!(impl const $imp, $method for $t, $u,
                #[stable(feature = "op_assign_builtins_by_ref", since = "1.22.0")]);
  };
  // comment
  (
    impl const $imp:ident,
    $method:ident for $t:ty,
    $u:ty,
    #[$attr:meta]
  ) => {
        #[$attr]
        #[rustc_const_unstable(feature = "const_ops", issue = "90080")]
        impl const $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
  };
  (impl $imp:ident, $method:ident for $t:ty, $u:ty, #[$attr:meta]) => {
        #[$attr]
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
  };
  (@impl $($T:ident)+) => {
        maybe_tuple_doc! {
            $($T)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:PartialEq),+> PartialEq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized
            {
                #[inline]
                fn eq(&self, other: &($($T,)+)) -> bool {
                    $( ${ignore(T)} self.${index()} == other.${index()} )&&+
                }
                #[inline]
                fn ne(&self, other: &($($T,)+)) -> bool {
                    $( ${ignore(T)} self.${index()} != other.${index()} )||+
                }
            }
        }

        maybe_tuple_doc! {
            $($T)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Eq),+> Eq for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized
            {}
        }

        maybe_tuple_doc! {
            $($T)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:PartialOrd + PartialEq),+> PartialOrd for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized
            {
                #[inline]
                fn partial_cmp(&self, other: &($($T,)+)) -> Option<Ordering> {
                    lexical_partial_cmp!($( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
                #[inline]
                fn lt(&self, other: &($($T,)+)) -> bool {
                    lexical_ord!(lt, $( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
                #[inline]
                fn le(&self, other: &($($T,)+)) -> bool {
                    lexical_ord!(le, $( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
                #[inline]
                fn ge(&self, other: &($($T,)+)) -> bool {
                    lexical_ord!(ge, $( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
                #[inline]
                fn gt(&self, other: &($($T,)+)) -> bool {
                    lexical_ord!(gt, $( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
            }
        }

        maybe_tuple_doc! {
            $($T)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Ord),+> Ord for ($($T,)+)
            where
                last_type!($($T,)+): ?Sized
            {
                #[inline]
                fn cmp(&self, other: &($($T,)+)) -> Ordering {
                    lexical_cmp!($( ${ignore(T)} self.${index()}, other.${index()} ),+)
                }
            }
        }

        maybe_tuple_doc! {
            $($T)+ @
            #[stable(feature = "rust1", since = "1.0.0")]
            impl<$($T:Default),+> Default for ($($T,)+) {
                #[inline]
                fn default() -> ($($T,)+) {
                    ($({ let x: $T = Default::default(); x},)+)
                }
            }
        }
  };
  ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = $crate::panicking::AssertKind::Eq;
                    // comment
                    // comment
                    // comment
                    $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::None);
                }
            }
        }
  };
  (
    $left:expr,
    $right:expr,
    $($arg:tt)+
  ) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = $crate::panicking::AssertKind::Eq;
                    // comment
                    // comment
                    // comment
                    $crate::panicking::assert_failed(kind, &*left_val, &*right_val, $crate::option::Option::Some($crate::format_args!($($arg)+)));
                }
            }
        }
  };
  ($left:expr, $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                $crate::panicking::assert_matches_failed(
                    left_val,
                    $crate::stringify!($($pattern)|+ $(if $guard)?),
                    $crate::option::Option::None
                );
            }
        }
  };
  (
    $left:expr,
    $(|)? $($pattern:pat_param)|+ $(if $guard:expr)?,
    $($arg:tt)+
  ) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                $crate::panicking::assert_matches_failed(
                    left_val,
                    $crate::stringify!($($pattern)|+ $(if $guard)?),
                    $crate::option::Option::Some($crate::format_args!($($arg)+))
                );
            }
        }
  };
  (
    $expression:expr,
    $(|)? $($pattern:pat_param)|+ $(if $guard:expr)? $(,)?
  ) => {
        match $expression {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
  };
  (
    enum $name:ident < $($T:ident),+
    > { $($variant:ident $(($field:ident))?),* $(,)? }
  ) => {
        impl<$($T: Mark),+> Mark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn mark(unmarked: Self::Unmarked) -> Self {
                match unmarked {
                    $($name::$variant $(($field))? => {
                        $name::$variant $((Mark::mark($field)))?
                    })*
                }
            }
        }

        impl<$($T: Unmark),+> Unmark for $name <$($T),+> {
            type Unmarked = $name <$($T::Unmarked),+>;
            fn unmark(self) -> Self::Unmarked {
                match self {
                    $($name::$variant $(($field))? => {
                        $name::$variant $((Unmark::unmark($field)))?
                    })*
                }
            }
        }
  };
  (struct $name:ident $(< $($T:ident),+ >)? { $($field:ident),* $(,)? }) => {
        impl<S, $($($T: Encode<S>),+)?> Encode<S> for $name $(<$($T),+>)? {
            fn encode(self, w: &mut Writer, s: &mut S) {
                $(self.$field.encode(w, s);)*
            }
        }

        impl<'a, S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
            for $name $(<$($T),+>)?
        {
            fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
                $name {
                    $($field: DecodeMut::decode(r, s)),*
                }
            }
        }
  };
  (
    enum $name:ident $(< $($T:ident),+ >)? {
      $($variant:ident $(($field:ident))*),* $(,)?
    }
  ) => {
        impl<S, $($($T: Encode<S>),+)?> Encode<S> for $name $(<$($T),+>)? {
            fn encode(self, w: &mut Writer, s: &mut S) {
                // comment
                // comment
                #[allow(non_upper_case_globals)]
                mod tag {
                    #[repr(u8)] enum Tag { $($variant),* }

                    $(pub const $variant: u8 = Tag::$variant as u8;)*
                }

                match self {
                    $($name::$variant $(($field))* => {
                        tag::$variant.encode(w, s);
                        $($field.encode(w, s);)*
                    })*
                }
            }
        }

        impl<'a, S, $($($T: for<'s> DecodeMut<'a, 's, S>),+)?> DecodeMut<'a, '_, S>
            for $name $(<$($T),+>)?
        {
            fn decode(r: &mut Reader<'a>, s: &mut S) -> Self {
                // comment
                // comment
                #[allow(non_upper_case_globals)]
                mod tag {
                    #[repr(u8)] enum Tag { $($variant),* }

                    $(pub const $variant: u8 = Tag::$variant as u8;)*
                }

                match u8::decode(r, s) {
                    $(tag::$variant => {
                        $(let $field = DecodeMut::decode(r, s);)*
                        $name::$variant $(($field))*
                    })*
                    _ => unreachable!(),
                }
            }
        }
  };
  (
    $(
      fn $name:ident $(< $($param:ident),* >)? for $(extern $abi:tt)? fn(
        $($arg:ident: $arg_ty:ty),*
      ) -> $ret_ty:ty;
    )+
  ) => {
        $(pub const fn $name<
            $($($param,)*)?
            F: Fn($($arg_ty),*) -> $ret_ty + Copy
        >(f: F) -> $(extern $abi)? fn($($arg_ty),*) -> $ret_ty {
            // comment
            // comment
            assert!(mem::size_of::<F>() == 0, "selfless_reify: closure must be zero-sized");

            $(extern $abi)? fn wrapper<
                $($($param,)*)?
                F: Fn($($arg_ty),*) -> $ret_ty + Copy
            >($($arg: $arg_ty),*) -> $ret_ty {
                let f = unsafe {
                    // comment
                    // comment
                    mem::MaybeUninit::<F>::uninit().assume_init()
                };
                f($($arg),*)
            }
            let _f_proof = f;
            wrapper::<
                $($($param,)*)?
                F
            >
        })+
  };
  (
    fn drop(&mut self, $arg:ident: $arg_ty:ty)
  ) => (fn drop(&mut self, $arg: $arg_ty) { mem::drop($arg) });

  (
    fn clone(&mut self, $arg:ident: $arg_ty:ty) -> $ret_ty:ty
  ) => (fn clone(&mut self, $arg: $arg_ty) -> $ret_ty { $arg.clone() });

  ($e:expr) => {
    {
        loop {
            match $e {
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                res => break res,
            }
        }
    }
  };
  ($($x:ident),*) => {
    $(
        // comment
    )*
  };
}

macro path_local($x:ident) {
    generic::ty::Path::new_local(sym::$x)
}

macro pathvec_std($($rest:ident)::+) {
    {
    vec![ $( sym::$rest ),+ ]
    }
}

macro path_std($($x:tt)*) {
    generic::ty::Path::new( pathvec_std!( $($x)* ) )
}
macro takes_u32_returns_u32($ident:ident) {
    fn $ident(arg: u32) -> u32;
}
macro simple_nonterminal($nt_ident:ident, $nt_lifetime:lifetime, $nt_tt:tt) {
    macro n(a $nt_ident b $nt_lifetime c $nt_tt d) {
        struct S;
    }

    n!(a $nt_ident b $nt_lifetime c $nt_tt d);
}
macro complex_nonterminal($nt_item:item) {
    macro n(a $nt_item b) {
        struct S;
    }

    n!(a $nt_item b); // comment
}
// source: "../../../ext/jinx-rust/tests/samples/macro/macro.transform.rs"