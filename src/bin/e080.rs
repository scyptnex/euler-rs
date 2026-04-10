use num::{BigInt, Zero};

#[derive(Debug)]
struct Sqdc {
    n: u64,
    rem: BigInt,
    p: BigInt,
}

impl Sqdc {
    fn new(n: u64) -> Self {
        assert!(n < 100);
        Self {
            n,
            rem: BigInt::zero(),
            p: BigInt::zero(),
        }
    }
}

impl Iterator for Sqdc {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let c = (BigInt::from(100) * &self.rem) + BigInt::from(self.n);
        self.n = 0;
        let (x, y) = (0..=9)
            .map(|x| {
                let xbi = BigInt::from(x);
                let y = BigInt::from(20) * &self.p;
                let y = y + &xbi;
                let y = y * &xbi;
                (x, y)
            })
            .take_while(|(_, y)| y < &c)
            .max()
            .unwrap();
        self.rem = c - y;
        self.p = (BigInt::from(10) * &self.p) + BigInt::from(x);
        Some(x)
    }
}

fn digi_sum(n: u64) -> u64 {
    Sqdc::new(n).take(100).sum()
}

fn solve() -> u64 {
    (1..=100)
        .filter(|n: &u64| n.isqrt() * n.isqrt() != *n)
        .map(digi_sum)
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_sqdc() {
        assert_eq!(
            Sqdc::new(2).take(10).collect_vec(),
            vec![1, 4, 1, 4, 2, 1, 3, 5, 6, 2]
        );
    }

    #[test]
    fn test_digi_sum() {
        assert_eq!(digi_sum(2), 475);
    }
    #[test]
    fn test_solve() {
        assert_eq!(solve(), 40886);
    }
}
