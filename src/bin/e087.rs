use std::collections::HashSet;

use euler_rs::prime::sieve;

fn solve(lim: u64) -> u64 {
    let ps = sieve(lim.isqrt() as usize);
    let mut nums = HashSet::new();
    for a in &ps {
        for b in &ps {
            let tmp = a.pow(2) + b.pow(3);
            if tmp >= lim {
                continue;
            }
            for c in &ps {
                let res = tmp + c.pow(4);
                if res <= lim {
                    nums.insert(res);
                }
            }
        }
    }
    nums.len() as u64
}

fn main() {
    println!("{}", solve(50_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(50), 4);
        assert_eq!(solve(50_000_000), 1097343);
    }
}
