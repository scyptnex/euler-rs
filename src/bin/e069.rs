use euler_rs::{prime::PrimeSieve, totient::totient};

fn solve(n: u64) -> u64 {
    let ps = PrimeSieve::new(n as usize);
    (2..=n)
        .map(|k| (k, totient(k, &ps)))
        .map(|(k, tk)| (k, (k as f64) / (tk as f64)))
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap()
        .0
}

fn main() {
    println!("{}", solve(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 6);
    }
}
