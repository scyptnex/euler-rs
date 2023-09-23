fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (1..)
        .map(|n| n.to_string())
        .flat_map(|n| n.into_bytes().into_iter())
        .map(|b| b - b'0')
        .enumerate()
        .filter(|(i, _)| is_pow_10(*i + 1))
        .take(7)
        .map(|(_, v)| v as u64)
        .product()
}

fn is_pow_10(n: usize) -> bool {
    let mut n = n;
    while n > 1 {
        if n % 10 != 0 {
            return false;
        }
        n /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 210);
    }
}
