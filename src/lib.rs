//! A simple Fibonacci iterator.

/// An iterator that generates Fibonacci numbers.
/// Starts at 0 and 1, and yields the next number in the sequence on each call to `next()`.
/// The sequence is unbounded and stops when an integer overflow occurs.
///
/// # Examples
///
/// ```
/// use fibonacci_iterator::FibonacciIterator;
///
/// let mut fibonacci = FibonacciIterator::new();
/// assert_eq!(fibonacci.next(), Some(0));
/// assert_eq!(fibonacci.next(), Some(1));
/// assert_eq!(fibonacci.next(), Some(1));
/// assert_eq!(fibonacci.next(), Some(2));
/// assert_eq!(fibonacci.next(), Some(3));
/// assert_eq!(fibonacci.next(), Some(5));
/// ```







pub struct FibonacciIterator {
    a: u32,
    b: u32,
}

impl FibonacciIterator {
    /// Creates a new `FibonacciIterator`.
    pub fn new() -> Self {
        FibonacciIterator {
            a: 0,
            b: 1,
        }
    }
}

impl Iterator for FibonacciIterator {
    type Item = u32;

    /// Returns the next Fibonacci number in the sequence.
    ///
    /// Returns `Some(number)` if the next number can be calculated without overflow,
    /// otherwise returns `None`.
    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.a.checked_add(self.b)?;
        let new_a = self.b;
        let new_b = next_val;
        self.a = new_a;
        self.b = new_b;
        Some(next_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fibonacci = FibonacciIterator::new();
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
    }

    #[test]
    fn test_overflow() {
        let mut fibonacci = FibonacciIterator::new();
        let mut last = 0;
        for _ in 0..100 {
            last = match fibonacci.next() {
                Some(x) => x,
                None => break,
            };
        }
        assert_eq!(last, 2971215073);
        assert_eq!(fibonacci.next(), None);
    }
}
