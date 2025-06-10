//! Fibonacci iterator library

/// Fibonacci struct
#[derive(Debug)]
pub struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    /// Creates a new Fibonacci sequence iterator.
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    /// Calculates the next Fibonacci number.
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(self.a)
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
    }
}
