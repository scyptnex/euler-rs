use euler_rs::prime::PrimeBank;

fn main() {
    println!("{}", solve());
}

fn solve() -> i64 {
    let mut primes = PrimeBank::new();
    let mut maxc = 0;
    let mut maxv = 0;
    for a in -999..=999i64 {
        for b in -1000..=1000i64 {
            let s = pg(&mut primes, a, b);
            if s <= maxv {
                continue;
            }
            maxv = s;
            maxc = a * b;
        }
    }
    maxc
}

fn pg(primes: &mut PrimeBank, a: i64, b: i64) -> usize {
    (0..)
        .map(|n| n * n + a * n + b)
        .take_while(|p| *p > 0 && primes.query(*p as u64))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut primes = PrimeBank::new();
        assert_eq!(pg(&mut primes, 1, 41), 40);
        assert_eq!(pg(&mut primes, -79, 1601), 80);
    }
}
