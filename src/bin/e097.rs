fn solve() -> u64 {
    let mut two_pow = 7830457;
    let mut ret = 1;
    while two_pow > 16 {
        ret *= 1 << 16;
        ret = ret % 10_000_000_000;
        two_pow -= 16;
    }
    while two_pow > 0 {
        ret *= 2;
        two_pow -= 1;
    }
    ret = ret % 10_000_000_000;
    ret *= 28433;
    ret += 1;
    ret = ret % 10_000_000_000;
    ret
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(), 8739992577);
    }
}
