use std::{thread, time};

use webkit;

fn main() {
	let window_A = webkit::open(300, 500);
	let window_B = webkit::open(500, 300);
	
	window_A.set_body("Window A");
	window_B.set_body("Window B");
	
	window_A.set_title("Window A");
	window_B.set_title("Window B");
	
	window_A.show();
	window_B.show();
}