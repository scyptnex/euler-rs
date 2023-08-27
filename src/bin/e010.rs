use euler_rs::prime::PrimeIt;

fn main() {
    println!("{}", solve(2_000_000));
}

fn solve(n: u64) -> u64 {
    PrimeIt::new().take_while(|p| *p < n).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(10), 17);
    }
}
