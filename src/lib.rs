//! This crate consists of a single struct, [`Waitable`] that holds a value
//! which can be get, set and waited upon.

use parking_lot::{Condvar, Mutex};

/// A waitable variable
///
/// This struct is essentially the `Mutex` and `Condvar` tuple documented in
/// [`parking_lot::Condvar`], presented in an easy-to-use form.
///
/// [`parking_lot::Condvar`]: `parking_lot::Condvar#examples`
#[derive(Debug)]
pub struct Waitable<T> {
	value: Mutex<T>,
	condvar: Condvar,
}

impl<T> Waitable<T> {
	/// Create a new waitable variable
	///
	/// # Examples
	///
	/// ```
	/// use waitable::Waitable;
	///
	/// let w = Waitable::new(0);
	/// ```
	pub fn new(value: T) -> Self {
		Self {
			value: Mutex::new(value),
			condvar: Condvar::new(),
		}
	}

	/// Sets the value of the waitable
	///
	/// This function wakes up ([`notify_all`]) all waiters so that they may
	/// check their waiting conditions and return if appropriate.
	///
	/// [`notify_all`]: parking_lot::Condvar::notify_all
	pub fn set(&self, value: T) {
		let mut vg = self.value.lock();
		*vg = value;
		self.condvar.notify_all();
	}

	/// Waits until a condition is satisfied
	///
	/// This function will block the current thread until the predicate
	/// specified by `cond` returns `true` on the current value of the
	/// Waitable.
	///
	/// If `cond` initially returns `true`, no blocking is done.
	pub fn wait_cond<F: Fn(&T) -> bool>(&self, cond: F) {
		let mut vg = self.value.lock();
		while !cond(&*vg) {
			self.condvar.wait(&mut vg);
		}
	}
}

impl<T: Copy> Waitable<T> {
	/// Gets the value in the waitable
	pub fn get(&self) -> T {
		*self.value.lock()
	}
}

impl<T: PartialEq> Waitable<T> {
	/// Waits until the value of the waitable equals a certain value
	///
	/// This function is essentially equivalent to
	/// `wait_cond(|val| val == value)`
	pub fn wait(&self, value: &T) {
		let mut vg = self.value.lock();
		while &*vg != value {
			self.condvar.wait(&mut vg);
		}
	}
}

impl<T: Default> Default for Waitable<T> {
	/// Constructs a waitable containing the default value of the inner type
	fn default() -> Self {
		Self::new(T::default())
	}
}
