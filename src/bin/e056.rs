use euler_rs::decimal::Digs;
use itertools::Itertools;
use num::BigUint;

fn dcount((a, b): (u64, u32)) -> u64 {
    BigUint::from(a)
        .pow(b)
        .to_str_radix(10)
        .as_str()
        .digs()
        .iter()
        .sum()
}

fn solve() -> u64 {
    (1..100)
        .cartesian_product(1..100)
        .map(dcount)
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 972);
    }
}
