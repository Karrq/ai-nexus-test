#[test]
fn test_fibonacci_sequence() {
    let mut fibonacci = fibonacci_sequence::Fibonacci::new();
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(1));
    assert_eq!(fibonacci.next(), Some(2));
    assert_eq!(fibonacci.next(), Some(3));
    assert_eq!(fibonacci.next(), Some(5));
    assert_eq!(fibonacci.next(), Some(8));
    assert_eq!(fibonacci.next(), Some(13));
    assert_eq!(fibonacci.next(), Some(21));
    assert_eq!(fibonacci.next(), Some(34));
    assert_eq!(fibonacci.next(), Some(55));

    // Check that it iterates indefinitely
    for _ in 0..1000 {
        fibonacci.next();
    }
}
