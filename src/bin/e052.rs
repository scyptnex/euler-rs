use euler_rs::decimal::Digs;
fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (1..)
        .find(|n| csd(*n, 2) && csd(*n, 3) && csd(*n, 4) && csd(*n, 5) && csd(*n, 6))
        .unwrap()
}

fn csd(num: u64, mult: u64) -> bool {
    num.sdigs() == (mult * num).sdigs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(csd(125874, 2));
    }
}
