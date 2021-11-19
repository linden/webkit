use webkit;

fn main() {
	let window_a = webkit::open(300, 500, false);
	let window_b = webkit::open(500, 300, false);
	
	window_a.set_body("Window A");
	window_b.set_body("Window B");
	
	window_a.set_title("Window A");
	window_b.set_title("Window B");
	
	window_a.show();
	window_b.show();
}