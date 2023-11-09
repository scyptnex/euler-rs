use euler_rs::prime::PrimeBank;
use num::integer::Roots;

fn main() {
    println!("{}", solve(4));
}

fn solve(run_len: usize) -> u64 {
    let mut pb = PrimeBank::new();
    let mut count = 0;
    for i in 2.. {
        if run_len == count_distinct_prime_factors(i, &mut pb) {
            count += 1;
        } else {
            count = 0;
        }
        if count == run_len {
            return i + 1 - (count as u64);
        }
    }
    panic!();
}

fn count_distinct_prime_factors(num: u64, pb: &mut PrimeBank) -> usize {
    if num == 1 {
        return 0;
    } else if pb.query(num) {
        return 1;
    }
    for i in 1..=num.sqrt() {
        if num % i != 0 || !pb.query(i) {
            continue;
        }
        let mut rem = num;
        while rem % i == 0 {
            rem /= i;
        }
        return count_distinct_prime_factors(rem, pb) + 1;
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(2), 14);
        assert_eq!(solve(3), 644);
    }

    #[test]
    fn test_cdpf() {
        let mut pb = PrimeBank::new();
        assert_eq!(count_distinct_prime_factors(14, &mut pb), 2);
        assert_eq!(count_distinct_prime_factors(15, &mut pb), 2);
        assert_eq!(count_distinct_prime_factors(16, &mut pb), 1);
    }
}
