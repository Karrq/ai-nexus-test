//! Fibonacci sequence iterator.

/// A struct that represents a Fibonacci sequence iterator.
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    a: u128,
    b: u128,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence iterator starting from 0 and 1.
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.a;
        let new_b = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = new_b;
        Some(next_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let mut fib = Fibonacci::new();
        assert_eq!(fib.next(), Some(0));
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
    fn test_overflow() {
        let mut fib = Fibonacci::new();
        // exhaust the iterator until it returns None (overflow)
        for _ in 0..1000 {
            if fib.next().is_none() {
                return;
            }
        }
        panic!("Overflow not detected");
    }
}
