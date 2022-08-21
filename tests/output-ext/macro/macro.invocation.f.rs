declare_clippy_lint! {
    /// ### q
    /// ```rust
    /// let b: Vec<&str> = vec![];
    /// if !b.is_empty() {
    ///     panic!(q: {:?}", b);
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// let b: Vec<&str> = vec![];
    /// assert!(b.is_empty(), "there are sad people: {:?}", b);
    /// ```
    #[clippy::version = "1.57.0"]
    pub MANUAL_ASSERT,
    pedantic,
    "`panic!` and only a `panic!` in `if`-then statement"
}

fn aux<Xs: HList, Ys: HList>(xs: Xs, ys: Ys) -> Expr!(Xs + Ys) where Xs: Add<Ys> {
  xs + ys
}

declare_lint_pass!(Attributes => [
    INLINE_ALWAYS,
    DEPRECATED_SEMVER,
    USELESS_ATTRIBUTE,
    BLANKET_CLIPPY_RESTRICTION_LINTS,
]);

fn main() {
  let xs: HList!(&str, bool, Vec<u64>) = hlist!("foo", false, vec![]);
  let ys: HList!(u64, [u8; 3], ()) = hlist!(0, [0, 1, 2], ());
  let zs: Expr!(
    HList![&str] +
      HList![bool] +
      HList![Vec<u64>] +
      (HList![u64] + HList![[u8; 3], ()]) +
      HList![]
  ) = aux(xs, ys);
  higher_order!(subst ($x:expr, $y:expr, $foo:expr) => (($x + $y, $foo)));
  assert_eq!(zs, hlist!["foo", false, vec![], 0, [0, 1, 2], ()]);
  quote!(fn $name() -> bool { true });
  impl_lint_pass!(Arithmetic => [INTEGER_ARITHMETIC, FLOAT_ARITHMETIC]);
  let result =
    quote! {
        macro_rules! generated_foo {
            (1 $$x:expr $$($$y:tt,)* $$(= $$z:tt)*) => {};
        }
    };
  if
    !matches!(
      macro_name.as_str(),
      "assert_eq" | "debug_assert_eq" | "assert_ne" | "debug_assert_ne"
    )
  {
  }
  if
    matches!(cx.tcx.type_of(id).kind(), ty::Adt(adt, _) if ty_adt.did() == adt.did())
  {
  }
  if
    !matches!(
      get_expr_use_or_unification_node(tcx, expr),
      None |
        Some((
          Node::Stmt(Stmt {
            kind: StmtKind::Expr(_) |
            StmtKind::Semi(_) |
            StmtKind::Local(Local {
              pat: Pat {
                kind: PatKind::Wild,
                ..
              },
              ..
            }),
            ..
          }),
          _,
        ))
    )
  {
  }
  if_chain! {
        if let ExprKind::If(cond, then, None) = expr.kind;
        if !matches!(cond.kind, ExprKind::Let(_));
        if !expr.span.from_expansion();
        let then = peel_blocks_with_stmt(then);
        if let Some(macro_call) = root_macro_call(then.span);
        if cx.tcx.item_name(macro_call.def_id) == sym::panic;
        if !cx.tcx.sess.source_map().is_multiline(cond.span);
        if let Some(format_args) = FormatArgsExpn::find_nested(cx, then, macro_call.expn);
        then {
            let mut applicability = Applicability::MachineApplicable;
            let format_args_snip = snippet_with_applicability(cx, format_args.inputs_span(), "..", &mut applicability);
            let cond = cond.peel_drop_temps();
            let (cond, not) = match cond.kind {
                ExprKind::Unary(UnOp::Not, e) => (e, ""),
                _ => (cond, "!"),
            };
            let cond_sugg = sugg::Sugg::hir_with_applicability(cx, cond, "..", &mut applicability).maybe_par();
            let sugg = format!("assert!({not}{cond_sugg}, {format_args_snip});");
            span_lint_and_sugg(
                cx,
                MANUAL_ASSERT,
                expr.span,
                "only a `panic!` in `if`-then statement",
                "try",
                sugg,
                Applicability::MachineApplicable,
            );
        }
    }
  if_chain! {
        if meets_msrv(msrv.as_ref(), &msrvs::TOOL_ATTRIBUTES);
        // check cfg_attr
        if attr.has_name(sym::cfg_attr);
        if let Some(items) = attr.meta_item_list();
        if items.len() == 2;
        // check for `rustfmt`
        if let Some(feature_item) = items[0].meta_item();
        if feature_item.has_name(sym::rustfmt);
        // check for `rustfmt_skip` and `rustfmt::skip`
        if let Some(skip_item) = &items[1].meta_item();
        if skip_item.has_name(sym!(rustfmt_skip)) ||
            skip_item.path.segments.last().expect("empty path in attribute").ident.name == sym::skip;
        // Only lint outer attributes, because custom inner attributes are unstable
        // Tracking issue: https://github.com/rust-lang/rust/issues/54726
        if attr.style == AttrStyle::Outer;
        then {
            span_lint_and_sugg(
                cx,
                DEPRECATED_CFG_ATTR,
                attr.span,
                "`cfg_attr` is deprecated for rustfmt and got replaced by tool attributes",
                "use",
                "#[rustfmt::skip]".to_string(),
                Applicability::MachineApplicable,
            );
        }
    }
  if_chain! {
        if let ExprKind::Binary(ref op, left, right) = &expr.kind;
        if let Some((candidate, check)) = normalize_le_ge(op, left, right);
        if let Some((from, to)) = get_types_from_cast(check, INTS, "max_value", "MAX");

        then {
            Conversion::try_new(candidate, from, to)
        } else {
            None
        }
    }
  let help = format!(
    "because `{}` is the {} value for this type, {}",
    snippet(cx, culprit.expr.span, "x"),
    match culprit.which {
      ExtremeType::Minimum => "minimum",
      ExtremeType::Maximum => "maximum",
    },
    conclusion
  );

  let msg = format!(
    "this `{}` can be collapsed into the outer `{}`",
    if matches!(inner, IfLetOrMatch::Match(..)) {
      "match"
    } else {
      "if let"
    },
    if outer_is_match {
      "match"
    } else {
      "if let"
    }
  );
  let mut contents = format!(
    indoc! {
      "
            #![warn(clippy::{})]

            fn main() {{
                // test code goes here
            }}
        "
    },
    lint_name
  );
  format!(
    "store.register_{lint_pass}_pass(move || Box::new({module_name}::{camel_name}::new(msrv)));\n    ",
    lint_pass = lint.pass,
    module_name = lint.name,
    camel_name = to_camel_case(lint.name)
  );
  format!(
    indoc! {
      r#"
            # {}

            [package]
            name = "{}"
            version = "0.1.0"
            publish = false

            [workspace]
        "#
    },
    hint,
    lint_name
  );
  match iter.next() {
    // #[clippy::version = "version"] pub
    Some((TokenKind::Pound, _)) => {
      match_tokens!(iter, OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket Ident);
    }
    // pub
    Some((TokenKind::Ident, _)) => (),
    _ => {
      continue;
    }
  }
  let (name, group, desc) =
    match_tokens!(
        iter,
        // LINT_NAME
        Ident(name) Comma
        // group,
        Ident(group) Comma
        // "description" }
        Literal{..}(desc) CloseBrace
        // #[clippy::version = "version"]
        Pound OpenBracket Ident Colon Colon Ident Eq Literal{..} CloseBracket
        // pub LINT_NAME,
        Ident Ident(name) Comma
        // "description"
        Literal{kind: LiteralKind::Str{..},..}(reason)
        // ("old_name",
        Whitespace OpenParen Literal{kind: LiteralKind::Str{..},..}(old_name) Comma
        // "new_name"),
        Whitespace Literal{kind: LiteralKind::Str{..},..}(new_name) CloseParen Comma
    );

  info!(
    //debug
    "{}: sending function_code={:04x} data={:04x} crc=0x{:04X} data={:02X?}",
    self.name,
    function_code,
    data,
    crc,
    output_cmd
  );
}

