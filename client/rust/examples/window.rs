use std::{thread, time};

use webkit;

fn main() {
	let window = webkit::open(250, 250, false);
		
	let mut index = 0;
	
	window.show();
	
	loop {
		window.set_body(&format!("Hello for the {}th time", index));
		window.set_title(&format!("Hello #{}", index));
		
		thread::sleep(time::Duration::from_millis(500));
		index = index + 1;
	}
}