use euler_rs::totient::totient_sieve;

fn solve(lim: u64) -> u64 {
    totient_sieve(lim).iter().skip(2).sum()
}

fn main() {
    println!("{}", solve(1_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(8), 21);
    }
}
