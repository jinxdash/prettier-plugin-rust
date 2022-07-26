//! Expect 1 empty line below

//! Expect 0 empty line below
//! Expect 1 empty line below

1;
//! Expect 1 empty line below

//! Expect 0 empty line below
1;

{
	#![attr] // comment after #![attr]
}

match () {
	#![attr] // comment after #![attr]
}

impl Foo<[u8; {
    #![cfg_attr(not(FALSE), rustc_dummy(cursed_inner))]
    #![allow(unused)]
    struct Inner {
        field: [u8; {
            #![cfg_attr(not(FALSE), rustc_dummy(another_cursed_inner))]
            1
        }]
    }

    0
}]> {
    #![cfg_eval]
    #![print_attr]
    #![cfg_attr(not(FALSE), rustc_dummy(evaluated_attr))]

    fn bar() {
        #[cfg(FALSE)] let a = 1;
    }
}

#[cfg_eval]
#[print_attr]
struct S1 {
    #[cfg(FALSE)]
    field_false: u8,
    #[cfg(all(/*true*/))]
    #[cfg_attr(FALSE, unknown_attr)]
    #[cfg_attr(all(/*true*/), allow())]
    field_true: u8,
}

fn main() {
    // 
    // 
    let _ = #[cfg_eval] #[print_attr] #[cfg_attr(not(FALSE), rustc_dummy)]
    (#[cfg(FALSE)] 0, #[cfg(all(/*true*/))] 1,);
}

#[empty_helper] // a
                // b
                // c
#[derive(Empty)]
struct S {
    #[empty_helper] // d
    field: [u8; {
        use empty_helper; // e

        #[empty_helper] // f
        struct U;

        mod inner {
            // g
            #[empty_helper]
            struct V;

            gen_helper_use!();

            #[derive(GenHelperUse)] // h
            struct Owo;

            use empty_helper as renamed;
            #[renamed] // i
            struct Wow;
        }

        0
    }]
}

fn f() {
  return (
    property.isIdentifier() &&
     FUNCTIONS[property.node.name] &&
     (object.isIdentifier(JEST_GLOBAL) ||
       (callee.isMemberExpression() && shouldHoistExpression(object))) &&
    FUNCTIONS[property.node.name](expr.get("arguments"))
  );

  return (
    chalk.bold(
      "No tests found related to files changed since last commit.\n",
    ) +
    chalk.dim(
      if patternInfo.watch 
        {r#"Press "a" to run all tests, or run Jest with "--watchAll"."#}
        else {r#"Run Jest without "-o" to run all tests."#},
    )
  );

  return !filePath.includes(coverageDirectory) &&
    !filePath.endsWith(".${SNAPSHOT_EXTENSION}");
	
  return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD;

}

fn f() {
  if (position)
    {return A {name: pair};}
  else
    {return A {name: pair.substring(0, position), value: pair.substring(position + 1)};}
}

fn f() {
  if (position)
    {return A {name: pair};}
  else
    {return A {
      name: pair.substring(0, position),
      value: pair.substring(position + 1)
    };}
}

fn f() {
  if let Some(_) = try {} {
  }
}

pub fn delete_upload_session(&self, sess: &UploadSession) -> Result<()> {
    self.client
        .delete(&sess.upload_url)
        .send()?
        .parse_empty()
}

let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

fn very_very_very_very_very_long_fun_name(x: i32) -> Vec<i32> {
	vec![x]
}

fn main() {
	let very_very_very_very_very_very_very_very_very_long_var_name = 13;
	let all = very_very_very_very_very_long_fun_name(
		very_very_very_very_very_very_very_very_very_long_var_name,
	)
	.iter()
	.map(|x| x + very_very_very_very_very_very_long_var_name);
	let more = 13;
	let other = vec![1, 2, 3]
		.iter()
		.map(|x| x + very_very_very_very_very_very_long_var_name);

    Pin::new(&mut self.0).poll(cx).is_ready();
}

fn f() {
    something.do_stuff(|| {
        {
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
        }
    });
}
let kind = match rvalue {
	Rvalue::Ref(_, borrow_kind, _)
		if borrow_kind.allows_two_phase_borrow() =>
	{
		RetagKind::TwoPhase
	}
	Rvalue::AddressOf(..) => RetagKind::Raw,
	_ => RetagKind::Default,
};

use exonum::{
    api::{Api, ApiError},
    blockchain::{self, BlockProof, Blockchain, Transaction, TransactionSet},
    crypto::{Hash, PublicKey},
    helpers::Height,
    node::TransactionSend,
    storage::{ListProof, MapProof},
};

let mut arms = variants.iter().enumerate().map(|(i, &(ident, v_span, ref summary))| {
	let i_expr = cx.expr_usize(v_span, i);
	let pat = cx.pat_lit(v_span, i_expr);

	let path = cx.path(v_span, vec![substr.type_ident, ident]);
	let thing = rand_thing(cx, v_span, path, summary, |cx, sp| rand_call(cx, sp));
	cx.arm(v_span, vec!( pat ), thing)
}).collect::<Vec<ast::Arm> >();

let input = Input {
    foo: 42,
};
some_fn_with_struct_and_closure(input, |foo| {
    println!("foo: {:?}", foo);
});
some_fn_with_struct_and_closure(Input {
    foo: 42,
}, |foo| {
    println!("foo: {:?}", foo);
});
fn main() {
    assert_eq!(code, unindent(r#"
        def hello():
            print("Hello, world!")
        
        hello()
    "#));
    assert_eq(code, unindent(r#"
        def hello():
            print("Hello, world!")
        
        hello()
    "#));
	assert_eq!(s, wrap(A {
        x: 10,
        y: 20,
        z: 30,
    }));
    assert_eq(s, wrap(A {
        x: 10,
        y: 20,
        z: 30,
    }));
    assert_eq!(s, wrap(A { x: 10, y: 20, z: 30, }));
    assert_eq(s, wrap(A { x: 10, y: 20, z: 30, }));
}

fn speak_raw(
	self,
	seed: u32,
	language: Language<B, P>,
	speaker: Speaker<N, M>,
) -> Synthesize<Jitter<Sequence<Select<Intonate<Phonemize<core::iter::Map<Self::IntoIter, fn(char) -> Event<char>>,B,N,M,>,P,>,N,M,>,N,M,>,N,M,>,N,M,> {
	self.parse_raw()
		.phonemize(&language, &speaker)
		.intonate(&language, &speaker)
		.select(&speaker)
		.sequence(speaker.sample_rate)
		.jitter(seed, &speaker)
		.synthesize()
}

use {
    fidl::endpoints::RequestStream,
    fuchsia_async as fasync,
    fuchsia_zircon as zx,
    std::marker::PhantomData,
};

fn f() -> Vec<u32> {
    [1, 2, 3]
        .iter()
        .map(|&x| {
            return x * 2;
        })
        .collect()
}

Client::new()
    .request(
        Request::get(&req.state().upstream)
            .header(
                header::ACCEPT,
                HeaderValue::from_static(CONTENT_TYPE_GRAPH_V1),
            )
            .body(Body::empty())
            .expect("unable to form request"),
    )
    .from_err::<Error>()
    .and_then(|res| {
        if res.status().is_success() {
            future::ok(res)
        } else {
            future::err(format_err!(
                "failed to fetch upstream graph: {}",
                res.status()
            ))
        }
    })
    .and_then(|res| res.into_body().concat2().from_err::<Error>())
    .and_then(|body| {
        let graph: Graph = serde_json::from_slice(&body)?;
        Ok(HttpResponse::Ok()
            .content_type(CONTENT_TYPE_GRAPH_V1)
            .body(serde_json::to_string(&graph)?))
    })

fn foo() {
    parse_simple_ok("() ()", vec![
		SimpleSexpr::List { opening: "(".into(), closing: ")".into(), entire: "()".into(), children: vec![], }, 
		SimpleSexpr::List { opening: "(".into(), closing: ")".into(), entire: "()".into(), children: vec![], }]
	);
}

impl X {
    pub const SOMETHING: usize = mem::size_of::<i64>() // field A
        + mem::size_of::<i64>() // field B
        + mem::size_of::<i64>() // field C
+ mem::size_of::<i64>() // field D
        + mem::size_of::<i64>() // field E
        + mem::size_of::<i64>() // field F
        + mem::size_of::<i64>() // field G
        + mem::size_of::<i64>(); // field H
}

pub type Iter<'a, D> = impl DoubleEndedIterator<Item = (SomethingSomethingSomethingLongType<D>)> + ExactSizeIterator + 'a;

pub type Iter<'a, D>: BoundDoubleEndedIterator<Item = (SomethingSomethingSomethingLongType)> + ExactSizeIterator + 'a;

pub type Iter<'a, D>: BoundDoubleEndedIterator<Item = (SomethingSomethingSomethingLongType<D>)> + ExactSizeIterator + 'a;

let a = Some(2).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).map(|v| v).unwrap(/* fine because we know it's a Some */);

use module::{
  submodule_A::{Type_A1, Type_A2},
  submodule_B::{Type_B1, Type_B2},
};

fn f1() -> Box<
    FnMut1() -> Thing1<
        WithType = LongItemName,
        Error = LONGLONGLONGLONGLONGONGEvenLongerErrorNameLongerLonger,
    >,
> {
}
fn f2() -> Box<
    FnMu2t() -> Thing2<
            WithType = LongItemName,
            Error = LONGLONGLONGLONGLONGONGEvenLongerErrorNameLongerLonger,
        > + fmt::Write,
> {
}

flags! {
    enum Permissions: c_int {
R = 1,
W = 2,
X = 16,
}
}

impl CalibrationType {
    #[rustfmt::skip] // In this case aligning the match branches makes for more readable code.
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "alpha-beta" => Some(Self::AlphaBeta),
            "bin-shift"  => Some(Self::BinShift),
            "enum"       => Some(Self::Enum),
            "none"       => Some(Self::None),
            _            => None
        }
    }
    
    #[rustfmt::skip] // In this case aligning the match branches makes for more readable code.
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::AlphaBeta => "alpha-beta",
            Self::BinShift  => "bin-shift",
            Self::Enum      => "enum",
            Self::None      => "none"
        }
    }
}

pub trait PrettyPrinter<'tcx>:
    Printer<
        'tcx,
        Error = fmt::Error,
        Path = Self,
        Region = Self,
        Type = Self,
        DynExistential = Self,
        Const = Self,
    >{
         //
}

pub trait PrettyPrinter<'tcx>:
    Printer<
        'tcx,
        Error = fmt::Error,
        Path = Self,
        Region = Self,
        Type = Self,
        DynExistential = Self,
        Const = Self,
    > + Send {
         //
}

Arc::get_mut(&mut runtest).unwrap().get_mut().unwrap().take().unwrap()();

struct X<'a>(
    #[X(X = "_________________________________________________________________________")]
    pub  &'a u32,
    // ^^
);

enum Foo {
    Bar,
    Baz = /* Block comment */ 123,
    Quux = // Line comment
	124,
}

#![feature(trait_alias)]
trait Foo =/*comment*/std::fmt::Debug + Send;
trait Bar =/*comment*/Foo + Sync;

type Kilometers =/*comment*/i32;

mod tests {
    fn test_datetime() {
        for &(year, month, day, hour, min, sec, micro, is_leap) in &[
            (2021, 1, 20, 22, 39, 46, 186605, false), // time of writing :)
            (2020, 2, 29, 0, 0, 0, 0, false), // leap day hehe
            (2016, 12, 31, 23, 59, 59, 123456, false), // latest leap second
            (2016, 12, 31, 23, 59, 59, 123456, true), // latest leap second
            (1156, 3, 31, 11, 22, 33, 445566, false), // long ago
            (1, 1, 1, 0, 0, 0, 0, false), // Jan 01, 1 AD - can't go further than this
            (3000, 6, 5, 4, 3, 2, 1, false), // the future
            (9999, 12, 31, 23, 59, 59, 999999, false), // Dec 31, 9999 AD - can't go further than this
        ] {}
    }
}

#[cfg(windows)]
use glium::glutin::platform::windows::EventLoopExtWindows;
#[cfg(windows)]
use glium::glutin::{platform::windows::IconExtWindows, window::Icon};

fn main() {
    println!("");
    println!("" /* " */);
    println!("" /* \" */);
}

struct Bar(());
struct Foo(Bar);

fn main() {
    let foo = Foo(Bar(()));

    foo.0.0;
}

tokio::select! {
	result = reader => {
            match result {
		Ok(v) => {
		    eprintln!(
                    "message: {}",
                    v
                    );
                },
		Err(_) => {
		    eprintln!(
                        "error: {}",
                        e
                    );
                },
            }
	},
	_ = writer => {
            // Comment
            eprintln!(
                "completed: {}",
                some_other_field
            );
	}
}

//! Some docs here
#![cfg_attr(bootstrap, doc = "xxx")]

#![cfg_attr(debug_assertions, stable(feature = "rust1", since = "1.0.0"))]

fn main() {
    let condition_a = true;
    let condition_b = false;
    let x = 123.456789
        + if condition_a {
            x + y + z + w
        } else {
            123.456789
        }
        + if condition_b {
            x - y - z - w
        } else {
            123.456789
        };
}

#[cfg(not(miri))] // Miri does not deduplicate consts

fn test() {
    self.1 .0;
}

fn main() {
    let x = 111;	/* First Comment line 1
					 * First Comment line 2 */

    let x = 222;   /* Second Comment line 1
					* Second Comment line 2 */
}

fn main() {
    if /*w*/ 5 /*x*/ == /*y*/ 6 /*z*/ {}
}

fn printsomething() {
    println!(
"line__1
line______2
line________3
line___________4"
            );
}

const USAGE: &'static str = "
toyunda-player.

Usage:
  toyunda-player [options] <file>
  toyunda-player -h | --help
  toyunda-player --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --invert      Invert the screen.
";

fn main() {
    x.f("
Article(id, title).
Vote(user, id).
VC(id, votes) = votes:COUNT(u, Vote(u, id)).
AwV(id, title, votes) = Article(id, title) * VC(id, votes).
Vote_n(user, id, strength=1) <- Vote(user, id).
VS(id, score) = score:SUM(strength, Vote_n+(_, id, strength)).
AwV_n(id, title, score) = Article(id, title) * VS(id, score).
").unwrap();
}

const USAGE: &'static str = " 
...
";

let program = glium::Program::from_source(&display, &include_str!("./shaders/vertex.glsl"), &include_str!("./shaders/fragment.glsl"), None).unwrap();

let mut g_buffer = MultiOutputFrameBuffer::with_depth_buffer(api.facade, [("color", &g_albedo)].iter().cloned(), &g_depth);

macro_rules! foo {
    ($a:ident : $b:ty) => {};
    ($a:ident $b:ident $c:ident) => {};
}

self.0.take_action(Log).result().map_err(|()| LoggerError);

let n: i32 = std::env::args().nth(1).map(parse).unwrap_or(Ok(100))?;

fn main() {
    return doughnutFryer
        .start()
        .then(|_| _frostingGlazer.start())
        .then(|_| Future.wait([
              _conveyorBelts.start(),
              sprinkleSprinkler.start(),
              sauceDripper.start()
            ]))
        .catchError(cannotGetConveyorBeltRunning)
        .then(|_| tellEveryoneDonutsAreJustAboutDone())
        .then(|_| Future.wait([
              croissantFactory.start(),
              _giantBakingOvens.start(),
              butterbutterer.start()
            ])
                .catchError(_handleBakingFailures)
                .timeout(scriptLoadingTimeout, _handleBakingFailures)
                .catchError(cannotGetConveyorBeltRunning))
        .catchError(cannotGetConveyorBeltRunning)
        .then(|_| {
      _logger.info("Let's eat!");
    });
}
self.projection_matrix = Matrix4::new(
	1.0/r, 0.0,   0.0,         0.0,   // NOTE: first column!
	0.0,   1.0/t, 0.0,         0.0,   // 2nd
	0.0,   0.0,   2.0/(n-f),   0.0,   // 3rd
	0.0,   0.0,   (f+n)/(n-f), 1.0    // 4th
);

let explicit_conversion_preserves_semantics = 
	|| !is_mod || (is_mod && attrs.map_or(true, |a| a.is_empty())); 


fn default_user_agent_string(agent: UserAgent) -> String {
    match agent {
        UserAgent::Desktop => {
            DESKTOP_UA_STRING
        }
        UserAgent::Android => {
            "Mozilla/5.0 (Android; Mobile; rv:37.0) Servo/1.0 Firefox/37.0"
        }
    }.to_owned()
}

#![allow(
    clippy::needless_pass_by_value,
    clippy::new_ret_no_self,
    clippy::new_without_default_derive,
)]
copysign(0.5 * P2_HI - (2.0 * s * r_ - (P2_LO - 2.0 * c) - (0.5 * P2_HI - 2.0 * f)), i);

