use euler_rs::decimal::Digs;
use num::BigUint;

fn is_palindrome(s: &str) -> bool {
    let dgs: Vec<u32> = s.digs();
    let rdgs = dgs.iter().rev().copied().collect::<Vec<u32>>();
    dgs == rdgs
}

fn rad(u: &BigUint) -> BigUint {
    let ds = u.to_str_radix(10);
    let rds: String = ds.chars().rev().collect();
    BigUint::parse_bytes(rds.as_bytes(), 10).unwrap() + u
}

fn is_lychrel(u: &u64) -> bool {
    let mut cur = BigUint::from(*u);
    for _ in 0..50 {
        cur = rad(&cur);
        if is_palindrome(&cur.to_str_radix(10)) {
            return false;
        }
    }
    true
}

fn solve() -> usize {
    (1..10000).filter(is_lychrel).count()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindromic() {
        assert!(!is_palindrome("1337"));
        assert!(is_palindrome("1331"));
    }

    #[test]
    fn test_lychrel() {
        assert!(!is_lychrel(&47));
        assert!(!is_lychrel(&349));
        assert!(is_lychrel(&4994));
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 249);
    }
}
