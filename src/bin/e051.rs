use euler_rs::decimal::Digs;
use euler_rs::prime::*;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    let mut pb = PrimeBank::new();
    PrimeIt::new().find(|p| mpfsize(*p, &mut pb) >= 8).unwrap()
}

fn mpfsize(num: u64, pb: &mut PrimeBank) -> usize {
    num.digs()
        .iter()
        .map(|d| pfsize(num, *d, pb))
        .max()
        .unwrap_or(0)
}

fn pfsize(num: u64, digit: u64, pb: &mut PrimeBank) -> usize {
    let s = if *num.digs().first().unwrap() == digit {
        1
    } else {
        0
    };
    (s..=9)
        .map(|r| dig_replace(num, digit, r))
        .filter(|num| pb.query(*num))
        .count()
}

fn dig_replace(num: u64, digit: u64, replace: u64) -> u64 {
    let mut result = 0;
    for d in num.digs() {
        result = 10 * result;
        if d == digit {
            result += replace;
        } else {
            result += d;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(dig_replace(100, 0, 3), 133);
        assert_eq!(dig_replace(010, 0, 3), 13);
        assert_eq!(dig_replace(100, 1, 0), 0);

        let mut pb = PrimeBank::new();
        assert_eq!(pfsize(13, 1, &mut pb), 7);
        assert_eq!(pfsize(56003, 0, &mut pb), 7);

        assert_eq!(mpfsize(56003, &mut pb), 7);
    }
}