impl Foo {
    fn foo(&self) -> Box<FnMut(i64) -> i64> {
        Box::new(move |aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa| {
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
        })
    }
}

m.map_data(|mut rs| {
	rs.retain(|r| {
		r.iter().enumerate().all(|(i, v)| {
			if let Some(ref rv) = on[i] {
				rv == v
			} else {
				true
			}
		})
	});
	rs.retain(|r| {
		r.iter().enumerate().all(|(i, v)| 
			if let Some(ref rv) = on[i] {
				rv == v
			} else {
				true
			}
		)
	});
});

pub fn uumain(args: Vec<String>) -> i32 {
    let matches = App::new(executable!())
        .arg(            Arg::with_name(OPT_CHANGE)
            .short("c").long("ch")
)
        .arg(
            Arg::with_name(OPT_DEREFERENCE).help("aazezae affect the referent of each symbolic link this is qthe default the symbolic link itself"));
}

impl S {
    fn f() {
        self.g(
            {
                if b {
                    Err(w(
                        i(
                            "expected float or integer types for both operands of {}, got '{}' and '{}'",
                            token,
                        )), left)
                }
                match d {
                    _ => h(m,b), 
                }
            },
        )
    }
}

match () {
	(AngleBracketedArg::Arg(_), None) | (AngleBracketedArg::Constraint(_), Some(_)) => {
	}
}

