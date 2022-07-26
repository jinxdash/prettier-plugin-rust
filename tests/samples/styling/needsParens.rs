let (A {} | a() | []) = ();
if let (A {} | a() | []) = () {}

type A: B + C;
type A: impl B + C;

trait A = B + C;
trait A = impl B + C;