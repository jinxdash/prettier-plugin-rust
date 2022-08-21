macro_rules! a {
  (+) => { + };
  (+=) => { += };
  (&) => { & };
  (&&) => { && };
  (&=) => { &= };
  (@) => { @ };
  (!) => { ! };
  (^) => { ^ };
  (^=) => { ^= };
  (:) => { : };
  (::) => { :: };
  (,) => { , };
  (/) => { / };
  (/=) => { /= };
  (.) => { . };
  (..) => { .. };
  (...) => { ... };
  (..=) => { ..= };
  (=) => { = };
  (==) => { == };
  (>=) => { >= };
  (>) => { > };
  (<=) => { <= };
  (<) => { < };
  (*=) => { *= };
  (!=) => { != };
  (|) => { | };
  (|=) => { |= };
  (||) => { || };
  (#) => { # };
  (?) => { ? };
  (->) => { -> };
  (< -) => { <- };
  (%) => { % };
  (%=) => { %= };
  (=>) => { => };
  (;) => { ; };
  (<<) => { << };
  (<<=) => { <<= };
  (>>) => { >> };
  (>>=) => { >>= };
  (*) => { * };
  (-) => { - };
  (-=) => { -= };
  (~) => { ~ };
}

a! { + }
a! { += }
a! { & }
a! { && }
a! { &= }
a! { @ }
a! { ! }
a! { ^ }
a! { ^= }
a! { : }
a! { :: }
a! { , }
a! { / }
a! { /= }
a! { . }
a! { .. }
a! { ... }
a! { ..= }
a! { = }
a! { == }
a! { >= }
a! { > }
a! { <= }
a! { < }
a! { *= }
a! { != }
a! { | }
a! { |= }
a! { || }
a! { # }
a! { ? }
a! { -> }
a! { <- }
a! { % }
a! { %= }
a! { => }
a! { ; }
a! { << }
a! { <<= }
a! { >> }
a! { >>= }
a! { * }
a! { - }
a! { -= }
a! { ~ }

b![+];
b![+=];
b![&];
b![&&];
b![&=];
b![@];
b![!];
b![^];
b![^=];
b![:];
b![::];
b![,];
b![/];
b![/=];
b![.];
b![..];
b![...];
b![..=];
b![=];
b![==];
b![>=];
b![>];
b![<=];
b![<];
b![*=];
b![!=];
b![|];
b![|=];
b![||];
b![#];
b![?];
b![->];
b![<-];
b![%];
b![%=];
b![=>];
b![;];
b![<<];
b![<<=];
b![>>];
b![>>=];
b![*];
b![-];
b![-=];
b![~];

c!(+);
c!(+=);
c!(&);
c!(&&);
c!(&=);
c!(@);
c!(!);
c!(^);
c!(^=);
c!(:);
c!(::);
c!(,);
c!(/);
c!(/=);
c!(.);
c!(..);
c!(...);
c!(..=);
c!(=);
c!(==);
c!(>=);
c!(>);
c!(<=);
c!(<);
c!(*=);
c!(!=);
c!(|);
c!(|=);
c!(||);
c!(#);
c!(?);
c!(->);
c!(<-);
c!(%);
c!(%=);
c!(=>);
c!(;);
c!(<<);
c!(<<=);
c!(>>);
c!(>>=);
c!(*);
c!(-);
c!(-=);
c!(~);

macro_rules! x {
  ($p:pat =>) => {};
  ($p:pat,) => {};
  ($p:pat =) => {};
  ($p:pat |) => {};
  ($p:pat if) => {};
  ($p:pat in) => {};
  ($e:expr =>) => {};
  ($e:expr,) => {};
  ($e:expr;) => {};
  ($t:ty {}) => {};
  ($t:ty,) => {};
  ($t:ty =>) => {};
  ($t:ty:) => {};
  ($t:ty =) => {};
  ($t:ty >) => {};
  ($t:ty;) => {};
  ($t:ty |) => {};
  ($t:ty as) => {};
  ($t:ty where) => {};
  ($t:ty []) => {};
  ($t:ty $b:block) => {};
  ($s:stmt =>) => {};
  ($s:stmt,) => {};
  ($s:stmt;) => {};
  ($p:path {}) => {};
  ($p:path,) => {};
  ($p:path =>) => {};
  ($p:path:) => {};
  ($p:path =) => {};
  ($p:path >) => {};
  ($p:path;) => {};
  ($p:path |) => {};
  ($p:path as) => {};
  ($p:path where) => {};
  ($p:path []) => {};
  ($p:path $b:block) => {};
  ($b:block ()) => {};
  ($b:block []) => {};
  ($b:block {}) => {};
  ($b:block,) => {};
  ($b:block =>) => {};
  ($b:block:) => {};
  ($b:block =) => {};
  ($b:block >) => {};
  ($b:block;) => {};
  ($b:block |) => {};
  ($b:block +) => {};
  ($b:block ident) => {};
  ($b:block $p:pat) => {};
  ($b:block $e:expr) => {};
  ($b:block $t:ty) => {};
  ($b:block $s:stmt) => {};
  ($b:block $p:path) => {};
  ($b:block $c:block) => {};
  ($b:block $i:ident) => {};
  ($b:block $t:tt) => {};
  ($b:block $i:item) => {};
  ($b:block $m:meta) => {};
  ($i:ident()) => {};
  ($i:ident []) => {};
  ($i:ident {}) => {};
  ($i:ident,) => {};
  ($i:ident =>) => {};
  ($i:ident:) => {};
  ($i:ident =) => {};
  ($i:ident >) => {};
  ($i:ident;) => {};
  ($i:ident |) => {};
  ($i:ident +) => {};
  ($i:ident ident) => {};
  ($i:ident $p:pat) => {};
  ($i:ident $e:expr) => {};
  ($i:ident $t:ty) => {};
  ($i:ident $s:stmt) => {};
  ($i:ident $p:path) => {};
  ($i:ident $b:block) => {};
  ($i:ident $j:ident) => {};
  ($i:ident $t:tt) => {};
  ($i:ident $j:item) => {};
  ($i:ident $m:meta) => {};
  ($t:tt ()) => {};
  ($t:tt []) => {};
  ($t:tt {}) => {};
  ($t:tt,) => {};
  ($t:tt =>) => {};
  ($t:tt:) => {};
  ($t:tt =) => {};
  ($t:tt >) => {};
  ($t:tt;) => {};
  ($t:tt |) => {};
  ($t:tt +) => {};
  ($t:tt ident) => {};
  ($t:tt $p:pat) => {};
  ($t:tt $e:expr) => {};
  ($t:tt $v:ty) => {};
  ($t:tt $s:stmt) => {};
  ($t:tt $p:path) => {};
  ($t:tt $b:block) => {};
  ($t:tt $i:ident) => {};
  ($t:tt $v:tt) => {};
  ($t:tt $i:item) => {};
  ($t:tt $m:meta) => {};
  ($i:item ()) => {};
  ($i:item []) => {};
  ($i:item {}) => {};
  ($i:item,) => {};
  ($i:item =>) => {};
  ($i:item:) => {};
  ($i:item =) => {};
  ($i:item >) => {};
  ($i:item;) => {};
  ($i:item |) => {};
  ($i:item +) => {};
  ($i:item ident) => {};
  ($i:item $p:pat) => {};
  ($i:item $e:expr) => {};
  ($i:item $t:ty) => {};
  ($i:item $s:stmt) => {};
  ($i:item $p:path) => {};
  ($i:item $b:block) => {};
  ($i:item $j:ident) => {};
  ($i:item $t:tt) => {};
  ($i:item $j:item) => {};
  ($i:item $m:meta) => {};
  ($m:meta ()) => {};
  ($m:meta []) => {};
  ($m:meta {}) => {};
  ($m:meta,) => {};
  ($m:meta =>) => {};
  ($m:meta:) => {};
  ($m:meta =) => {};
  ($m:meta >) => {};
  ($m:meta;) => {};
  ($m:meta |) => {};
  ($m:meta +) => {};
  ($m:meta ident) => {};
  ($m:meta $p:pat) => {};
  ($m:meta $e:expr) => {};
  ($m:meta $t:ty) => {};
  ($m:meta $s:stmt) => {};
  ($m:meta $p:path) => {};
  ($m:meta $b:block) => {};
  ($m:meta $i:ident) => {};
  ($m:meta $t:tt) => {};
  ($m:meta $i:item) => {};
  ($m:meta $n:meta) => {};
  ($ty:ty <) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
  ($ty:ty < foo,) => (); //~ ERROR `$ty:ty` is followed by `<`, which is not allowed for `ty`
  ($ty:ty,) => ();
  (($ty:ty)) => ();
  ({ $ty:ty }) => ();
  ([$ty:ty]) => ();
  ($bl:block <) => ();
  ($pa:pat >) => (); //~ ERROR `$pa:pat` is followed by `>`, which is not allowed for `pat`
  ($pa:pat,) => ();
  ($pa:pat $pb:pat $ty:ty,) => ();
  //~^ ERROR `$pa:pat` is followed by `$pb:pat`, which is not allowed
  //~^^ ERROR `$pb:pat` is followed by `$ty:ty`, which is not allowed
  ($($ty:ty)* -) => (); //~ ERROR `$ty:ty` is followed by `-`
  ($($a:ty, $b:ty)* -) => (); //~ ERROR `$b:ty` is followed by `-`
  ($($ty:ty)-+) => (); //~ ERROR `$ty:ty` is followed by `-`, which is not allowed for `ty`
  (
    $($a:expr)*
    $($b:tt)*
  ) => { };
  //~^ ERROR `$a:expr` is followed by `$b:tt`, which is not allowed for `expr` fragments
}

#![rustc_dummy("hi", 1, 2, 1.012, pi = 3.14, bye, name("John"))]
#[rustfmt::r#final(final)]
lexes! { a #foo }
lexes! {
  continue 'foo;
}
lexes! {
  match "..." {
  }
}
lexes! { r#let#foo } // Identifier<"r#let"; raw: true> PunctuationToken<#> Identifier<"foo">

fn f() {
  unsafe {
    asm!("");
    asm!("", options());
    asm!("", options(nostack, nomem));
    asm!("{}", in(reg) 4);
    asm!("{0}", out(reg) a);
    asm!("{name}", name = inout(reg) b);
    asm!("{} {}", out(reg) _, inlateout(reg) b => _);
    asm!("", out("al") _, lateout("rcx") _);
    asm!("beep~", "boop!");
    asm!("beep~ {}, 42", "boop! {}, 24", in(reg) a, out(reg) b);
    asm!("boop! {1}, 24", "beep~ {0}, 42", in(reg) a, out(reg) b);
    asm!("beep~ {}, 42", "boop! {name}, 24", in(reg) a, name = out(reg) b);
    asm!("beep~
boop!");
    asm!("beep~\nboop!");
    asm!("beep~\n\tboop!");
    asm!("beep~\nboop!", "boop3\nboop4");
  }
}
debug!(?value);
debug!(
  "VariantDef::new(name = {:?}, variant_did = {:?}, ctor_def_id = {:?}, discr = {:?},
     fields = {:?}, ctor_kind = {:?}, adt_kind = {:?}, parent_did = {:?})",
  name,
  variant_did,
  ctor_def_id,
  discr,
  fields,
  ctor_kind,
  adt_kind,
  parent_did
);
slice_interners!(
    substs: _intern_substs(GenericArg<'tcx>),
    canonical_var_infos: _intern_canonical_var_infos(CanonicalVarInfo<'tcx>),
    poly_existential_predicates:
        _intern_poly_existential_predicates(ty::Binder<'tcx, ExistentialPredicate<'tcx>>),
    predicates: _intern_predicates(Predicate<'tcx>),
    projs: _intern_projs(ProjectionKind),
    place_elems: _intern_place_elems(PlaceElem<'tcx>),
    bound_variable_kinds: _intern_bound_variable_kinds(ty::BoundVariableKind),
);

impl_binder_encode_decode! {
    &'tcx ty::List<Ty<'tcx>>,
    ty::FnSig<'tcx>,
    ty::ExistentialPredicate<'tcx>,
    ty::TraitRef<'tcx>,
    Vec<ty::GeneratorInteriorTypeCause<'tcx>>,
}
impl_arena_copy_decoder! {<'tcx>
    Span,
    rustc_span::symbol::Ident,
    ty::Variance,
    rustc_span::def_id::DefId,
    rustc_span::def_id::LocalDefId,
    (rustc_middle::middle::exported_symbols::ExportedSymbol<'tcx>, rustc_middle::middle::exported_symbols::SymbolExportInfo),
}

bitflags! {
    #[derive(HashStable, TyEncodable, TyDecodable)]
    pub struct AdtFlags: u32 {
        const NO_ADT_FLAGS        = 0;
        /// Indicates whether the ADT is an enum.
        const IS_ENUM             = 1 << 0;
        /// Indicates whether the ADT is a union.
        const IS_UNION            = 1 << 1;
        /// Indicates whether the ADT is a struct.
        const IS_STRUCT           = 1 << 2;
        /// Indicates whether the ADT is a struct and has a constructor.
        const HAS_CTOR            = 1 << 3;
        /// Indicates whether the type is `PhantomData`.
        const IS_PHANTOM_DATA     = 1 << 4;
        /// Indicates whether the type has a `#[fundamental]` attribute.
        const IS_FUNDAMENTAL      = 1 << 5;
        /// Indicates whether the type is `Box`.
        const IS_BOX              = 1 << 6;
        /// Indicates whether the type is `ManuallyDrop`.
        const IS_MANUALLY_DROP    = 1 << 7;
        /// Indicates whether the variant list of this ADT is `#[non_exhaustive]`.
        /// (i.e., this flag is never set unless this ADT is an enum).
        const IS_VARIANT_LIST_NON_EXHAUSTIVE = 1 << 8;
    }
}
rustc_dep_node_append!([define_dep_nodes!][ <'tcx>
    // We use this for most things when incr. comp. is turned off.
    [] Null,

    [anon] TraitSelect,

    // WARNING: if `Symbol` is changed, make sure you update `make_compile_codegen_unit` below.
    [] CompileCodegenUnit(Symbol),

    // WARNING: if `MonoItem` is changed, make sure you update `make_compile_mono_item` below.
    // Only used by rustc_codegen_cranelift
    [] CompileMonoItem(MonoItem),
]);

decl_derive!([Decodable] => serialize::decodable_derive);

let ret = structure.gen_impl(
  quote! {
    gen impl rustc_errors::AddSubdiagnostic for @Self {
        fn add_to_diagnostic(self, #diag: &mut rustc_errors::Diagnostic) {
            use rustc_errors::{Applicability, IntoDiagnosticArg};
            #implementation
        }
    }
}
);
// source: "../../../ext/jinx-rust/tests/samples/macro/macro.tokens.rs"