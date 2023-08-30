use euler_rs::factors::get_factors;
use std::collections::HashSet;

fn main() {
    println!("{}", solve(28123));
}

fn solve(l: u64) -> u64 {
    let abundants: HashSet<u64> = (11..l).filter(|x| is_abundant(*x)).collect();
    (1..l).filter(|i| !is_sum_abundant(*i, &abundants)).sum()
}

fn is_sum_abundant(n: u64, abundants: &HashSet<u64>) -> bool {
    if n < 24 {
        return false;
    }
    abundants
        .iter()
        .filter(|a| **a < n)
        .any(|a| abundants.contains(&(n - *a)))
}

fn is_abundant(n: u64) -> bool {
    get_factors(n).sum::<u64>() > n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(is_abundant(12));
        assert!(!is_abundant(13));
        assert!(!is_abundant(28));
    }
}
