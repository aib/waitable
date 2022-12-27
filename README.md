# Waitable

A synchronized (atomic) value container implementing the Mutex+Condvar pattern for efficient blocking waits

## Usage

```rust
use std::sync::Arc;
use waitable::Waitable;

let w = Arc::new(Waitable::new(0));

println!("Spawning thread...");

let join_handle = {
	let w = w.clone();
	std::thread::spawn(move || {
		println!("Thread waiting...");
		w.wait(&42);
		println!("Thread done waiting");
	})
};

println!("Waiting to set...");
std::thread::sleep(std::time::Duration::from_millis(500));

println!("Setting...");
w.set(42);

join_handle.join().unwrap();
println!("All done");
```

```
Spawning thread...
Waiting to set...
Thread waiting...
Setting...
Thread done waiting
All done
```

