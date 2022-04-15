use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub struct EcManager {
	at_rest: bool,
	last_event_point: SystemTime,
	last_score: f32,
}

impl EcManager {
	pub fn new_ref() -> EcManagerRef {
		let result = EcManager {
			at_rest: false,
			last_event_point: SystemTime::now(),
			last_score: 0.0,
		};
		Arc::new(Mutex::new(result))
	}

	pub fn update_score(&mut self) {
		let now = SystemTime::now();
		let duration = now.duration_since(self.last_event_point).unwrap().as_secs_f32();
		self.last_event_point = now;
		if self.at_rest {
			self.last_score -= duration * 3.;
		} else {
			self.last_score += duration;
		}
		if self.last_score < 0.0 {
			self.last_score = 0.0;
		}
	}

	pub fn rest_switch(&mut self, on: bool) {
		if self.at_rest == on {
			return
		}
		self.update_score();
		self.at_rest = !self.at_rest;
	}

	pub fn get_score(&mut self) -> i32 {
		self.update_score();
		self.last_score as i32
	}
}

pub type EcManagerRef = Arc<Mutex<EcManager>>;
