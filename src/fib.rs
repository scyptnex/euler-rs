use std::iter::Iterator;

pub struct Fibonacci {
    next_fib: u64,
    next_next_fib: u64,
}

impl Fibonacci {
    pub fn new(a: u64, b: u64) -> Self {
        Fibonacci {
            next_fib: a,
            next_next_fib: b,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.next_fib + self.next_next_fib;
        let y = self.next_fib;
        self.next_fib = self.next_next_fib;
        self.next_next_fib = x;
        Some(y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seq() {
        assert_eq!(
            Fibonacci::new(1, 2).take(10).collect::<Vec<u64>>(),
            vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89]
        );
    }
}
