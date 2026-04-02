use itertools::*;
use num::BigRational;
use num::FromPrimitive;
use num::One;

fn e_term(i: usize) -> usize {
    if i == 0 {
        2
    } else if i % 3 != 2 {
        1
    } else {
        2 * ((i + 1) / 3)
    }
}

fn frac(n: usize) -> BigRational {
    let v = (0..n).map(e_term).collect_vec();
    let mut cur = BigRational::from_usize(*v.last().unwrap()).unwrap();
    for i in v.iter().rev().skip(1) {
        cur = BigRational::from_usize(*i).unwrap() + (BigRational::one() / cur);
    }
    cur
}

fn solve() -> u32 {
    frac(100)
        .numer()
        .to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(
            (0..8).map(e_term).collect_vec(),
            vec![2, 1, 2, 1, 1, 4, 1, 1]
        );
        assert_eq!(frac(2), BigRational::from_usize(3).unwrap());
        assert_eq!(
            frac(10),
            BigRational::new(BigInt::from(1457), BigInt::from(536))
        );
        assert_eq!(solve(), 272);
    }
}
