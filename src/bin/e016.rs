use euler_rs::decimal::Digs;
use num::bigint::BigUint;

fn main() {
    println!("{}", solve(1000));
}

fn solve(exp: u32) -> u64 {
    BigUint::from(2u64)
        .pow(exp)
        .to_string()
        .as_str()
        .digs()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(15), 26);
    }
}
