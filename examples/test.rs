use spez::spez;

trait A {}
trait B {}

impl A for i32 {}
impl B for i32 {}

impl A for &str {}
impl B for &str {}

fn main() {
	//let x = 0;
	let x = [1, 2, 3];
	//let x = [1.0, 2.0, 3.0];
	//let x = &b"asdf"[..];
	//let x = &();
	//let x = String::new();
	//let x = ();

	spez! {
		for x;
		//for x[0];
		//for &x;
		match<T: A + B> T where i32: From<T> {
			println!("A + B + Into<i32>")
		}
		match<T> [T; 3] where T: A {
			println!("array of 3 things implementing A")
		}
		match<T> [T; 3] {
			println!("array of 3")
		}
		match<T> &[T] {
			println!("slice")
		}
		match<T: ?Sized> &T {
			println!("reference")
		}
		match i32 {
			println!("i32")
		}
		match String {
			println!("String");
		}
	}
}
