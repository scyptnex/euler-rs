use euler_rs::prime::PrimeIt;

fn main() {
    println!("{}", solve(10001));
}

fn solve(n: usize) -> u64 {
    PrimeIt::new().nth(n - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(6), 13);
    }
}
