use std::collections::{hash_map::Entry, HashMap};

use euler_rs::sqrt::ContinuedFraction;

fn frac_period(n: usize) -> (Vec<usize>, usize) {
    if n.isqrt() * n.isqrt() == n {
        return (Vec::new(), 0);
    }
    let mut cur = ContinuedFraction::new(n);
    let mut ret = Vec::new();
    let mut memo = HashMap::<ContinuedFraction, usize>::new();
    loop {
        let entry = memo.entry(cur);
        if let Entry::Occupied(en) = entry {
            return (ret, *en.get());
        }
        entry.or_insert(ret.len());
        let t = cur.next().unwrap();
        ret.push(t);
    }
}

fn solve(ul: usize) -> usize {
    (1..=ul)
        .map(frac_period)
        .map(|(v, r)| v.len() - r)
        .filter(|p| p % 2 == 1)
        .count()
}

fn main() {
    println!("{}", solve(10000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(frac_period(23), (vec![4, 1, 3, 1, 8], 1));
        assert_eq!(solve(13), 4);
    }
}
