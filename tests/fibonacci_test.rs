use fibonacci_iter::Fibonacci;

#[test]
fn test_fibonacci_sequence() {
    let mut fibonacci = Fibonacci::new();
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
}

#[test]
fn test_large_fibonacci_numbers() {
    let mut fibonacci = Fibonacci::new();
    // Skip the first few numbers
    for _ in 0..15 {
        fibonacci.next();
    }
    assert_eq!(fibonacci.next(), Some(987));
    assert_eq!(fibonacci.next(), Some(1597));
    assert_eq!(fibonacci.next(), Some(2584));
}

#[test]
fn test_fibonacci_always_returns_some() {
    let mut fibonacci = Fibonacci::new();
    for _ in 0..100 {
        assert!(fibonacci.next().is_some());
    }
}
