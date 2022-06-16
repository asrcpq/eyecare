mod ec_manager;
use ec_manager::EcRef;

use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream, ec: EcRef) {
	eprintln!("New connection");
	let mut buffer = String::new();
	let mut reader = BufReader::new(stream.try_clone().unwrap());
	while let Ok(len) = reader.read_line(&mut buffer) {
		if len == 0 { break }
		eprintln!("recv cmd: {}", buffer.trim());
		match buffer.trim() {
			"rest on" => ec.rest_switch(true),
			"rest off" => ec.rest_switch(false),
			"get score" => {
				let score = ec.get_score() as i32;
				let _ = stream.write_all(format!("{}", score).as_bytes());
			},
			_ => {},
		}
	}
	eprintln!("Disconnected");
}

fn main() -> std::io::Result<()> {
	let _ = std::fs::remove_file("eyecare.sock");
	let listener = UnixListener::bind("eyecare.sock")?;
	let ec = EcRef::default();
	{
		let ec = ec.clone();
		thread::spawn(move || ec.run());
	}

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				let ec = ec.clone();
				thread::spawn(move || handle_client(stream, ec));
			}
			Err(_) => break,
		}
	}
	Ok(())
}
