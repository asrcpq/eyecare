mod ec_manager;
use ec_manager::{EcManager, EcManagerRef};

use std::thread;
use std::io::BufReader;
use std::io::prelude::*;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(mut stream: UnixStream, ecman: EcManagerRef) {
	eprintln!("New connection");
	let mut buffer = String::new();
	let mut reader = BufReader::new(stream.try_clone().unwrap());
	while let Ok(len) = reader.read_line(&mut buffer) {
		if len == 0 { break }
		let mut ecman = ecman.lock().unwrap();
		eprintln!("recv cmd: {}", buffer.trim());
		match buffer.trim() {
			"rest on" => ecman.rest_switch(true),
			"rest off" => ecman.rest_switch(false),
			"get score" => {
				let score = ecman.get_score();
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
	let ecman = EcManager::load_to_ref("score.txt");

	for stream in listener.incoming() {
		let ecman = ecman.clone();
		match stream {
			Ok(stream) => {
				thread::spawn(move || handle_client(stream, ecman));
			}
			Err(_) => break,
		}
	}
	Ok(())
}
