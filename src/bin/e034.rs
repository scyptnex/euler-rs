use euler_rs::decimal::Digs;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (3..=10_000_000).filter(|n| sof(*n) == *n).sum()
}

fn sof(n: u64) -> u64 {
    n.digs().into_iter().map(facto).sum()
}

fn facto(n: u64) -> u64 {
    (1..=n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(sof(145), 145);
    }
}
