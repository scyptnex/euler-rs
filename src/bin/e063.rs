use num::BigUint;

fn ds(n: u32) -> Vec<BigUint> {
    let ten = BigUint::from(10usize);
    let upper = ten.pow(n);
    let lower = upper.clone() / ten;
    let r = (1..)
        .map(|k: usize| BigUint::from(k))
        .map(|k: BigUint| k.pow(n))
        .filter(|n| n >= &lower)
        .take_while(|n| n < &upper)
        .collect();
    r
}

fn solve() -> usize {
    (1..)
        .map(|n| ds(n))
        .take_while(|v| v.len() > 0)
        .map(|v| v.len())
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 49);
    }
}
