use euler_rs::prime::sieve;
use std::collections::HashSet;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let primes = sieve(1_000_000).into_iter().collect::<HashSet<_>>();
    primes
        .iter()
        .filter(|p| **p > 10)
        .filter(|p| truncatable(**p, &primes))
        .sum()
}

fn truncatable(p: u64, primes: &HashSet<u64>) -> bool {
    for exp in 1.. {
        let filt = 10u64.pow(exp);
        if filt > p {
            break;
        }
        if !primes.contains(&(p / filt)) || !primes.contains(&(p % filt)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let primes = sieve(4000).into_iter().collect::<HashSet<_>>();
        assert!(truncatable(3797, &primes));
    }
}
