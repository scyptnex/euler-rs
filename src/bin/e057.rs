use num::{BigInt, BigRational, One};

fn solve() -> usize {
    let one = BigInt::one();
    let two = &one + &one;
    let mut ret = 0;
    let mut frac = BigRational::new(BigInt::one(), BigInt::one() + BigInt::one());
    for _ in 1..=1000 {
        let chk = &frac + &one;
        if chk.numer().to_str_radix(10).len() > chk.denom().to_str_radix(10).len() {
            ret += 1;
        }
        frac = BigRational::one() / (&frac + &two);
    }
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
        assert_eq!(solve(), 153);
    }
}
