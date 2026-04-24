use std::collections::{HashMap, HashSet};

use euler_rs::prime::PrimeSieve;

fn solve(lim: u64) -> u64 {
    let sv = PrimeSieve::new(lim as usize * 2);
    let mut memo = HashMap::<u64, HashSet<u64>>::default();
    let mut needed = (2..=lim).collect::<HashSet<_>>();
    let mut ret = 0;
    for n in 2u64.. {
        let mut makeable = HashSet::default();
        if !sv.is_prime(n as usize) {
            // For every pair of factors ab of n.
            for f in 2..=n.isqrt() {
                if n % f != 0 {
                    continue;
                }
                let f2 = n / f;
                let rem = n - f - f2;
                // I can at least make a set with the factors and 1's to fill.
                makeable.insert(2 + rem);
                // I can also make sets by swapping out each factor for a subset that makes that factor.
                [f, f2].iter().for_each(|ff| {
                    for sub in &memo[ff] {
                        makeable.insert(1 + rem + sub);
                    }
                });
            }
            let mut counted = false;
            for make in &makeable {
                counted = needed.remove(make) || counted;
            }
            if counted {
                ret += n
            }
        }
        assert!(memo.insert(n, makeable).is_none());
        if needed.is_empty() {
            break;
        }
    }
    ret
}

fn main() {
    println!("{}", solve(12000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 4);
        assert_eq!(solve(6), 30);
        assert_eq!(solve(12), 61);
        assert_eq!(solve(12000), 7587457);
    }
}
