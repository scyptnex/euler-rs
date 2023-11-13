use euler_rs::prime::sieve;

fn main() {
    println!("{}", solve(1_000_000));
}

fn solve(lim: usize) -> u64 {
    let primes = sieve(lim);
    let prime_set: std::collections::HashSet<u64> = primes.iter().copied().collect();
    (1..std::cmp::min(primes.len(), 600))
        .flat_map(|window_len| {
            primes
                .windows(window_len)
                .map(move |window| (window.iter().sum(), window_len))
        })
        .filter(|(p, _)| prime_set.contains(p))
        .max_by_key(|(_, w)| *w)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(100), 41);
    }
}
