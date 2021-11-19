use std::io::prelude::*;
use std::net::TcpStream;

use serde::{Deserialize, Serialize};
use serde_json;
use rand::Rng;
use btl::detach;

#[derive(Serialize, Deserialize)]
struct Request<'a> {
   	event: &'a str,
    body: Option<&'a str>,
	key: &'a str
}

#[derive(Clone)]
pub struct Window {
	key: String,
	port: u16
}

fn random_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%#@!~";
    let mut randomizer = rand::thread_rng();

    let password: String = (0..256).map(|_| { let index = randomizer.gen_range(0..71); CHARSET[index] as char }).collect();
	
	return password;
}

fn is_port_free(port: u16) -> bool {
    if let Err(_) = TcpStream::connect(("127.0.0.1", port)) {
		return true
	} else {
		return false
	}
}

fn random_port() -> u16 {
	for port in 2025..65535 {
		if is_port_free(port) == true {
			return port;
		}
	}
	
	panic!("magically no ports were found");
}

pub fn open(height: u16, width: u16, proxy: bool) -> Window {
	let window = Window {
		key: random_key(),
		port: random_port()
	};
	
	if cfg!(target_os = "macos") {
		let proxied = if proxy == true {
				"true"
			} else {
				"false"
			};
		
		detach! {
			"./webkitd {} '{}' {} {} {} >> ./webkitd.log" window.port window.key height width proxied;
		};
	} else {
		unimplemented!("currently we only support MacOS");
	}
	
	loop {
		if is_port_free(window.port) == false {
			break
		}
	}
	
	return window;
}

impl Window {
	fn send_request(&self, event: &str, body: Option<&str>) {
		let request = Request {
			event: event,
			body: body,
			key: &self.key
		};
		
		let plain = serde_json::to_string(&request).unwrap();
	    let mut stream = TcpStream::connect(&format!("127.0.0.1:{}", &self.port)).unwrap();

	    if let Err(error) = stream.write(plain.as_bytes()) {
	    	panic!("couldn't make request to window because {}", error)
	    }
	}
	
	pub fn hide(&self) {
		self.send_request("hide", None);
	}

	pub fn show(&self) {
		self.send_request("show", None);
	}

	pub fn set_title(&self, title: &str) {
		self.send_request("set_title", Some(title));
	}

	pub fn set_body(&self, body: &str) {
		self.send_request("set_body", Some(body));
	}
	
	pub fn close(&self) {
		self.send_request("close", None);
	}
}
