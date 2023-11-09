use euler_rs::prime::PrimeBank;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let mut pb = PrimeBank::new();
    let mut pb2 = PrimeBank::new();
    (2..)
        .filter(|i| i % 2 == 1)
        .filter(|i| !pb.query(*i))
        .find(|i| !can_do(*i, &mut pb2))
        .unwrap()
}

fn can_do(num: u64, pb: &mut PrimeBank) -> bool {
    (1..)
        .map(|i| 2 * i * i)
        .take_while(|i| *i < num)
        .map(|i| num - i)
        .any(|i| pb.query(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut pb = PrimeBank::new();
        assert!(can_do(9, &mut pb));
    }
}
