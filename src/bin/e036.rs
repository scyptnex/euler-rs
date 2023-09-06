fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    (1..1000000).filter(solvep).sum()
}

fn solvep(n: &u64) -> bool {
    pali(format!("{}", n)) && pali(format!("{:b}", n))
}

fn pali(s: String) -> bool {
    !s.bytes().zip(s.bytes().rev()).any(|(a, b)| a != b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(solvep(&585));
        assert!(!solvep(&5885));
    }
}
