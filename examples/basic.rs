use fibonacci_iter::Fibonacci;

fn main() {
    let mut fib = Fibonacci::new();
    for _ in 0..10 {
        println!("{}", fib.next().unwrap());
    }
}
