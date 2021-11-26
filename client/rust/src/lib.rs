use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Serialize, Deserialize, Debug)]
struct Request {
   	event: &'static str,
    body: Option<String>
}

#[derive(Clone, Debug)]
pub struct Window {
	sender: Sender<Request>
}

//TODO (Linden): use height, width & proxy.
pub fn open(height: u16, width: u16, proxy: bool) -> Window {
	if cfg!(target_os = "macos") == false {
		unimplemented!("currently we only support MacOS");
	}
	
	let mut command = Command::new("./webkitd").stdin(Stdio::piped()).spawn().expect("couldn't spawn webkitd");
	
	let (client_sender, worker_receiver) = channel::<Request>();
	
	thread::spawn(move || {
		let input = command.stdin.as_mut().unwrap();
		
		while let Ok(request) = worker_receiver.recv() {
			let plain = serde_json::to_string(&request).unwrap();
			
			input.write_all((plain + "\n").as_bytes()).unwrap();
		}
	});
	
	let window = Window { sender: client_sender };
	
	if proxy == true {
		window.toggle_proxy();
	}
	
	window.set_height(height);
	window.set_width(width);
	
	return window;
}

impl Window {
	fn send_request(&self, request: Request) {
		self.sender.send(request).unwrap();
	}
	
	pub fn hide(&self) {
		self.send_request(Request{ event: "hide", body: None });
	}

	pub fn show(&self) {
		self.send_request(Request{ event: "show", body: None });
	}
	
	pub fn set_height(&self, height: u16) {
		self.send_request(Request{ event: "set_height", body: Some(height.to_string()) });
	}
	
	pub fn set_width(&self, width: u16) {
		self.send_request(Request{ event: "set_width", body: Some(width.to_string()) });
	}

	pub fn set_title(&self, title: String) {
		self.send_request(Request{ event: "set_title", body: Some(title) });
	}

	pub fn set_body(&self, body: String) {
		self.send_request(Request{ event: "set_body", body: Some(body) });
	}
	
	pub fn close(&self) {
		self.send_request(Request{ event: "close", body: None });
	}
	
	pub fn toggle_proxy(&self) {
		self.send_request(Request{ event: "toggle_proxy", body: None });
	}
}