macro_rules! map_and_then_print {
    ($value:expr, |$pat:pat| $map:expr) => {{
        let $pat = $value;
        let s = $map;
        println!("{}", s);
    }};
}

map_and_then_print!(1, |x| x + 3);
// Prints "4"

match head.packet_type()? {
    p
    @
    (PacketType::Connect
    | PacketType::ConnAck
    | PacketType::SubAck
    | PacketType::UnsubAck) => return Err(Error::WrongPacket(p)),
}

#![feature(raw)]
#![panic_runtime]
#![feature(panic_runtime)]

// `real_imp` is unused with Miri, so silence warnings.
#![cfg_attr(miri, allow(dead_code))]

fn foo() -> () where {}

for i in 0..n + 1 {}

if alpha
	> x_m * (f1 / x1).ln()
		+ (n - (m as f64) + 0.5) * (z / w).ln()
		+ ((y - m) as f64) * (w * p / (x1 * q)).ln()
		// ________
		+ stirling(f1)
		+ stirling(z)
		- stirling(x1)
		- stirling(w)
	{
	continue;
}

match (self, other) {
	(&U32(ref v1), &U32(ref v2)) => v1 == v2,
	(&USize(ref v1), &USize(ref v2)) => v1 == v2,
	(&U32(ref v1), &USize(ref v2)) => {
		v1.len() == v2.len()
		&& v1.iter().zip(v2.iter()).all(|(x, y)| *x as usize == *y)
	}
	(&USize(ref v1), &U32(ref v2)) => {
		v1.len() == v2.len()
		&& v1.iter().zip(v2.iter()).all(|(x, y)| *x == *y as usize)
	}
}

fn main() {
    let o_num = Some(123);
    
    let x = if let Some(n) =
        // ________
        o_num {
            n * 2
        } else {
            0
        };
    
    println!("Number: {}", x);
}

struct Foo {
    // a: i32,
    //
    // b: i32,
}

struct Foo {
    a: i32,
    //
    // b: i32,
}

macro_rules! m {
	($a:expr) => {
		if $a {
			return;
			}
	};
}

let write_status = |
	status: &mut Vec<ansi_term::ANSIString>,
	diff: &Diff,
	heading: &str,
	color: &Style,
	show_hints: bool,
	hints: &[&str]
	| -> Result<bool> {}

macro_rules! member_mut {
	($self:expr, $member:expr) => {{
		use self::Member::*;
		let r = &mut *$self;
		match $member {
			A => &mut r.a,
			B => &mut r.b,
			}
		}};
}
if let Some(ref /*mut*/ state) = foo {
}

