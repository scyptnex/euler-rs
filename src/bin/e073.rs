use num::integer::gcd;

fn count_tween(den: u64) -> u64 {
    (2 * den + 1..3 * den)
        .filter(|nn| nn % 6 == 0)
        .map(|nn| nn / 6)
        .filter(|n| gcd(*n, den) == 1)
        .count() as u64
}

fn solve(n: u64) -> u64 {
    (2..=n).map(count_tween).sum()
}

fn main() {
    println!("{}", solve(12000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(8), 3);
    }
}
