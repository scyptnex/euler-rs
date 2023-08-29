use euler_rs::decimal::Digs;
use num::bigint::BigUint;

fn main() {
    println!("{}", solve(100));
}

fn solve(v: u64) -> u64 {
    let mut fact: BigUint = num::traits::One::one();
    for i in 1..v {
        fact = fact * i;
    }
    format!("{}", fact).as_str().digs().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 27);
    }
}
