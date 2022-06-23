use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct EcData {
	pub score: f32,
	pub rest: bool,
}

#[derive(Clone, Default)]
pub struct EcRef {
	ec: Arc<Mutex<EcData>>,
}

impl EcRef {
	pub fn rest_switch(&self, on: bool) {
		let mut ec = self.ec.lock().unwrap();
		ec.rest = on;
	}

	pub fn get_score(&self) -> f32 {
		let ec = self.ec.lock().unwrap();
		ec.score
	}

	pub fn run(&self) {
		loop {
			std::thread::sleep(std::time::Duration::from_secs(1));
			{
				let mut ec = self.ec.lock().unwrap();
				if ec.rest {
					if ec.score > 3600.0 {
						ec.score -= 3.0;
					} else {
						ec.score -= 5.0;
					}
					if ec.score < 0.0 {
						ec.score = 0.0;
					}
				} else {
					ec.score += 1.0;
				}
			}
		}
	}
}