cfg_if! {
    if #[cfg(feature = "std_detect_file_io")] {
        #[cfg_attr(test, macro_use(println))]
        extern crate std;

        #[allow(unused_imports)]
        use std::{arch, fs, io, mem, sync};
    } else {
        #[cfg(test)]
        #[macro_use(println)]
        extern crate std;

        #[allow(unused_imports)]
        use core::{arch, mem, sync};
    }
}

op_utils! {
  Add;
  AddAssign;
  Sub;
  SubAssign;
  Mul;
  MulAssign;
  Div;
  DivAssign;
  Rem;
  RemAssign;
  BitXor;
  BitXorAssign;
  BitAnd;
  BitAndAssign;
  BitOr;
  BitOrAssign;
  Shl;
  ShlAssign;
  Shr;
  ShrAssign;
}

msrv_aliases! {
    1,53,0 { OR_PATTERNS, MANUAL_BITS }
    1,52,0 { STR_SPLIT_ONCE }
    1,51,0 { BORROW_AS_PTR, UNSIGNED_ABS }
    1,50,0 { BOOL_THEN }
    1,47,0 { TAU }
    1,46,0 { CONST_IF_MATCH }
    1,45,0 { STR_STRIP_PREFIX }
    1,43,0 { LOG2_10, LOG10_2 }
    1,42,0 { MATCHES_MACRO, SLICE_PATTERNS, PTR_SLICE_RAW_PARTS }
    1,41,0 { RE_REBALANCING_COHERENCE, RESULT_MAP_OR_ELSE }
    1,40,0 { MEM_TAKE, NON_EXHAUSTIVE, OPTION_AS_DEREF }
    1,38,0 { POINTER_CAST }
    1,37,0 { TYPE_ALIAS_ENUM_VARIANTS }
    1,36,0 { ITERATOR_COPIED }
    1,35,0 { OPTION_COPIED, RANGE_CONTAINS }
    1,34,0 { TRY_FROM }
    1,30,0 { ITERATOR_FIND_MAP, TOOL_ATTRIBUTES }
    1,28,0 { FROM_BOOL }
    1,17,0 { FIELD_INIT_SHORTHAND, STATIC_IN_CONST, EXPECT_ERR }
    1,16,0 { STR_REPEAT }
    1,24,0 { IS_ASCII_DIGIT }
}

