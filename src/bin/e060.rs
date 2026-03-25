use std::collections::{HashMap, HashSet};

use euler_rs::prime::PrimeSieve;
use itertools::Itertools;

fn catnum(a: &usize, b: &usize) -> usize {
    let mut mul = 10;
    while mul <= *b {
        mul *= 10;
    }
    a * mul + *b
}

fn digisplit(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (1..)
        .map(|e| 10usize.pow(e))
        .take_while(move |d| *d <= n)
        .map(move |d| (n / d, n % d))
        .filter(move |(l, r)| catnum(l, r) == n)
}

fn ppairs(ps: &PrimeSieve) -> HashSet<(usize, usize)> {
    ps.iter()
        .flat_map(|p| digisplit(p).filter(|(l, r)| ps.is_prime(*l) && ps.is_prime(*r)))
        .collect::<HashSet<(usize, usize)>>()
}

fn has_group(
    count: usize,
    lst: &mut Vec<usize>,
    adj: &HashMap<usize, HashSet<usize>>,
    cdt: HashSet<usize>,
) -> Option<Vec<usize>> {
    if cdt.is_empty() {
        return None;
    }
    let mut cv = cdt.iter().collect_vec();
    cv.sort();
    for c in cv {
        lst.push(*c);
        if lst.len() == count {
            return Some(lst.clone());
        }
        if adj.contains_key(c) {
            let ccdt = cdt.intersection(&adj[c]).copied().collect();
            let sg = has_group(count, lst, adj, ccdt);
            if sg.is_some() {
                return sg;
            }
        }
        lst.pop();
    }
    None
}

fn lsp(count: usize, ps: PrimeSieve) -> usize {
    let pairs = ppairs(&ps);
    let rpairs = pairs
        .iter()
        .filter(|(a, b)| a < b && pairs.contains(&(*b, *a)))
        .copied()
        .collect::<HashSet<(usize, usize)>>();
    let mut adj = HashMap::<usize, HashSet<usize>>::new();
    for (a, b) in &rpairs {
        adj.entry(*a).or_default().insert(*b);
    }
    let adj = adj;
    let mut starts = adj.keys().collect_vec();
    starts.sort();
    for s in starts {
        if let Some(grp) = has_group(count, &mut vec![*s], &adj, adj[s].clone()) {
            return grp.iter().sum();
        }
    }
    panic!();
}

fn solve() -> usize {
    let ps = PrimeSieve::new(84_000_000);
    lsp(5, ps)
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppairs() {
        let ps = PrimeSieve::new(50);
        let mut pair_vec = ppairs(&ps).into_iter().collect_vec();
        pair_vec.sort();
        assert_eq!(pair_vec, vec![(2, 3), (3, 7)]);
    }

    #[test]
    fn test_solve() {
        assert_eq!(digisplit(1010).collect_vec(), vec![(101, 0), (10, 10)]);
        assert_eq!(catnum(&12, &13), 1213);
        assert_eq!(catnum(&10, &10), 1010);

        let ps = PrimeSieve::new(700_000);
        assert_eq!(lsp(4, ps), 792);
    }
}
