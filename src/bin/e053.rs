use euler_rs::number::Factorial;
use itertools::*;
use num::BigUint;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    (1..=100)
        .cartesian_product(1..=100)
        .filter(|(n, r)| n >= r)
        .filter(|(n, r)| choose(*n, *r) >= BigUint::from(1_000_000u64))
        .count()
}

fn choose(n: u64, r: u64) -> BigUint {
    n.factorial() / (r.factorial() * (n - r).factorial())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(choose(5, 3), BigUint::from(10u64));
    }
}