kot! {
  struct W {
    foo: u8,
    bar: u16,
  }
  impl Clone for W {
    fn clone() -> W {
      panic!();
    }
  }
}

kot! {
    a(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins) a
    [mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins] a
    {
        mushkins mushkins mushkins mushkins mushkins mushkins mushkins
        mushkins mushkins mushkins
    } a
}

kot!(mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
mushkins mushkins);
kot![mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
mushkins mushkins];
kot! {
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
  mushkins;
}

#[rustc_dummy(mushkins mushkins mushkins mushkins mushkins mushkins mushkins
mushkins mushkins mushkins)]
#[rustc_dummy[mushkins mushkins mushkins mushkins mushkins mushkins mushkins
mushkins mushkins mushkins]]
#[rustc_dummy {
    mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins
    mushkins mushkins
}]
#[rustc_dummy = "mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins mushkins"]
match t.kind() {
  ty::Int(i) =>
    find_fit!(i, val, negative,
                  I8 => [U8] => [I16, I32, I64, I128],
                  I16 => [U16] => [I32, I64, I128],
                  I32 => [U32] => [I64, I128],
                  I64 => [U64] => [I128],
                  I128 => [U128] => []),
  ty::Uint(u) =>
    find_fit!(u, val, negative,
                  U8 => [U8, U16, U32, U64, U128] => [],
                  U16 => [U16, U32, U64, U128] => [],
                  U32 => [U32, U64, U128] => [],
                  U64 => [U64, U128] => [],
                  U128 => [U128] => []),
  _ => None,
}
x! {
    /// [d]: ../b.md#a
    pub G,
    d,
    "c",
    @d = x {
        b: "a",
    };
}
throw_span_err!(
  attr.span().unwrap(),
  &format!(
    "the `#[{}{}]` attribute can only be applied to fields of type {}",
    name,
    match meta {
      Meta::Path(_) => "",
      Meta::NameValue(_) => " = ...",
      Meta::List(_) => "(...)",
    },
    ty_name
  )
);

