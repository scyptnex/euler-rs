use num::{BigInt, One, Zero};

fn solve(lim: i64) -> i64 {
    let lim = BigInt::from(lim);
    let mut prev = BigInt::zero();
    let mut last = BigInt::one();
    loop {
        let r: BigInt = &last * 6 - &prev;
        let sqrt: BigInt = &r * &r * 8 + 1;
        let b: BigInt = (&r * 2 + 1 + sqrt.sqrt()) / 2;
        if &r + &b > lim {
            return b.to_str_radix(10).parse().unwrap();
        }
        prev = last;
        last = r;
    }
}

fn main() {
    println!("{}", solve(1_000_000_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(20), 15);
        assert_eq!(solve(22), 85);
    }
}
