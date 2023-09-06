use itertools::*;
use num::BigUint;
use std::collections::HashSet;

fn main() {
    println!("{}", solve(100));
}

fn solve(upper: i64) -> usize {
    (2..upper + 1)
        .cartesian_product(2..upper + 1)
        .map(|(a, b)| BigUint::from(a as u64).pow(b as u32))
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(5), 15);
    }
}
