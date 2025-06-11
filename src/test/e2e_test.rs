#[test]
fn test_fibonacci_sequence() {
    let mut fibonacci = fibonacci_sequence::Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(0));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
    assert_eq!(fibonacci.next(), Some(5));
    assert_eq!(fibonacci.next(), Some(8));
    assert_eq!(fibonacci.next(), Some(13));
    assert_eq!(fibonacci.next(), Some(21));
    assert_eq!(fibonacci.next(), Some(34));

    // Check for large numbers (example, not exhaustive)
    let mut fibonacci_large = fibonacci_sequence::Fibonacci::new();
    for _ in 0..50 { // Advance the iterator
        fibonacci_large.next();
    }
    assert!(fibonacci_large.next().is_some()); // Ensure it produces a value
}
