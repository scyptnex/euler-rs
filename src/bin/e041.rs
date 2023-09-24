use euler_rs::decimal::Digs;
use euler_rs::prime::sieve;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    sieve(1_000_000_000)
        .into_iter()
        .filter(|n| is_pandig(*n))
        .last()
        .unwrap()
}

fn is_pandig(n: u64) -> bool {
    let s = n.to_string();
    let d = n
        .digs()
        .into_iter()
        .collect::<std::collections::HashSet<_>>();
    s.len() == d.len() && !d.contains(&0) && d.iter().all(|x| *x as usize <= s.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(is_pandig(1234));
        assert!(is_pandig(1));
        assert!(!is_pandig(234));
        assert!(!is_pandig(11));
    }
}
