/// A struct that generates the Fibonacci sequence.
///
/// Implements the `Iterator` trait to produce an infinite sequence of Fibonacci numbers.
#[derive(Debug, Clone, Copy)]
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Creates a new `Fibonacci` sequence generator.
    ///
    /// Starts the sequence with the initial values 0 and 1.
    pub fn new() -> Self {
        Fibonacci {
            a: 0,
            b: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    /// Generates the next Fibonacci number in the sequence.
    ///
    /// Returns `Some(number)` where `number` is the next Fibonacci number.
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.a;
        self.a = self.b;
        self.b = next.checked_add(self.b).unwrap_or(0);
        Some(next)
    }
}
