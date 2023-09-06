use euler_rs::prime::PrimeIt;
use std::collections::HashSet;

fn main() {
    println!("{}", solve(1_000_000));
}

fn solve(lim: u64) -> usize {
    let primes: HashSet<_> = PrimeIt::new().take_while(|p| *p < lim).collect();
    primes.iter().filter(|p| is_cirq(**p, &primes)).count()
}

fn is_cirq(p: u64, primes: &HashSet<u64>) -> bool {
    if p < 10 {
        return true;
    }
    let mut shifted = shifty(p);
    while shifted != p {
        if shifted % 10 == 0 {
            return false;
        }
        if !primes.contains(&shifted) {
            return false;
        }
        shifted = shifty(shifted);
    }
    true
}

fn shifty(n: u64) -> u64 {
    n / 10 + (n % 10) * (10u64.pow(n.ilog10()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(shifty(13), 31);
        assert_eq!(shifty(99), 99);
        assert_eq!(shifty(10), 1);
        assert_eq!(solve(100), 13);
    }
}
