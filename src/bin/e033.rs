use itertools::*;
use num::rational::Ratio;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    *(10..100)
        .cartesian_product(10..100)
        .filter(|(n, d)| n < d)
        .filter(|(n, d)| n % 10 != 0 || d % 10 != 0)
        .filter(|(n, d)| dig_cancel(*n, *d))
        .inspect(|x| {
            dbg!(x);
        })
        .map(|(n, d)| Ratio::new(n, d))
        .product::<Ratio<u64>>()
        .denom()
}

fn dig_cancel(n: u64, d: u64) -> bool {
    let f = |v, b| if b { v / 10 } else { v % 10 };
    [true, false]
        .into_iter()
        .cartesian_product([true, false])
        .map(|(nb, db)| (f(n, nb), f(d, db), f(n, !nb), f(d, !db)))
        .filter(|(_, _, on, od)| on == od)
        .map(|t| (t.0, t.1))
        .filter(|t| t.1 != 0)
        .any(|(np, dp)| Ratio::new(n, d) == Ratio::new(np, dp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(dig_cancel(49, 98));
    }
}