provide! { 
    <'tcx> tcx, def_id, other, cdata,
    explicit_item_bounds => { table }
    explicit_predicates_of => { table }
    adt_destructor => {
        let _ = cdata;
        tcx.calculate_dtor(def_id, |_,_| Ok(()))
    }
    extern_crate => {
        let r = *cdata.extern_crate.lock();
        r.map(|c| &*tcx.arena.alloc(c))
    }
    reachable_non_generics => {
        let reachable_non_generics = tcx
            .exported_symbols(cdata.cnum)
            .iter()
            .filter_map(|&(exported_symbol, export_info)| {
                if let ExportedSymbol::NonGeneric(def_id) = exported_symbol {
                    Some((def_id, export_info))
                } else {
                    None
                }
            })
            .collect();

        reachable_non_generics
    }
    dep_kind => {
        let r = *cdata.dep_kind.lock();
        r
    }
    module_children => {
        let mut result = SmallVec::<[_; 8]>::new();
        cdata.for_each_module_child(def_id.index, |child| result.push(child), tcx.sess);
        tcx.arena.alloc_slice(&result)
    }

    missing_extern_crate_item => {
        let r = matches!(*cdata.extern_crate.borrow(), Some(extern_crate) if !extern_crate.is_direct());
        r
    }

    used_crate_source => { Lrc::clone(&cdata.source) }
    debugger_visualizers => { cdata.get_debugger_visualizers() }

    exported_symbols => {
        let syms = cdata.exported_symbols(tcx);

        // a

        syms
    }
}

cfg_if::cfg_if! {
    if #[cfg(unix)] {
    } else if #[cfg(any(target_os = "hermit",
                        target_os = "solid_asp3",
                        all(target_vendor = "fortanix", target_env = "sgx")
    ))] {
    } else if #[cfg(all(windows, not(miri)))] {
    }
}

ast_fragments! {
    Expr(P<ast::Expr>) { "expression"; one fn visit_expr; fn visit_expr; fn make_expr; }
    Pat(P<ast::Pat>) { "pattern"; one fn visit_pat; fn visit_pat; fn make_pat; }
    Ty(P<ast::Ty>) { "type"; one fn visit_ty; fn visit_ty; fn make_ty; }
    Stmts(SmallVec<[ast::Stmt; 1]>) {
        "statement"; many fn flat_map_stmt; fn visit_stmt(); fn make_stmts;
    }
    Items(SmallVec<[P<ast::Item>; 1]>) {
        "item"; many fn flat_map_item; fn visit_item(); fn make_items;
    }
    TraitItems(SmallVec<[P<ast::AssocItem>; 1]>) {
        "trait item";
        many fn flat_map_trait_item;
        fn visit_assoc_item(AssocCtxt::Trait);
        fn make_trait_items;
    }
    ImplItems(SmallVec<[P<ast::AssocItem>; 1]>) {
        "impl item";
        many fn flat_map_impl_item;
        fn visit_assoc_item(AssocCtxt::Impl);
        fn make_impl_items;
    }
    ForeignItems(SmallVec<[P<ast::ForeignItem>; 1]>) {
        "foreign item";
        many fn flat_map_foreign_item;
        fn visit_foreign_item();
        fn make_foreign_items;
    }
    Arms(SmallVec<[ast::Arm; 1]>) {
        "match arm"; many fn flat_map_arm; fn visit_arm(); fn make_arms;
    }
    ExprFields(SmallVec<[ast::ExprField; 1]>) {
        "field expression"; many fn flat_map_expr_field; fn visit_expr_field(); fn make_expr_fields;
    }
    PatFields(SmallVec<[ast::PatField; 1]>) {
        "field pattern";
        many fn flat_map_pat_field;
        fn visit_pat_field();
        fn make_pat_fields;
    }
    GenericParams(SmallVec<[ast::GenericParam; 1]>) {
        "generic parameter";
        many fn flat_map_generic_param;
        fn visit_generic_param();
        fn make_generic_params;
    }
    Params(SmallVec<[ast::Param; 1]>) {
        "function parameter"; many fn flat_map_param; fn visit_param(); fn make_params;
    }
    FieldDefs(SmallVec<[ast::FieldDef; 1]>) {
        "field";
        many fn flat_map_field_def;
        fn visit_field_def();
        fn make_field_defs;
    }
    Variants(SmallVec<[ast::Variant; 1]>) {
        "variant"; many fn flat_map_variant; fn visit_variant(); fn make_variants;
    }
    Crate(ast::Crate) { "crate"; one fn visit_crate; fn visit_crate; fn make_crate; }
}

