use num::integer::gcd;

fn closest(n: u64) -> u64 {
    let x = (n * 3) as f64;
    let x = x / 7.0;
    x.floor() as u64
}

fn closest_reduced_frac(n: u64) -> u64 {
    let mut c = closest(n);
    if c * 7 == 3 * n {
        c -= 1;
    }
    while gcd(c, n) != 1 {
        if c <= 1 {
            return 1;
        }
        c -= 1;
    }
    c
}

fn solve(lim: u64) -> (u64, u64) {
    (3..=lim)
        .map(|i| (closest_reduced_frac(i), i))
        .max_by(|a, b| (a.0 * b.1).cmp(&(a.1 * b.0)))
        .unwrap()
}

fn main() {
    println!("{}", solve(1_000_000).0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(closest(5), 2);
        assert_eq!(closest(7), 3);
        assert_eq!(closest(70), 30);
        assert_eq!(closest_reduced_frac(70), 29);
        assert_eq!(solve(8), (2, 5));
    }
}
