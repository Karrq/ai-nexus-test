#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(fib.next(), Some(55));
    }

    #[test]
    fn test_fibonacci_overflow() {
        let mut fib = Fibonacci::new();
        // Consume enough values to approach overflow
        for _ in 0..90 {
            fib.next();
        }
        // Depending on the integer type, the exact overflow point may vary.
        // This test checks that the iterator eventually returns None.
        let mut overflowed = false;
        for _ in 0..100 {
            if fib.next().is_none() {
                overflowed = true;
                break;
            }
        }
        assert!(overflowed, "Fibonacci iterator should overflow eventually");
    }
}

struct Fibonacci {
    a: u32,
    b: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a: 1,
            b: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a.checked_add(self.b)?;
        self.a = self.b;
        self.b = next;
        Some(self.a)
    }
}
