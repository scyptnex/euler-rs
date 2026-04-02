use euler_rs::sqrt::{evaluate_continued_frac, find_continued_period, ContinuedFraction};
use num::BigRational;

// Returns a ratio x/y that minimizes the equation x^2 - dy^2 = 1.
fn fundamental_diophantine(d: usize) -> BigRational {
    let (_, p) = find_continued_period(d);
    let r = p.len();
    let sol = if r % 2 == 0 { r } else { 2 * r };
    let terms = ContinuedFraction::new(d).take(sol).collect();
    let fundamental = evaluate_continued_frac(terms);
    fundamental
}

fn greatest_x_diophantine(d_in: usize) -> usize {
    (2..=d_in)
        .filter(|d| d.isqrt() * d.isqrt() != *d)
        .map(|d| (d, fundamental_diophantine(d)))
        .max_by(|(_, x1), (_, x2)| x1.numer().cmp(x2.numer()))
        .unwrap()
        .0
}

fn solve() -> usize {
    greatest_x_diophantine(1000)
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(greatest_x_diophantine(7), 5);
    }
}
