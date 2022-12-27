use waitable::Waitable;

#[test]
fn test_get_and_set_int() {
	let w = Waitable::new(1);
	assert_eq!(1, w.get());
	w.set(42);
	assert_eq!(42, w.get());
}

#[test]
fn test_get_and_set_struct() {
	#[derive(Clone, Copy, Debug, PartialEq)]
	struct Foo(i8, bool);

	let w = Waitable::new(Foo(0, true));
	assert_eq!(Foo(0, true), w.get());

	w.set(Foo(-1, false));
	assert_eq!(Foo(-1, false), w.get());
}
