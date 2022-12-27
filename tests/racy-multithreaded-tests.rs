use waitable::Waitable;

use std::sync::Arc;
use std::thread;

type Counter = std::sync::atomic::AtomicI32;

fn sleep_a_little() { thread::sleep(std::time::Duration::from_millis(100)); }
fn increment(c: &Arc<Counter>) { c.store(get(c) + 1, std::sync::atomic::Ordering::SeqCst) }
fn decrement(c: &Arc<Counter>) { c.store(get(c) - 1, std::sync::atomic::Ordering::SeqCst) }
fn get(c: &Arc<Counter>) -> i32 { c.load(std::sync::atomic::Ordering::SeqCst) }

fn create_thread<F: FnOnce() + Send + 'static>(running: Arc<Counter>, proc: F) -> thread::JoinHandle<()> {
	increment(&running);
	thread::spawn(move || {
		proc();
		decrement(&running);
	})
}

#[test]
fn test_simple_wait_cond() {
	struct Foo(i8);

	let running = Arc::new(Counter::new(0));

	let w = Arc::new(Waitable::new(Foo(-1)));

	let jh = {
		let w = w.clone();
		create_thread(running.clone(), move || {
			w.wait_cond(|foo| foo.0 == 42);
		})
	};

	sleep_a_little();
	assert_eq!(1, get(&running));

	w.set(Foo(3));
	sleep_a_little();
	assert_eq!(1, get(&running));

	w.set(Foo(42));
	sleep_a_little();
	assert_eq!(0, get(&running));

	jh.join().unwrap();
}

#[test]
fn test_multiple_waits() {
	let running = Arc::new(Counter::new(0));

	let w = Arc::new(Waitable::<i32>::new(-10));

	let jh1 = {
		let w = w.clone();
		create_thread(running.clone(), move || {
			w.wait_cond(|val| *val == 42);
		})
	};

	sleep_a_little();
	assert_eq!(1, get(&running));

	let jh2 = {
		let w = w.clone();
		create_thread(running.clone(), move || {
			w.wait_cond(|val| *val == 43);
		})
	};

	sleep_a_little();
	assert_eq!(2, get(&running));

	let jh3 = {
		let w = w.clone();
		create_thread(running.clone(), move || {
			w.wait(&42);
		})
	};

	sleep_a_little();
	assert_eq!(3, get(&running));

	w.set(1);

	sleep_a_little();
	assert_eq!(3, get(&running));

	w.set(43);

	sleep_a_little();
	assert_eq!(2, get(&running));

	w.set(42);

	sleep_a_little();
	assert_eq!(0, get(&running));

	jh1.join().unwrap();
	jh2.join().unwrap();
	jh3.join().unwrap();
}
