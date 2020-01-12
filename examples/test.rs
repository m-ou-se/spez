use spez::spez;

trait A {}
trait B {}

impl A for i32 {}
impl B for i32 {}

impl A for &str {}
impl B for &str {}

fn main() {
	let x = 0;
	//let x = [1, 2, 3];
	//let x = [1.0, 2.0, 3.0];
	//let x = &b"asdf"[..];
	//let x = &();
	//let x = String::new();
	//let x = ();

	let result = spez! {
		for x;
		//for z @ x[0];
		//for z @ &x;
		match<T: A + B> T where i32: From<T> -> String {
			println!("A + B + Into<i32>");
			format!("Test {}", i32::from(x))
		}
		match<T> [T; 3] where T: A -> i32 {
			println!("array of 3 things implementing A");
			9
		}
		match<T> [T; 3] {
			println!("array of 3");
		}
		match<T> &[T] {
			println!("slice");
		}
		match<T: ?Sized> &T -> (i32, i32) {
			println!("reference");
			(1, 2)
		}
		match i32 {
			println!("i32")
		}
		match String {
			println!("String")
		}
	};

	println!("{:?}", result);
}
