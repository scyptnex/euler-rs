use itertools::*;
use std::collections::HashSet;

fn main() {
    println!("{}", solve());
}

fn solve() -> u64 {
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
        .into_iter()
        .permutations(9)
        .flat_map(|perm| {
            let perm_ref = &perm;
            let cur: HashSet<u64> = (1..=7)
                .flat_map(|left| (1..=8 - left).flat_map(move |right| works(perm_ref, left, right)))
                .collect();
            cur.into_iter()
        })
        .collect::<HashSet<u64>>()
        .iter()
        .sum()
}

fn works(perm: &Vec<u32>, left: usize, right: usize) -> Option<u64> {
    let (l, rp) = perm.split_at(left);
    let (r, p) = rp.split_at(right);
    let prod = numify(p);
    match numify(l) * numify(r) == prod {
        true => Some(prod),
        false => None,
    }
}

fn numify(digs: &[u32]) -> u64 {
    digs.iter()
        .rev()
        .enumerate()
        .map(|(p, v)| 10u64.pow(p as u32) * (*v as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(numify(&[1, 2, 3]), 123);
    }
}
