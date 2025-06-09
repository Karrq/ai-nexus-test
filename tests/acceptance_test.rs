use fibonacci_sequence::Fibonacci;

#[test]
fn acceptance_test_fibonacci_starts_correctly() {
    let mut fibonacci = Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
}

#[test]
fn acceptance_test_fibonacci_continues_correctly() {
    let mut fibonacci = Fibonacci::new();
    fibonacci.next(); // 1
    fibonacci.next(); // 1
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
}

#[test]
fn acceptance_test_fibonacci_iterates_many_times() {
    let mut fibonacci = Fibonacci::new();
    for _ in 0..10 {
        fibonacci.next();
    }
    // After 10 iterations, the sequence should continue.
    assert_eq!(fibonacci.next(), Some(89));
}
