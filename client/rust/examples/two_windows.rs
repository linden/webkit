use webkit;

fn main() {
	let window_a = webkit::open(300, 500, false);
	let window_b = webkit::open(500, 300, false);

	window_a.set_body(String::from("Window A"));
	window_b.set_body(String::from("Window B"));

	window_a.set_title(String::from("Window A"));
	window_b.set_title(String::from("Window B"));

	window_a.show();
	window_b.show();
	
	loop {}
}