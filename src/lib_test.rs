#[cfg(test)]
mod tests {
    use fibonacci_iterator::Fibonacci;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
    }

    #[test]
    fn test_fibonacci_overflow() {
        let mut fib = Fibonacci::new();
        // Advance the iterator to near the maximum value.
        for _ in 0..92 {
            fib.next();
        }

        // The next value should be None due to overflow.
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_fibonacci_reset() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        
        // Reset the iterator
        fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(1));
    }
}
