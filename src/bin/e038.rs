use itertools::*;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (0..10000)
        .cartesian_product(2..5)
        .map(|(s, e)| to_pandig(s, e))
        .filter(|s| is_pandigital(s))
        .map(|s| s.parse::<u64>().unwrap())
        .max()
        .unwrap()
}

fn to_pandig(seed: u64, extent: u64) -> String {
    (1..=extent)
        .map(|x| seed * x)
        .map(|x| x.to_string())
        .collect::<String>()
}

fn is_pandigital(num: &str) -> bool {
    if num.len() != 9 {
        return false;
    }
    let digs = num.chars().collect::<std::collections::HashSet<_>>();
    digs.len() == 9 && !digs.contains(&'0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(is_pandigital("123456789"));
        assert!(!is_pandigital("123456788"));
        assert!(!is_pandigital("123456780"));
    }
}
