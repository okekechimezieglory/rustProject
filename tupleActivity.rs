// * use a function that returnns a tuple
fn coordinate() -> (i32, i32) {
	(1, 7)
}

fn main() {
	// Destructure the return values into two variables
	let (x, y) = coordinate();

	// * use an if....else if...else block
	if y > 5 {
		println!(">5");
	} else if y < 5 {
		println!("<5");
	} else {
		println!("=5");
	}
}
