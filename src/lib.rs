use parking_lot::{Condvar, Mutex};

#[derive(Debug)]
pub struct Waitable<T> {
	value: Mutex<T>,
	condvar: Condvar,
}

impl<T> Waitable<T> {
	pub fn new(value: T) -> Self {
		Self {
			value: Mutex::new(value),
			condvar: Condvar::new(),
		}
	}

	pub fn set(&self, value: T) {
		let mut vg = self.value.lock();
		*vg = value;
		self.condvar.notify_all();
	}

	pub fn wait_until<F: Fn(&T) -> bool>(&self, cond: F) {
		let mut vg = self.value.lock();
		while !cond(&*vg) {
			self.condvar.wait(&mut vg);
		}
	}
}

impl<T: Copy> Waitable<T> {
	pub fn get(&self) -> T {
		*self.value.lock()
	}
}

impl<T: PartialEq> Waitable<T> {
	pub fn wait(&self, value: &T) {
		let mut vg = self.value.lock();
		while &*vg != value {
			self.condvar.wait(&mut vg);
		}
	}
}