impl<
	Target: FromEvent<A> + FromEvent<B>,
	A: Widget2<Ctx = C>,
	B: Widget2<Ctx = C>,
	C: for<'a> CtxFamily<'a>,
> Widget2 for WidgetEventLifter<Target, A, B> {
	type Ctx = C;
	type Event = Vec<Target>;
}

async {
    // Do
    // some
    // work
}.await;

fn main() {
    token!(dyn);
}

fn build_sorted_static_get_entry_names(
    mut entries: Vec<(u8, &'static str)>,
) -> (impl Fn(
    AlphabeticalTraversal,
    Box<dyn dirents_sink::Sink<AlphabeticalTraversal>>,
) -> BoxFuture<'static, Result<Box<dyn dirents_sink::Sealed>, Status>>
          + Send
          + Sync
          + 'static) {
}

fn main() {
    bbbbbbbbbbbbbbbbbb::ccccccccccccccccccccccccccccccccccccccc::dddddddddddddddddddd(
        eeeeeeeeeeeeeeeeeeee::ffffffffffffffffffff(
            ggggggggg(hhhhhhhhhhhhhhhh),
            iiiiiiiii(jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj),
        ),
    );
}

fn f() -> Box<
    dyn FnMut() -> Thing< WithType = LongItemName, Error = LONGLONGLONGLONGLONGONGEvenLongerErrorNameLongerLonger>,
>{
}

trait Foo where {}
struct Bar where {}
impl<> Foo<> for Bar<> where {}

impl Foo {
    fn foo() {
        self.report.add_non_formatted_ranges(visitor.skipped_range.clone());
    }
}

fn test() {
    let aaaaaaaaaaaaaaaa = ffffffffffffffffffffffff
        .iter()
        .filter_map(|_| {
            if bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
                == ccccccccccccccccccccccc
                    .dddddddddddddddddddddddd()
                    .eeeeeeeeeeeeeeeeeeeeee()
            {
                ();
            }
        })
        .collect();
}

fn main() {
    let visual_studio_path = {
        let vswhere_stdout = String::from_utf8(vswhere_output.stdout)
    .expect("vswhere output is not valid UTF-8");
        String::from(vswhere_stdout.trim())
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn handles_mid_demangling() {
        assert_eq!(
            crate::demangle_line("        lea     rax, [rip + _ZN55_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Display$GT$3fmt17h510ed05e72307174E]"),
                "        lea     rax, [rip + <&\'a T as core::fmt::Display>::fmt]");
    }
}

fn main() {
    let i = test::<-1>();
    println!("Hello, {}!", i);
}

fn test<const T: i8>() -> i8 {
    {T}
}

vm.get_method_or_type_error(
    obj.clone(),
    "__getitem__",
    || format!("'{}' object is not iterable", obj.class().name)
)?;

type ProposeTransactionsFuture:
	Future<
		Item = ProposeTransactionsResponse<Self::MessageId>,
		Error = Error,
	>;

type ProposeTransactionsFuture:
	Future<
		Item = ProposeTransactionsResponse<Self::MessageId>,
		Error = Error,
	>
	+ Send
	+ 'static;

fn main() {
	/* Common case: The double precision result is fine. */
	if (ui & 0x1fffffff) != 0x10000000  /* not a halfway case */
		|| e == 0x7ff /* NaN */
		|| (result - xy == z as f64 && result - z as f64 == xy) /* exact */
		|| fegetround() != FE_TONEAREST /* not round-to-nearest */
	{
	}
}

pub fn foo(config: &Config) {
    let csv = RefCell::new(create_csv(config, "foo"));
    {
        let mut csv = csv.borrow_mut();
        for (i1, i2, i3) in iproduct!(0..2, 0..3, 0..3) {
            csv.write_field(format!("γ[{}.{}.{}]", i1, i2, i3))
                .unwrap();
            csv.write_field(format!("d[{}.{}.{}]", i1, i2, i3)).unwrap();
            csv.write_field(format!("i[{}.{}.{}]", i1, i2, i3)).unwrap();
        }
        csv.write_record(None::<&[u8]>).unwrap();
    }
}

fn main() {
    let a = Foo {
        something: Some(1),
        bar: 2,
    };
    if let Foo { something: Some(something), .. } = a {
        println!("{}", something);
    }
}

fn main() {
    let dv = (2. * m * l * dtheta * dtheta * theta.sin()
        + 3. * m * g * theta.sin() * theta.cos()
        + 4. * u
        - 4. * b * v)
        / (4. * (M + m) - 3. * m * theta.cos().powi(2));
    let ddtheta = (-3. * m * l * dtheta * dtheta * theta.sin() * theta.cos()
        - 6. * (M + m) * g * theta.sin()
        - 6. * (u - b * v) * theta.cos())
        / (4. * l * (m + M) - 3. * m * l * theta.cos().powi(2));
    let V: Array2<_> = (((&lq + &vi).mapv(f64::exp) - &q) * (mi_minus_mi_t).mapv(f64::cos)
        - ((&lq - &vi).mapv(f64::exp) - &q) * (mi_plus_mi_t).mapv(f64::cos))
        * e.as_row()
        * e.as_column()
        * 0.5;
    dVdm.slice_mut(s![.., .., j]).assign(
        &(((Array2::zeros((d, d)) + u.as_column() - u.as_row()) * &U1
            + (Array2::<f64>::zeros((d, d)) + u.as_column() + u.as_row()) * &U2)
            * e.as_column()
            * e.as_row()),
    );
	{
        {
            {
                {
                    {
                        let LdXi: Array2<_> = (dmahai
                            + kdX.slice(s![.., i, d]).into_column()
                            + kdX.slice(s![.., j, d]))
                            * &L;
                    }
                }
            }
        }
    }
	let dnlml = Array1::from_shape_fn(ln_hyperparams.len(), |i| {
        0.5 * (
            self.covfunc.deriv_covariance(ln_hyperparams, train_inputs, i)
            * &W
        ).scalar_sum()
    });
	dVdi.slice_mut(s![.., output_index, .., input_index])
	.assign(
		&(c * (iR
			.column(input_index)
			.into_column()
			.dot(&lb.view().into_row())
			- (&t * &tlb_view.into_column()).reversed_axes()
			+ tlbdi2)),
	);
	{
        let tdX: Array1<_> = -0.5
            * t
            * (&iR.t()
                * test_covariance
                * (-2. * &inv_sq_len_scales_i - 2. * &inv_sq_len_scales_i))
              .sum_axis(Axis(0));
    }
	{
        let dSds: Array2<_> = r2
            * (2. * i2SpW.dot(&m_minus_z.as_column()).dot(&m_minus_z.as_row())
                - Array2::<f64>::eye(D))
              .dot(&i2SpW)
            - 2. * L * &dLds;
    }
	let f = future::poll_fn(move || {
        match tokio_threadpool::blocking(|| f.poll()).unwrap() {
            Async::Ready(v) => v,
            Async::NotReady => Ok(Async::NotReady),
        }
    });
	test("Your number: ", match input {
        BigEnum::One => 0,
        BigEnum::Two => 0,
        BigEnum::Three => 0,
        BigEnum::Four => 0,
        BigEnum::Five => 0,
        BigEnum::Six => 0,
        BigEnum::Seven => 0,
        BigEnum::Eight => 0,
        BigEnum::Nine => 0,
        BigEnum::Ten => 0,
        BigEnum::Eleven => 0,
        BigEnum::Twelve => 0,
	});
	window
		.task_manager()
		.dom_manipulation_task_source()
		.queue(
			task!(fire_dom_content_loaded_event: move || {
				let document = document.root();
				document.upcast::<EventTarget>().fire_bubbling_event(atom!("DOMContentLoaded"));
				update_with_current_time_ms(&document.dom_content_loaded_event_end);
			}),
			window.upcast(),
		)
		.unwrap();
	
    foo(|| {
        loop {
            foo();
        }
    });
    foo(|| {
        while true {
            foo();
        }
    });
    foo(|| {
        if true {
            foo();
        }
    });
	tokio::spawn(async {
        println!();
    });

	// this block will be properly formatted
	{
		let a = 1;
		let b=2;
		let c = " very very very very very very very very very very very very very very very very very very long string in a block";
	}
	
	let v = vec![1, 2];
	
	// this block won't be formatted
	v.iter().for_each(|_| {
		let a = 1;
		let b=2;
		let c = " very very very very very very very very very very very very very very very very very very long string in a block";
	});
	
	// this block with shorter str will be properly formatted
	v.iter().for_each(|_| {
		let a = 1;
		let b=2;
		let c = " less very very long string in a block";
	});

	async fn forty_two() -> i32 {
		42
	}
	
	fn spawn_async(_f: impl Future<Output=()>) {
		unimplemented!();
	}
	
	fn main() {
		spawn_async(async {
			println!("{}", await!(forty_two()));
		});
	}
	if 1 {} else if eq!(b[1] == b'o' b'n' b't' b'e' b'n' b't' b'-' b'r' b'a' b'n' b'g' b'e' ) {}
}
static REPRO: &[usize] = &[
	#[cfg(feature = "zero")]
	0,
];

lazy_static! {
    pub static ref BLOCKING_POOL: tokio_threadpool::ThreadPool = {
        tokio_threadpool::Builder::new().pool_size(1).build()
    };

    static ref FOO: Foo = unsafe {
        very_long_function_name().another_function_with_really_long_name()
    };
}

static DEFAULT_HOOK: SyncLazy<Box<dyn Fn(&panic::PanicInfo<'_>) + Sync + Send + 'static>> =
    SyncLazy::new(|| {
        let hook = panic::take_hook();
        panic::set_hook(Box::new(|info| {
            // Invoke the default handler, which prints the actual panic message and optionally a backtrace
            (*DEFAULT_HOOK)(info);

            // Separate the output with an empty line
            eprintln!();

            // Print the ICE message
            rustc_driver::report_ice(info, BUG_REPORT_URL);
        }));
        hook
    });

fn main() {
	let factorial = |recur: &dyn Fn(u32) -> u32, arg: u32| -> u32 { if arg == 0 {1} else {arg * recur(arg-1)} };
	self.time_passes = config.opts.prints.is_empty()
		&& (config.opts.debugging_opts.time_passes || config.opts.debugging_opts.time);
	config.opts.maybe_sysroot = Some(config.opts.maybe_sysroot.clone().unwrap_or_else(|| {
        std::env::current_exe().unwrap().parent().unwrap().parent().unwrap().to_owned()
    }));
	Box::new(rustc_codegen_ssa::base::codegen_crate(
		LlvmCodegenBackend(()),
		tcx,
		crate::llvm_util::target_cpu(tcx.sess).to_string(),
		metadata,
		need_metadata_module,
	));
	let (codegen_results, work_products) = ongoing_codegen
		.downcast::<rustc_codegen_ssa::back::write::OngoingCodegen<GccCodegenBackend>>()
		.expect("Expected GccCodegenBackend's OngoingCodegen, found Box<Any>")
		.join(sess);
	if let Some(old) = old {
        self.cur = unsafe { (self.step)(old) };
    }
	let _prof_timer = tcx.prof.generic_activity_with_args(
		"codegen_module",
		&[cgu_name.to_string(), cgu.size_estimate().to_string()],
	);
	let tm = match (cgcx.tm_factory)(tm_factory_config) {};
	let _ = [0; {struct Foo; impl Foo {const fn get(&self) -> usize {5}}; Foo.get()}];
	syntactically_correct(loop { sup( '?'); }, if cond { 0 } else { 1 });
	unsafe { &*self.llmod_raw }
	llvm::LLVMRustDisposeTargetMachine(&mut *(self.tm as *mut _));
	let tm = (cgcx.tm_factory)(tm_factory_config).map_err(|e| write::llvm_err(&diag_handler, &e))?;
	target_machine_factory(sess, config::OptLevel::No, &features)(config)
		.unwrap_or_else(|err| llvm_err(sess.diagnostic(), &err).raise());
	let TestOutcome { completed: ok, errors: err, .. } = forest.process_obligations(&mut C(
		|obligation| match *obligation {
			"D'" => {
				d_count += 1;
				ProcessResult::Error("operation failed")
			}
			_ => unreachable!(),
		},
		|_| {},
	));
	let adjusted_span = (|| {
		if let ExprKind::Block { body } = &expr.kind && let Some(tail_ex) = body.expr {
			let mut expr = &this.thir[tail_ex];
			while let ExprKind::Block {
				body: Block { expr: Some(nested_expr), .. },
			}
			| ExprKind::Scope { value: nested_expr, .. } = expr.kind
			{
				expr = &this.thir[nested_expr];
			}
			this.block_context.push(BlockFrame::TailExpr {
				tail_result_is_ignored: true,
				span: expr.span,
			});
			return Some(expr.span);
		}
		None
	})();
	match (test.end, pat.end, lo, hi) {
		// pat < test
		(_, _, Greater, _) |
		(_, Excluded, Equal, _) |
		// pat > test
		(_, _, _, Less) | // <-
		(Excluded, _, _, Equal) => Some(true),
		_ => Some(false),
	}
	let overlaps: Vec<_> = pats
		.filter_map(|pat| Some((pat.ctor().as_int_range()?, pat.span())))
		.filter(|(range, _)| self.suspicious_intersection(range))
		.map(|(range, span)| (self.intersection(&range).unwrap(), span))
		.collect();
	match bb_data.terminator().kind {
		(JustBefore(n), JustBefore(m)) if n < m => n..=(m - 1),
		(JustBefore(n), AfterMax) => n..=u128::MAX,
		_ => unreachable!(), // Ruled out by the sorting and filtering we did
		(Some(to), Some(from)) => {
			(from == Ordering::Greater || from == Ordering::Equal)
				&& (to == Ordering::Less
					|| (other_end == self_end && to == Ordering::Equal))
		}
		Return | Resume | Abort | GeneratorDrop | Unreachable => {}

		Goto { target } => propagate(target, exit_state),

		Assert { target, cleanup: unwind, expected: _, msg: _, cond: _ }
		| Drop { target, unwind, place: _ }
		| DropAndReplace { target, unwind, value: _, place: _ }
		| FalseUnwind { real_target: target, unwind } => {
			if let Some(unwind) = unwind {
				if dead_unwinds.map_or(true, |dead| !dead.contains(bb)) {
					propagate(unwind, exit_state);
				}
			}

			propagate(target, exit_state);
		}
		mir::Rvalue::Ref(_, mir::BorrowKind::Mut { .. }, place)
		| mir::Rvalue::AddressOf(_, place) => (self.0)(place),
	
		_ => {}
		suggestion_kind @ "suggestion"
		| suggestion_kind @ "suggestion_short"
		| suggestion_kind @ "suggestion_hidden"
		| suggestion_kind @ "suggestion_verbose" => {
			let (span, applicability) = (|| match &info.ty {
				ty @ syn::Type::Path(..)
					if type_matches_path(ty, &["rustc_span", "Span"]) =>
				{
					let binding = &info.binding.binding;
					Ok((
						quote!(*#binding),
						quote!(rustc_errors::Applicability::Unspecified),
					))
				}
				_ => throw_span_err!(
					info.span.unwrap(),
					"wrong field type for suggestion",
					|diag| {
						diag.help("#[suggestion(...)] should be applied to fields of type Span or (Span, Applicability)")
					}
				),
			})()?;
			for arg in list.nested.iter() {
				if let syn::NestedMeta::Meta(syn::Meta::NameValue(arg_name_value)) = arg
				{
					if let syn::MetaNameValue { lit: syn::Lit::Str(s), .. } =
						arg_name_value
					{
						let name = arg_name_value
							.path
							.segments
							.last()
							.unwrap()
							.ident
							.to_string();
						let name = name.as_str();
						let formatted_str = self.build_format(&s.value(), arg.span());
						match name {
							"message" => {
								msg = Some(formatted_str);
							}
							"code" => {
								code = Some(formatted_str);
							}
							other => throw_span_err!(
								arg.span().unwrap(),
								&format!(
									"`{}` is not a valid key for `#[suggestion(...)]`",
									other
								)
							),
						}
					}
				}
			}
			let msg = if let Some(msg) = msg {
				quote!(#msg.as_str())
			} else {
				throw_span_err!(
					list.span().unwrap(),
					"missing suggestion message",
					|diag| {
						diag.help("provide a suggestion message using #[suggestion(message = \"...\")]")
					}
				);
			};
			let code = code.unwrap_or_else(|| quote! { String::new() });
			quote! {
				#diag.#suggestion_method(#span, #msg, #code, #applicability);
			}
		}
		other => throw_span_err!(
			list.span().unwrap(),
			&format!("invalid annotation list `#[{}(...)]`", other)
		),
		_ => self.failed |= !(self.cb)(expr),
	}
	ListItem {
		pre_comment,
		pre_comment_style,
		item: if self.inner.peek().is_none() && self.leave_last {
			None
		} else {
			(self.get_item_string)(&item)
		},
		post_comment,
		new_lines,
	};
	comment_len(item.pre_comment.as_ref().map(|x| &(*x)[..]))
		+ comment_len(item.post_comment.as_ref().map(|x| &(*x)[..]))
		+ item.item.as_ref().map_or(0, |s| unicode_str_width(s));
	Some(match rel {
        Rel::Lt => {
            match (lx, rx) {
                (Some(l @ ExtremeExpr { which: Maximum, .. }), _) => (l, AlwaysFalse), // max < x
                (_, Some(r @ ExtremeExpr { which: Minimum, .. })) => (r, AlwaysFalse), // x < min
                _ => return None,
            }
        },
        Rel::Le => {
            match (lx, rx) {
                (Some(l @ ExtremeExpr { which: Minimum, .. }), _) => (l, AlwaysTrue), // min <= x
                (Some(l @ ExtremeExpr { which: Maximum, .. }), _) => (l, InequalityImpossible), // max <= x
                (_, Some(r @ ExtremeExpr { which: Minimum, .. })) => (r, InequalityImpossible), // x <= min
                (_, Some(r @ ExtremeExpr { which: Maximum, .. })) => (r, AlwaysTrue), // x <= max
                _ => return None,
            }
        },
        Rel::Ne | Rel::Eq => return None,
    });
	pat.each_binding_or_first(&mut |_, id, span, _| match cx
		.typeck_results()
		.extract_binding_mode(cx.sess(), id, span)
		.unwrap()
	{
		BindingMode::BindByValue(_) if !is_copy(cx, cx.typeck_results().node_type(id)) => {
			capture = CaptureKind::Value;
		},
		BindingMode::BindByReference(Mutability::Mut) if capture != CaptureKind::Value => {
			capture = CaptureKind::Ref(Mutability::Mut);
		},
		_ => (),
	});
	for (parent_id, parent) in cx.tcx.hir().parent_iter(e.hir_id) {
        if let [
            Adjustment {
                kind: Adjust::Deref(_) | Adjust::Borrow(AutoBorrow::Ref(..)),
                target,
            },
            ref adjust @ ..,
        ] = *cx
            .typeck_results()
            .adjustments()
            .get(child_id)
            .map_or(&[][..], |x| &**x)
        {
            if let rustc_ty::RawPtr(TypeAndMut { mutbl: mutability, .. }) | rustc_ty::Ref(_, _, mutability) =
                *adjust.last().map_or(target, |a| a.target).kind()
            {
                return CaptureKind::Ref(mutability);
            }
        }
	}
	while let Some(higher::IfOrIfLet { cond, then, r#else }) = higher::IfOrIfLet::hir(expr) {
        if let Some(else_expr) = r#else {
            expr = else_expr;
        } else {
            break;
        }
    }
	let lorem = vec!["ipsum","dolor","sit","amet","consectetur","adipiscing","elit"];
    let hyper = Arc::new(Client::with_connector(HttpsConnector::new(TlsClient::new())));
	{
        {
            let creds = self.client
                .client_credentials(&self.config.auth.oauth2.id, &self.config.auth.oauth2.secret)?;
        }
    }
	given(
        r#"
        # Getting started
        ...
    "#,
    )
        .running(waltz);
	conn.query_row(
        r#"
            SELECT title, date
            FROM posts,
            WHERE DATE(date) = $1
        "#,
        &[],
        |row| {
            Post {
                title: row.get(0),
                date: row.get(1),
            }
        },
    )?;
	ThreadPool::new(Configuration::new().num_threads(1))
	.unwrap()
	.install(
		|| {
			scope(
				|s| {
					use std::sync::mpsc::channel;
					let (tx, rx) = channel();
					let a = s.spawn_future(lazy(move || Ok::<usize, ()>(rx.recv().unwrap())));
					//                          ^^^^ FIXME: why is this needed?
					let b = s.spawn_future(a.map(|v| v + 1));
					let c = s.spawn_future(b.map(|v| v + 1));
					s.spawn(move |_| tx.send(20).unwrap());
					result = Some(c.rayon_wait().unwrap());
				},
			);
		},
	);
    bootstrap.checks.register(
        "PERSISTED_LOCATIONS",
        move || if locations2.0.inner_mut.lock().poisoned {
            Check::new(
                State::Error,
                "Persisted location storage is poisoned due to a write failure",
            )
        } else {
            Check::new(State::Healthy, "Persisted location storage is healthy")
        },
    ); 
	self.cur_type()
		.num_template_args()
		.or_else(|| {
			let n: c_int = unsafe { clang_Cursor_getNumTemplateArguments(self.x) };

			if n >= 0 {
				Some(n as u32)
			} else {
				debug_assert_eq!(n, -1);
				None
			}
		})
		.or_else(|| {
			let canonical = self.canonical();
			if canonical != *self {
				canonical.num_template_args()
			} else {
				None
			}
		});

	if let VrMsg::ClientReply {request_num: reply_req_num, value, ..} = msg {
		let _ = safe_assert_eq!(reply_req_num, request_num, op);
		return Ok((request_num, op, value));
	}
	pub struct FileInput {
		input: StringInput,
		file_name: OsString,
	}
	match len {
		Some(len) => Ok(new(self.input, self.pos + len)),
		None => Err(self),
	}
	
}
fn x() {
    {
        let type_list: Vec<_> = try_opt!(types.iter().map(|ty| ty.rewrite(context, shape)).collect());
    }
}
impl Foo {
    fn map_pixel_to_coords(&self, point: &Vector2i, view: &View) -> Vector2f {
        unsafe {
            Vector2f::from_raw(ffi::sfRenderTexture_mapPixelToCoords(self.render_texture, point.raw(), view.raw()))
        }
    }
}

extern "C" {
	pub fn GetConsoleHistoryInfo(console_history_info: *mut ConsoleHistoryInfo) -> Boooooooooooooool;
    pub fn variadic_fn(first_parameter: FirstParameterType,
                       second_parameter: SecondParameterType,
                       ...); // no trailing comma
}

fn deconstruct(foo: Bar) -> (SocketAddr, Header, Method, RequestUri, HttpVersion, AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA) {
    let if_method_break: SocketAddr = if remote_addr.is_ipv4() {
        "0.0.0.0:0"
    } else {
        "[::]:0"
    }
    .parse()?;
    let method_chain_middle_await_break = reader
        .next_line()
        .await
        .unwrap_or_else(|_| Some(String::new()))
        .expect("failed to read line");
}

fn needs_paren(op: AssocOp, other: AssocOp, dir: Associativity) -> bool {
	other.precedence() < op.precedence()
		|| (other.precedence() == op.precedence()
			&& ((op != other && associativity(op) != dir)
				|| (op == other && associativity(op) != Associativity::Both)))
		|| is_shift(op) && is_arith(other)
		|| is_shift(other) && is_arith(op)
}

pub fn peel_blocks<'a>(mut expr: &'a Expr<'a>) -> &'a Expr<'a> {
    while let ExprKind::Block(
        Block {
            stmts: [],
            expr: Some(inner),
            rules: BlockCheckMode::DefaultBlock,
            ..
        },
        _,
    ) = expr.kind
    {
        expr = inner;
    }
    expr
}

pub fn target_features(sess: &Session) -> Vec<Symbol> {
    supported_target_features(sess)
        .iter()
        .filter_map(
            |&(feature, gate)| {
                if sess.is_nightly_build() || gate.is_none() { Some(feature) } else { None }
            },
        )
        .filter(|_feature| {
            // TODO(antoyo): implement a way to get enabled feature in libgccjit.
            false
        })
        .map(|feature| Symbol::intern(feature))
        .collect()
}


fn a() { Arc::new(|_| { Ok(()) })}
fn g<'a>(&self, x: usize, y:usize) -> Box<dyn Fn(bool) -> usize + 'a> {
	let f = move |t: bool| if t { x } else { y };
	return Box::new(f);
}

struct Compose<F, G>(F, G);
impl<T, F, G> FnOnce<(T,)> for Compose<F, G>
where
    F: FnOnce<(T,)>,
    G: FnOnce<(F::Output,)>,
{
    type Output = G::Output;
    extern "rust-call" fn call_once(self, (x,): (T,)) -> G::Output {
        (self.1)((self.0)(x))
    }
}

fn build_sorted_static_get_entry_names(
    mut entries: Vec<(u8, &'static str)>,
) -> (impl Fn(
    AlphabeticalTraversal,
    Box<dyn dirents_sink::Sink<AlphabeticalTraversal>>,
) -> BoxFuture<'static, Result<Box<dyn dirents_sink::Sealed>, Status>>
      + Send
      + Sync
      + 'static) {
}
fn qcxbfds() {
    {{{
        let explicit_arg_decls =
            explicit_arguments.into_iter()
            .enumerate()
            .map(|(index, (ty, pattern))| {
                let lvalue = Lvalue::Arg(index as u32);
                block = this.pattern(block,
                                     argument_extent,
                                     hair::PatternRef::Hair(pattern),
                                     &lvalue);
                ArgDecl { ty: ty }
            });
    }}}
}

#[print_target_and_args(first_outer)]
#[print_target_and_args(second_outer)]
impl Bar<{1 > 0}> for Foo<{true}> {
    #![print_target_and_args(first_inner)]
    #![print_target_and_args(second_inner)]
}

/// `cfg(...)`'s that are feature gated.
const GATED_CFGS: &[GatedCfg] = &[
    // (name in cfg, feature, function to check if the feature is enabled)
    (sym::target_abi, sym::cfg_target_abi, cfg_fn!(cfg_target_abi)),
    (sym::target_thread_local, sym::cfg_target_thread_local, cfg_fn!(cfg_target_thread_local)),
    (
        sym::target_has_atomic_equal_alignment,
        sym::cfg_target_has_atomic_equal_alignment,
        cfg_fn!(cfg_target_has_atomic_equal_alignment),
    ),
    (sym::target_has_atomic_load_store, sym::cfg_target_has_atomic, cfg_fn!(cfg_target_has_atomic)),
    (sym::sanitize, sym::cfg_sanitize, cfg_fn!(cfg_sanitize)),
    (sym::version, sym::cfg_version, cfg_fn!(cfg_version)),
];
[
	{
		while let Ok(flag) = input.try_parse(|input| {
			Ok(
				match_ignore_ascii_case! { &input.expect_ident().map_err(|_| ())?,
					"jis78" =>
						exclusive_value!((result, VariantEastAsian::JIS78 | VariantEastAsian::JIS83 |
												VariantEastAsian::JIS90 | VariantEastAsian::JIS04 |
												VariantEastAsian::SIMPLIFIED | VariantEastAsian::TRADITIONAL
										) => VariantEastAsian::JIS78)
					_ => return Err(()),
				},
			)
		}) {
			result.insert(flag);
		}
	}
]

#[cfg_attr(doc_cfg, doc(cfg(feature = "beep boop")))]
impl Foo for Bar {
	fn stuff(data: Leet) -> Result<Self> {
		let manual_clone = node.data == Data::Private || node.ident == "beep";
		if data.boop(Foo) && !(data.boop(Bar) && data.boop2(Too![=])) {
			data.stuff().map(A::B)
		} else if data.boop(Ident::abc)
			|| data.boop(Too![::]) && data.boop3(Ident::abc)
		{
			data.stuff().map(NestedMeta::Meta)
		} else {
			Err(data.error("brrr"))
		}
		s.match_indices(prefix).any(|(i, _)| { s[i + prefix.len()..].trim_start_matches('[') })
		if args.len() >= 2 && args[1] == "fail" {
			foo();
		} else if args.len() >= 2 && args[1] == "double-fail" {
			double();
		} else {
			runtest(&args[0]);
		}
	}
}

impl<T1, F1, T2, F2, T3, F3, E> Future
    for TryJoin3<F1, F2, F3>
    where
        F1: Future<Output = Result<T1, E>>,
        F2: Future<Output = Result<T2, E>>,
        F3: Future<Output = Result<T3, E>> 
{
    fn c() {}
}

c! {
    impl<E: A> A<E> {
        // 
        pub(crate) unsafe fn no_extra_indent_in_params<'a>(
            &'a self,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>>
        where
            &'a E: io::Read + 'a,
        {}
    }
    pub trait AsyncBufReadExt: AsyncBufRead {
        fn read_until<'a>(&'a mut self, byte: u8, buf: &'a mut Vec<u8>) -> ReadUntil<'a, Self>
            where Self: Unpin {
            read_until(self, byte, buf)
        }
    }
}

let BufWriter {
    inner:
        BufReader {
            inner,
            buf: rbuf,
            pos,
            cap,
            seek_state: rseek_state,
        },
    buf: wbuf,
    written,
    seek_state: wseek_state,
} = b;

{
    {
        fn clock(){
            match CONTEXT.try_with(|ctx| (*ctx.borrow()).as_ref().map(|ctx| ctx.as_inner().clock.clone())) {}
            match s.match_indices(prefix).any(|(i, _)| { s[i + prefix.len()..].trim_start_matches('[') }) {}
            match a //
            && b {}
        }
    }
}

pub(super) fn shutdown(self) {
    let (task1, _) = super::unowned(async {});
    let (task2, _) = super::unowned(async { 1 });
    let (task3, _) = super::unowned(async { f(); 2 });
    let vtable = self.header().vtable;
    unsafe { (vtable.shutdown)(self.ptr) }
}

fn inlining_last_if_else_block_is_awkward() {
    poll_fn(move |cx| {
        if !fired {
            return Poll::Pending;
        }
        
        if gate.load(SeqCst) {
            Poll::Ready
        } else {
            Poll::Pending
        }
    })
}

pub(super) fn vtable<T: Future, S: Schedule>() -> &'static Vtable {
    &(Vtable {
        poll: poll::<T, S>,
        dealloc: dealloc::<T, S>,
        try_read_output: try_read_output::<T, S>,
        try_set_join_waker: try_set_join_waker::<T, S>,
        drop_join_handle_slow: drop_join_handle_slow::<T, S>,
        drop_abort_handle: drop_abort_handle::<T, S>,
        remote_abort: remote_abort::<T, S>,
        shutdown: shutdown::<T, S>,
    })
}

// comment
const ________________ = self.exp != S::MIN_EXP && (self.sig[0] & sig_mask) == 0;

let r = if s.starts_with("0x") || s.starts_with("0X") {
	zzzzzzzzzzzzzz
} else {
	2
};

pub trait Float:
    Copy +
        Default +
        FromStr<Err = ParseError> +
        PartialOrd +
        fmt::Display +
        Neg<Output = Self> +
        AddAssign +
        SubAssign +
        MulAssign +
        DivAssign +
        RemAssign +
        Add<Output = StatusAnd<Self>> +
        Sub<Output = StatusAnd<Self>> +
        Mul<Output = StatusAnd<Self>> +
        Div<Output = StatusAnd<Self>> +
        Rem<Output = StatusAnd<Self>> {
    const BITS: usize;
}

// print-width: 80 -------------------------------------------------------------

fn space_before_where</**/>(_: F) where F: X {}

impl<'a, I, T: 'a, E> Iterator for Y<'a, I, E> where I: Iterator<Item = &'a (T, E)>,{}

impl<'a, I, T: 'a, E> IteratorIterator for Y<'a, I, E> where I: Iterator<Item = &'a (T, E)>,{}

impl<'a, I, T: 'a, E> Iterator for Y<'a, I, E> where I: Iterator<Item = &'a (T, E)>,{
    type Y;
}

prettier_always_breaks_this(b(|| ()), c);

self.and_this() .map(|| 1) .unwrap_or(0);

pub fn noop_visit_constraint<T: MutVisitor>(
    AssocConstraint { id, ident, gen_args, kind, span }: &mut AssocConstraint,
    vis: &mut T,
) {}

match kind {
  AssocConstraintKind::Equality { ref mut term } => {
    1
  }
}

impl X {
    pub fn ident(&self) -> Option<Ident> {
        match self.kind {
            AttrKind::Normal(ref item, _) => {
                if item.path.segments.len() == 1 {
                    Some(item.path.segments[0].ident)
                } else {
                    None
                }
            }
            AttrKind::DocComment(..) => None,
        }
    }
}

#![attr] // comment to the right of attr

/*
*/

a!(/**/);

Thing(/* _______ */);
pub /* _______ */ const a = {};

const a = "💖"
// ______________
// _______ _______ _______

/*
 _______
*/ foo /*
 _______
*/;

this.call(a, /* _______ */ b);

/*
foo

text
*/

/*
fn a() {
    println!("b");
}
// */

#[a="a"]
#[b] // v
#[cfg_attr(rustfmt, rustfmt::skip)]
pub static x: [i32; 0] = [];

f!()/* comment */;
f!{}/* comment */;
f![]/* comment */;

pub enum Foo {
    A, // `/** **/`
    B, // `/*!`
    C,
}

ProcessSystemError(A {
  code: acc.error.code, // _______
  originalError: acc.error, // _______
});

foo(A {}
  // _______
);

fn c() {} /*
          a

          b
          */

const f = static async |source, block, opts| {
  for entry in source {
    yield async move || {
      const cid = persist(entry.content.serialize(), block, opts).await;
      return A {
        cid,
        path: entry.path,
        unixfs: UnixFS.unmarshal(entry.content.Data),
        node: entry.content
      }
    }
  }
};