sides_mapping_methods! {
    MainStartCrossStart::InlineStartBlockStart => {
        main_start <=> inline_start,
        cross_start <=> block_start,
        main_end <=> inline_end,
        cross_end <=> block_end,
    }
}

fuzz_target!(|data: &[u8]| (
  if let Some(first) = data.first() {
    let index = *first as usize;
    if index >= ENCODINGS.len() {
      return;
    }
    let encoding = ENCODINGS[index];
    dispatch_test(encoding, &data[1..]);
  }
));

assert_eq!(count_compound_ops!(foo<=>bar <<<! -baz ++), 4);

print_bang! {
  /**
   *******
   * DOC *
   * DOC *
   * DOC *
   *******
   */
  pub struct S;
}

c! {
  pub const A: i32 = 0;
}
c! {
  pub enum B {}
}
c! {
  pub extern "C" fn c() {}
}
c! {
  pub mod d {}
}
c! {
  pub static E: i32 = 0;
}
c! {
  pub struct F;
}
c! {
  pub trait G {}
}
c! {
  pub type H = i32;
}
c! {
  pub use A as I;
}
c! {
  const A: i32 = 0;
}
c! {
  enum B {}
}
c! {
  extern "C" fn c() {}
}
c! {
  mod d {}
}
c! {
  static E: i32 = 0;
}
c! {
  struct F;
}
c! {
  trait G {}
}
c! {
  type H = i32;
}
c! {
  use A as I;
}
c! {
  pub(crate) const A: i32 = 0;
}
c! {
  pub(crate) enum B {}
}
c! {
  pub(crate) extern "C" fn c() {}
}
c! {
  pub(crate) mod d {}
}
c! {
  pub(crate) static E: i32 = 0;
}
c! {
  pub(crate) struct F;
}
c! {
  pub(crate) trait G {}
}
c! {
  pub(crate) type H = i32;
}
c! {
  pub(crate) use A as I;
}
c! {
  crate const A: i32 = 0;
}
c! {
  crate enum B {}
}
c! {
  crate extern "C" fn c() {}
}
c! {
  crate mod d {}
}
c! {
  crate static E: i32 = 0;
}
c! {
  crate struct F;
}
c! {
  crate trait G {}
}
c! {
  crate type H = i32;
}
c! {
  crate use A as I;
}
c! {
  pub(in foo) const A: i32 = 0;
}
c! {
  pub(in foo) enum B {}
}
c! {
  pub(in foo) extern "C" fn c() {}
}
c! {
  pub(in foo) mod d {}
}
c! {
  pub(in foo) static E: i32 = 0;
}
c! {
  pub(in foo) struct F;
}
c! {
  pub(in foo) trait G {}
}
c! {
  pub(in foo) type H = i32;
}
c! {
  pub(in foo) use A as I;
}
c! {
  pub(crate) struct A {
    pub a: i32,
    b: i32,
    pub(crate) c: i32,
  }
}
c! {
  pub struct B {
    a: i32,
    pub(crate) b: i32,
    pub c: i32,
  }
}
c! {
  struct C {
    pub(crate) a: i32,
    pub b: i32,
    c: i32,
  }
}

c! {
  pub(crate) struct D(pub i32, i32, pub(crate) i32);
}
c! {
  pub struct E(i32, pub(crate) i32, pub i32);
}
c! {
  struct F(pub(crate) i32, pub i32, i32);
}

c!(pub(crate);
c!(pub(self)));
c!(pub(super));
c!(pub(in self));
c!(pub(in super));
c!(pub(in path::to));
c!(pub(in ::path::to));
c!(pub(in self::path::to));
c!(pub(in super::path::to));
// source: "../../../ext/jinx-rust/tests/samples/macro/macro.invocation.rs"